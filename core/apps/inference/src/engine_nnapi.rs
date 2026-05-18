/// NNAPI-backed GPT-2 inference engine.
///
/// Requires Android API 31+ (BATCH_MATMUL). The model is compiled once at
/// construction and reused for every autoregressive step.
///
/// Weight layout: GPT-2 Conv1D stores weights [in, out].
/// NNAPI FULLY_CONNECTED expects [out, in], so all FC weights are transposed.
use anyhow::{Context, Result};
use memmap2::MmapOptions;
use nnapi::{
    sys::ANEURALNETWORKS_PREFER_SUSTAINED_SPEED, transpose_2d, GraphBuilder, NnapiCompilation,
};
use safetensors::SafeTensors;
use std::fs::File;

use crate::tokenizer::Tokenizer;

const SYSTEM_CONTEXT: &str =
    "The following is a conversation with Baby AI, a helpful baby-care assistant. \
     Baby AI only discusses feeding, sleep, diapers, growth, and baby milestones.\n\
     Parent:";
const EOS_TOKEN: u32 = 50256;
const MAX_CTX: usize = 128;
const VOCAB_SIZE: usize = 50257;
const N_EMBD: usize = 768;
const N_HEAD: usize = 12;
const HEAD_DIM: usize = 64; // N_EMBD / N_HEAD
const N_LAYER: usize = 12;
const MLP_DIM: usize = 3072; // 4 * N_EMBD

pub struct NnapiInferenceEngine {
    compilation: NnapiCompilation,
    tokenizer: Tokenizer,
}

impl NnapiInferenceEngine {
    pub fn load(model_dir: &str) -> Result<Self> {
        let path = format!("{}/model.safetensors", model_dir);
        let file = File::open(&path).with_context(|| format!("opening {}", path))?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let st = SafeTensors::deserialize(&mmap)?;

        let vocab_path = format!("{}/vocab.json", model_dir);
        let merges_path = format!("{}/merges.txt", model_dir);
        let tokenizer = Tokenizer::from_files(&vocab_path, &merges_path)?;

        let model = build_model(&st)?;
        let compilation = model.compile(ANEURALNETWORKS_PREFER_SUSTAINED_SPEED)?;

        Ok(Self { compilation, tokenizer })
    }

    pub fn generate<F>(&self, user_message: &str, max_tokens: usize, mut on_token: F) -> Result<()>
    where
        F: FnMut(String),
    {
        let prompt = format!("{} {}\nBaby AI:", SYSTEM_CONTEXT, user_message.trim());
        let mut tokens: Vec<i32> = self.tokenizer.encode(&prompt).into_iter().map(|t| t as i32).collect();
        tokens.truncate(MAX_CTX.saturating_sub(max_tokens));

        for _ in 0..max_tokens {
            let start = tokens.len().saturating_sub(MAX_CTX);
            let ctx = &tokens[start..];
            let pos = (ctx.len() - 1) as i32;

            let mut padded = vec![0i32; MAX_CTX];
            padded[..ctx.len()].copy_from_slice(ctx);

            let mut logits = vec![0f32; VOCAB_SIZE];
            let exec = self.compilation.create_execution()?;
            exec.set_input_i32(0, &padded)?;
            exec.set_input_i32(1, &[pos])?;
            exec.set_output_f32(0, &mut logits)?;
            exec.compute()?;

            let next = logits
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i as u32)
                .unwrap_or(0);

            if next == EOS_TOKEN { break; }
            on_token(self.tokenizer.decode(&[next]));
            tokens.push(next as i32);
        }

        Ok(())
    }
}

// ── Model construction ───────────────────────────────────────────────────────

