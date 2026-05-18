use burn::tensor::{backend::Backend, activation::softmax, Tensor, TensorData};

#[derive(Debug, Clone)]
pub struct Gpt2Config {
    pub n_vocab: usize,
    pub n_ctx: usize,
    pub n_embd: usize,
    pub n_head: usize,
    pub n_layer: usize,
}

impl Gpt2Config {
    pub fn gpt2_small() -> Self {
        Self {
            n_vocab: 50257,
            n_ctx: 1024,
            n_embd: 768,
            n_head: 12,
            n_layer: 12,
        }
    }
}

pub struct Block<B: Backend> {
    pub ln1_w: Tensor<B, 1>,
    pub ln1_b: Tensor<B, 1>,
    pub ln2_w: Tensor<B, 1>,
    pub ln2_b: Tensor<B, 1>,
    // GPT-2 Conv1D: weight shape [in, out] — multiply as  x @ w  (no transpose)
    pub c_attn_w: Tensor<B, 2>,   // [n_embd, 3*n_embd]
    pub c_attn_b: Tensor<B, 1>,
    pub c_proj_w: Tensor<B, 2>,   // [n_embd, n_embd]
    pub c_proj_b: Tensor<B, 1>,
    pub mlp_fc_w: Tensor<B, 2>,   // [n_embd, 4*n_embd]
    pub mlp_fc_b: Tensor<B, 1>,
    pub mlp_proj_w: Tensor<B, 2>, // [4*n_embd, n_embd]
    pub mlp_proj_b: Tensor<B, 1>,
    pub n_head: usize,
    pub n_embd: usize,
}

pub struct Gpt2<B: Backend> {
    pub wte: Tensor<B, 2>,   // [n_vocab, n_embd]
    pub wpe: Tensor<B, 2>,   // [n_ctx,   n_embd]
    pub blocks: Vec<Block<B>>,
    pub ln_f_w: Tensor<B, 1>,
    pub ln_f_b: Tensor<B, 1>,
    pub config: Gpt2Config,
}

impl<B: Backend> Gpt2<B> {
    pub fn forward(&self, token_ids: &[u32]) -> Tensor<B, 1> {
        let t = token_ids.len();
        let n_embd = self.config.n_embd;
        let n_vocab = self.config.n_vocab;

        let tok_emb = gather_rows(&self.wte, token_ids);
        let pos_ids: Vec<u32> = (0..t as u32).collect();
        let pos_emb = gather_rows(&self.wpe, &pos_ids);
        let mut x = tok_emb + pos_emb;

        for block in &self.blocks {
            x = block_forward(block, x);
        }

        x = layer_norm_3d(x, &self.ln_f_w, &self.ln_f_b);

        // LM head: tied to token embedding — [1, n_embd] @ [n_embd, n_vocab]
        let x_last = x.slice([0..1, (t - 1)..t, 0..n_embd]).reshape([1, n_embd]);
        x_last.matmul(self.wte.clone().transpose()).reshape([n_vocab])
    }
}

fn gather_rows<B: Backend>(table: &Tensor<B, 2>, ids: &[u32]) -> Tensor<B, 3> {
    let [_, n_embd] = table.dims();
    let rows: Vec<Tensor<B, 3>> = ids
        .iter()
        .map(|&id| {
            let i = id as usize;
            table.clone().slice([i..i + 1, 0..n_embd]).reshape([1, 1, n_embd])
        })
        .collect();
    Tensor::cat(rows, 1)
}

fn block_forward<B: Backend>(block: &Block<B>, x: Tensor<B, 3>) -> Tensor<B, 3> {
    let attn_out = causal_attn(block, layer_norm_3d(x.clone(), &block.ln1_w, &block.ln1_b));
    let x = x + attn_out;
    let mlp_out = mlp_forward(block, layer_norm_3d(x.clone(), &block.ln2_w, &block.ln2_b));
    x + mlp_out
}

fn causal_attn<B: Backend>(block: &Block<B>, x: Tensor<B, 3>) -> Tensor<B, 3> {
    let [b, t, c] = x.dims();
    let h = block.n_head;
    let hs = c / h;
    let device = x.device();

    let qkv = proj2d(x.reshape([b * t, c]), &block.c_attn_w, &block.c_attn_b)
        .reshape([b, t, 3 * c]);

    let q = qkv.clone().slice([0..b, 0..t, 0..c]).reshape([b, t, h, hs]).permute([0, 2, 1, 3]);
    let k = qkv.clone().slice([0..b, 0..t, c..2 * c]).reshape([b, t, h, hs]).permute([0, 2, 1, 3]);
    let v = qkv.slice([0..b, 0..t, 2 * c..3 * c]).reshape([b, t, h, hs]).permute([0, 2, 1, 3]);

    let scale = (hs as f32).sqrt();
    let att = q.matmul(k.permute([0, 1, 3, 2])) / scale;

    let mask = causal_mask::<B>(b, h, t, &device);
    let att = softmax(att + mask, 3);

    let y = att.matmul(v).permute([0, 2, 1, 3]).reshape([b, t, c]);
    proj2d(y.reshape([b * t, c]), &block.c_proj_w, &block.c_proj_b).reshape([b, t, c])
}

fn mlp_forward<B: Backend>(block: &Block<B>, x: Tensor<B, 3>) -> Tensor<B, 3> {
    let [b, t, c] = x.dims();
    let fc = gelu(proj2d(x.reshape([b * t, c]), &block.mlp_fc_w, &block.mlp_fc_b));
    proj2d(fc, &block.mlp_proj_w, &block.mlp_proj_b).reshape([b, t, c])
}

// GPT-2 Conv1D: y = x @ w + b  (w is already [in, out])
fn proj2d<B: Backend>(x: Tensor<B, 2>, w: &Tensor<B, 2>, b: &Tensor<B, 1>) -> Tensor<B, 2> {
    x.matmul(w.clone()) + b.clone().reshape([1, b.dims()[0]])
}

fn layer_norm_3d<B: Backend>(x: Tensor<B, 3>, w: &Tensor<B, 1>, b: &Tensor<B, 1>) -> Tensor<B, 3> {
    let [batch, seq, n_embd] = x.dims();
    let eps = 1e-5_f32;

    // mean_dim removes the dimension in Burn 0.21 → reshape back to [b, s, 1]
    let mean = x.clone().mean_dim(2).reshape([batch, seq, 1]);
    let diff = x - mean;
    let var = diff.clone().powf_scalar(2.0_f32).mean_dim(2).reshape([batch, seq, 1]);
    let normed = diff / (var + eps).sqrt();

    normed * w.clone().reshape([1, 1, n_embd]) + b.clone().reshape([1, 1, n_embd])
}

fn causal_mask<B: Backend>(b: usize, h: usize, t: usize, device: &B::Device) -> Tensor<B, 4> {
    let mut data = vec![0.0f32; t * t];
    for i in 0..t {
        for j in (i + 1)..t {
            data[i * t + j] = f32::NEG_INFINITY;
        }
    }
    let td = TensorData::new(data, [t * t]);
    Tensor::<B, 1>::from_data(td, device)
        .reshape([1, 1, t, t])
        .expand([b, h, t, t])
}

fn gelu<B: Backend>(x: Tensor<B, 2>) -> Tensor<B, 2> {
    let c = (2.0_f32 / std::f32::consts::PI).sqrt();
    let inner = (x.clone() + x.clone().powf_scalar(3.0_f32) * 0.044715_f32) * c;
    x.clone() * (inner.tanh() + 1.0_f32) * 0.5_f32
}