fn build_model(st: &SafeTensors<'_>) -> Result<nnapi::NnapiModel> {
    let mut b = GraphBuilder::new()?;

    // ── Model inputs: token_ids[MAX_CTX], pos_idx[1] ────────────────────────
    let token_ids = b.i32_tensor(&[MAX_CTX as u32])?;    // model input 0
    let pos_idx   = b.i32_tensor(&[1])?;                   // model input 1

    // ── Positional index vector [0..MAX_CTX) as constant ────────────────────
    let pos_ids_data: Vec<i32> = (0..MAX_CTX as i32).collect();
    let pos_ids = b.const_i32_tensor(&[MAX_CTX as u32], &pos_ids_data)?;

    // ── Embedding tables ─────────────────────────────────────────────────────
    let wte = load_const_2d(&mut b, st, "wte.weight", VOCAB_SIZE, N_EMBD)?;
    let wpe = load_const_2d(&mut b, st, "wpe.weight", 1024, N_EMBD)?;

    // tok_emb = wte[token_ids]  →  [MAX_CTX, N_EMBD]
    let tok_emb = b.gather(wte, 0, token_ids, &[MAX_CTX as u32, N_EMBD as u32])?;
    // pos_emb = wpe[0..MAX_CTX]  →  [MAX_CTX, N_EMBD]
    let pos_emb = b.gather(wpe, 0, pos_ids, &[MAX_CTX as u32, N_EMBD as u32])?;
    // x = tok_emb + pos_emb
    let mut x = b.add(tok_emb, pos_emb, &[MAX_CTX as u32, N_EMBD as u32])?;

    // ── Causal mask [1, MAX_CTX, MAX_CTX]: 0 = attend, -1e9 = masked ────────
    let mask_data = build_causal_mask(MAX_CTX);
    let causal_mask = b.const_f32_tensor(&[1, MAX_CTX as u32, MAX_CTX as u32], mask_data)?;

    // ── Transformer blocks ───────────────────────────────────────────────────
    for i in 0..N_LAYER {
        x = transformer_block(&mut b, st, i, x, causal_mask)?;
    }

    // ── Final layer norm ─────────────────────────────────────────────────────
    let ln_f_w = load_const_1d(&mut b, st, "ln_f.weight", N_EMBD)?;
    let ln_f_b = load_const_1d(&mut b, st, "ln_f.bias", N_EMBD)?;
    x = layer_norm(&mut b, x, ln_f_w, ln_f_b, MAX_CTX, N_EMBD)?;

    // ── LM head: select one position then project to vocab ───────────────────
    // Gather row at pos_idx  →  [1, N_EMBD]
    let last = b.gather(x, 0, pos_idx, &[1, N_EMBD as u32])?;

    // lm_head shares weights with wte [VOCAB_SIZE, N_EMBD]; already [out, in] for FC
    // Re-load so the model operand is separate (NNAPI requires unique operands per use)
    let lm_w = load_const_2d(&mut b, st, "wte.weight", VOCAB_SIZE, N_EMBD)?;
    let zero_bias = b.const_f32_tensor(&[VOCAB_SIZE as u32], vec![0.0f32; VOCAB_SIZE])?;
    // last [1, N_EMBD] × lm_w^T [VOCAB_SIZE, N_EMBD] → [1, VOCAB_SIZE]
    let logits_2d = b.fully_connected(last, lm_w, zero_bias, &[1, VOCAB_SIZE as u32])?;
    // Flatten to [VOCAB_SIZE] for output
    let logits = b.reshape(logits_2d, &[VOCAB_SIZE as i32], &[VOCAB_SIZE as u32])?;

    b.finish(&[token_ids, pos_idx], &[logits])
}

fn transformer_block(
    b: &mut GraphBuilder,
    st: &SafeTensors<'_>,
    i: usize,
    x: u32,
    causal_mask: u32,
) -> Result<u32> {
    let p = |s: &str| format!("h.{}.{}", i, s);
    let seq = MAX_CTX as u32;
    let hid = N_EMBD as u32;

    // LN1
    let ln1_w = load_const_1d(b, st, &p("ln_1.weight"), N_EMBD)?;
    let ln1_b = load_const_1d(b, st, &p("ln_1.bias"), N_EMBD)?;
    let ln1 = layer_norm(b, x, ln1_w, ln1_b, MAX_CTX, N_EMBD)?;

    // QKV projections — split c_attn weight [N_EMBD, 3*N_EMBD] into Q, K, V
    let (w_q, w_k, w_v, bq, bk, bv) = load_qkv(b, st, &p("attn.c_attn.weight"), &p("attn.c_attn.bias"))?;
    let q = b.fully_connected(ln1, w_q, bq, &[seq, hid])?;
    let k = b.fully_connected(ln1, w_k, bk, &[seq, hid])?;
    let v = b.fully_connected(ln1, w_v, bv, &[seq, hid])?;

    // Reshape + transpose: [seq, hid] → [n_head, seq, head_dim]
    let q_h = heads_split(b, q, seq)?;
    let k_h = heads_split(b, k, seq)?;
    let v_h = heads_split(b, v, seq)?;

    // Scores: Q @ K^T → [n_head, seq, seq]
    let scores = b.batch_matmul(q_h, k_h, false, true,
        &[N_HEAD as u32, seq, seq])?;
    // Scale by 1/sqrt(head_dim)
    let scale = b.const_f32_tensor(&[1], vec![1.0 / (HEAD_DIM as f32).sqrt()])?;
    let scaled = b.mul(scores, scale, &[N_HEAD as u32, seq, seq])?;
    // Apply causal mask (broadcasts [1, seq, seq] → [n_head, seq, seq])
    let masked = b.add(scaled, causal_mask, &[N_HEAD as u32, seq, seq])?;
    // Softmax over last dim
    let attn_w = b.softmax(masked, 1.0, 2, &[N_HEAD as u32, seq, seq])?;

    // attn_w @ V → [n_head, seq, head_dim]
    let attn_out = b.batch_matmul(attn_w, v_h, false, false,
        &[N_HEAD as u32, seq, HEAD_DIM as u32])?;

    // Merge heads: [n_head, seq, head_dim] → [seq, hid]
    let merged = heads_merge(b, attn_out, seq)?;

    // Output projection
    let c_proj_w = load_fc_weight(b, st, &p("attn.c_proj.weight"), N_EMBD, N_EMBD)?;
    let c_proj_b = load_const_1d(b, st, &p("attn.c_proj.bias"), N_EMBD)?;
    let attn_proj = b.fully_connected(merged, c_proj_w, c_proj_b, &[seq, hid])?;

    // Residual
    let x2 = b.add(x, attn_proj, &[seq, hid])?;

    // LN2
    let ln2_w = load_const_1d(b, st, &p("ln_2.weight"), N_EMBD)?;
    let ln2_b = load_const_1d(b, st, &p("ln_2.bias"), N_EMBD)?;
    let ln2 = layer_norm(b, x2, ln2_w, ln2_b, MAX_CTX, N_EMBD)?;

    // MLP
    let fc_w = load_fc_weight(b, st, &p("mlp.c_fc.weight"), N_EMBD, MLP_DIM)?;
    let fc_b = load_const_1d(b, st, &p("mlp.c_fc.bias"), MLP_DIM)?;
    let mlp1 = b.fully_connected(ln2, fc_w, fc_b, &[seq, MLP_DIM as u32])?;
    let mlp_gelu = gelu(b, mlp1, seq as usize, MLP_DIM)?;
    let proj_w = load_fc_weight(b, st, &p("mlp.c_proj.weight"), MLP_DIM, N_EMBD)?;
    let proj_b = load_const_1d(b, st, &p("mlp.c_proj.bias"), N_EMBD)?;
    let mlp2 = b.fully_connected(mlp_gelu, proj_w, proj_b, &[seq, hid])?;

    // Residual
    b.add(x2, mlp2, &[seq, hid])
}

// ── Composite ops ────────────────────────────────────────────────────────────

fn layer_norm(b: &mut GraphBuilder, x: u32, gamma: u32, beta: u32, seq: usize, hid: usize) -> Result<u32> {
    let s = seq as u32; let h = hid as u32;
    let mean  = b.mean(x, &[1], true, &[s, 1])?;
    let diff  = b.sub(x, mean, &[s, h])?;
    let dsq   = b.mul(diff, diff, &[s, h])?;
    let var   = b.mean(dsq, &[1], true, &[s, 1])?;
    let eps   = b.const_f32_tensor(&[1, 1], vec![1e-5f32])?;
    let ve    = b.add(var, eps, &[s, 1])?;
    let std   = b.sqrt(ve, &[s, 1])?;
    let norm  = b.div(diff, std, &[s, h])?;
    let sc    = b.mul(norm, gamma, &[s, h])?;
    b.add(sc, beta, &[s, h])
}

/// GELU: 0.5·x·(1 + tanh(√(2/π)·(x + 0.044715·x³)))
fn gelu(b: &mut GraphBuilder, x: u32, seq: usize, hid: usize) -> Result<u32> {
    let dims = [seq as u32, hid as u32];
    let c044  = b.const_f32_tensor(&[1], vec![0.044715f32])?;
    let csqrt = b.const_f32_tensor(&[1], vec![0.797_884_56f32])?; // sqrt(2/π)
    let cone  = b.const_f32_tensor(&[1], vec![1.0f32])?;
    let chalf = b.const_f32_tensor(&[1], vec![0.5f32])?;

    let x2  = b.mul(x, x, &dims)?;
    let x3  = b.mul(x2, x, &dims)?;
    let sc3 = b.mul(x3, c044, &dims)?;
    let sum = b.add(x, sc3, &dims)?;
    let inn = b.mul(sum, csqrt, &dims)?;
    let tnh = b.tanh(inn, &dims)?;
    let op  = b.add(tnh, cone, &dims)?;
    let hx  = b.mul(x, chalf, &dims)?;
    b.mul(hx, op, &dims)
}

/// [seq, N_EMBD] → reshape [seq, N_HEAD, HEAD_DIM] → transpose → [N_HEAD, seq, HEAD_DIM]
fn heads_split(b: &mut GraphBuilder, x: u32, seq: u32) -> Result<u32> {
    let r = b.reshape(x,
        &[seq as i32, N_HEAD as i32, HEAD_DIM as i32],
        &[seq, N_HEAD as u32, HEAD_DIM as u32])?;
    b.transpose(r, &[1, 0, 2], &[N_HEAD as u32, seq, HEAD_DIM as u32])
}

/// [N_HEAD, seq, HEAD_DIM] → transpose → [seq, N_HEAD, HEAD_DIM] → reshape → [seq, N_EMBD]
fn heads_merge(b: &mut GraphBuilder, x: u32, seq: u32) -> Result<u32> {
    let t = b.transpose(x, &[1, 0, 2], &[seq, N_HEAD as u32, HEAD_DIM as u32])?;
    b.reshape(t,
        &[seq as i32, N_EMBD as i32],
        &[seq, N_EMBD as u32])
}

// ── Weight loaders ───────────────────────────────────────────────────────────

/// Load a 1-D weight as a constant NNAPI operand.
fn load_const_1d(b: &mut GraphBuilder, st: &SafeTensors<'_>, name: &str, len: usize) -> Result<u32> {
    let floats = tensor_f32(st, name)?;
    b.const_f32_tensor(&[len as u32], floats)
}

/// Load a 2-D weight [rows, cols] — already transposed to [cols, rows] for FULLY_CONNECTED.
fn load_const_2d(b: &mut GraphBuilder, st: &SafeTensors<'_>, name: &str, rows: usize, cols: usize) -> Result<u32> {
    let floats = tensor_f32(st, name)?;
    // GPT-2 wte/wpe are [vocab/pos, embd] which IS already [rows, cols] for GATHER — no transpose needed.
    b.const_f32_tensor(&[rows as u32, cols as u32], floats)
}

/// Load a FC weight [in_size, out_size] and transpose → [out_size, in_size] for NNAPI.
fn load_fc_weight(b: &mut GraphBuilder, st: &SafeTensors<'_>, name: &str, in_size: usize, out_size: usize) -> Result<u32> {
    let floats = tensor_f32(st, name)?;
    let transposed = transpose_2d(&floats, in_size, out_size);
    b.const_f32_tensor(&[out_size as u32, in_size as u32], transposed)
}

/// Split c_attn weight [N_EMBD, 3*N_EMBD] into separate Q, K, V FC weights + biases.
fn load_qkv(
    b: &mut GraphBuilder,
    st: &SafeTensors<'_>,
    w_name: &str,
    bias_name: &str,
) -> Result<(u32, u32, u32, u32, u32, u32)> {
    let w = tensor_f32(st, w_name)?;    // [N_EMBD, 3*N_EMBD]
    let bias = tensor_f32(st, bias_name)?; // [3*N_EMBD]

    // Each slice is [N_EMBD, N_EMBD] in the original [in, out] layout.
    let slice_w = |start: usize| -> Vec<f32> {
        (0..N_EMBD).flat_map(|r| {
            let base = r * (3 * N_EMBD) + start;
            w[base..base + N_EMBD].iter().copied()
        }).collect()
    };

    let wq = transpose_2d(&slice_w(0),           N_EMBD, N_EMBD);
    let wk = transpose_2d(&slice_w(N_EMBD),      N_EMBD, N_EMBD);
    let wv = transpose_2d(&slice_w(2 * N_EMBD),  N_EMBD, N_EMBD);

    let bq_data = bias[..N_EMBD].to_vec();
    let bk_data = bias[N_EMBD..2 * N_EMBD].to_vec();
    let bv_data = bias[2 * N_EMBD..].to_vec();

    let dims_w = [N_EMBD as u32, N_EMBD as u32];
    let dims_b = [N_EMBD as u32];

    Ok((
        b.const_f32_tensor(&dims_w, wq)?,
        b.const_f32_tensor(&dims_w, wk)?,
        b.const_f32_tensor(&dims_w, wv)?,
        b.const_f32_tensor(&dims_b, bq_data)?,
        b.const_f32_tensor(&dims_b, bk_data)?,
        b.const_f32_tensor(&dims_b, bv_data)?,
    ))
}

// ── SafeTensors helpers ──────────────────────────────────────────────────────

fn tensor_f32(st: &SafeTensors<'_>, name: &str) -> Result<Vec<f32>> {
    let tv = st.tensor(name).with_context(|| format!("missing tensor: {}", name))?;
    as_f32(tv.data(), tv.dtype())
}

fn as_f32(data: &[u8], dtype: safetensors::Dtype) -> Result<Vec<f32>> {
    match dtype {
        safetensors::Dtype::F32 => Ok(data
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect()),
        safetensors::Dtype::F16 => Ok(data
            .chunks_exact(2)
            .map(|b| f16_to_f32(u16::from_le_bytes([b[0], b[1]])))
            .collect()),
        safetensors::Dtype::BF16 => Ok(data
            .chunks_exact(2)
            .map(|b| f32::from_bits((u16::from_le_bytes([b[0], b[1]]) as u32) << 16))
            .collect()),
        other => anyhow::bail!("unsupported dtype {:?}", other),
    }
}

fn f16_to_f32(bits: u16) -> f32 {
    let sign = ((bits >> 15) & 1) as u32;
    let exp  = ((bits >> 10) & 0x1f) as u32;
    let mant = (bits & 0x3ff) as u32;
    if exp == 0 {
        let v = mant as f32 / (1 << 24) as f32;
        return if sign == 0 { v } else { -v };
    }
    if exp == 31 {
        return if mant == 0 {
            if sign == 0 { f32::INFINITY } else { f32::NEG_INFINITY }
        } else { f32::NAN };
    }
    f32::from_bits((sign << 31) | ((exp + 127 - 15) << 23) | (mant << 13))
}

fn build_causal_mask(size: usize) -> Vec<f32> {
    let mut m = vec![0.0f32; size * size];
    for i in 0..size {
        for j in (i + 1)..size {
            m[i * size + j] = -1e9;
        }
    }
    m
}
