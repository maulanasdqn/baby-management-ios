use anyhow::{anyhow, Context, Result};
use burn::tensor::{backend::Backend, Tensor, TensorData};
use memmap2::MmapOptions;
use safetensors::SafeTensors;
use std::fs::File;

use crate::model::{Block, Gpt2, Gpt2Config};

pub fn load_gpt2<B: Backend>(model_dir: &str, device: &B::Device) -> Result<Gpt2<B>> {
    let path = format!("{}/model.safetensors", model_dir);
    let file = File::open(&path).with_context(|| format!("opening {}", path))?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    let tensors = SafeTensors::deserialize(&mmap)?;
    let config = Gpt2Config::gpt2_small();

    // HuggingFace GPT-2 safetensors (openai-community/gpt2) prefixes every
    // tensor with "transformer."; raw/converted checkpoints may not.
    let prefix = if tensors.names().iter().any(|n| n.starts_with("transformer.")) {
        "transformer."
    } else {
        ""
    };
    let tp = |s: &str| format!("{}{}", prefix, s);

    let wte    = load_2d::<B>(&tensors, &tp("wte.weight"),  device)?;
    let wpe    = load_2d::<B>(&tensors, &tp("wpe.weight"),  device)?;
    let ln_f_w = load_1d::<B>(&tensors, &tp("ln_f.weight"), device)?;
    let ln_f_b = load_1d::<B>(&tensors, &tp("ln_f.bias"),   device)?;

    let mut blocks = Vec::with_capacity(config.n_layer);
    for i in 0..config.n_layer {
        blocks.push(load_block::<B>(&tensors, i, &config, device, prefix)?);
    }

    Ok(Gpt2 { wte, wpe, blocks, ln_f_w, ln_f_b, config })
}

fn load_block<B: Backend>(
    tensors: &SafeTensors<'_>,
    idx: usize,
    config: &Gpt2Config,
    device: &B::Device,
    prefix: &str,
) -> Result<Block<B>> {
    let p = |s: &str| format!("{}h.{}.{}", prefix, idx, s);
    Ok(Block {
        ln1_w:      load_1d::<B>(tensors, &p("ln_1.weight"),        device)?,
        ln1_b:      load_1d::<B>(tensors, &p("ln_1.bias"),          device)?,
        ln2_w:      load_1d::<B>(tensors, &p("ln_2.weight"),        device)?,
        ln2_b:      load_1d::<B>(tensors, &p("ln_2.bias"),          device)?,
        c_attn_w:   load_2d::<B>(tensors, &p("attn.c_attn.weight"), device)?,
        c_attn_b:   load_1d::<B>(tensors, &p("attn.c_attn.bias"),   device)?,
        c_proj_w:   load_2d::<B>(tensors, &p("attn.c_proj.weight"), device)?,
        c_proj_b:   load_1d::<B>(tensors, &p("attn.c_proj.bias"),   device)?,
        mlp_fc_w:   load_2d::<B>(tensors, &p("mlp.c_fc.weight"),    device)?,
        mlp_fc_b:   load_1d::<B>(tensors, &p("mlp.c_fc.bias"),      device)?,
        mlp_proj_w: load_2d::<B>(tensors, &p("mlp.c_proj.weight"),  device)?,
        mlp_proj_b: load_1d::<B>(tensors, &p("mlp.c_proj.bias"),    device)?,
        n_head:     config.n_head,
        n_embd:     config.n_embd,
    })
}

fn load_1d<B: Backend>(
    tensors: &SafeTensors<'_>,
    name: &str,
    device: &B::Device,
) -> Result<Tensor<B, 1>> {
    let tv = tensors.tensor(name).with_context(|| format!("missing tensor: {}", name))?;
    let floats = as_f32(tv.data(), tv.dtype())?;
    let len = floats.len();
    let td = TensorData::new(floats, [len]);
    Ok(Tensor::<B, 1>::from_data(td, device))
}

fn load_2d<B: Backend>(
    tensors: &SafeTensors<'_>,
    name: &str,
    device: &B::Device,
) -> Result<Tensor<B, 2>> {
    let tv = tensors.tensor(name).with_context(|| format!("missing tensor: {}", name))?;
    let shape = tv.shape().to_vec();
    if shape.len() != 2 {
        return Err(anyhow!("{}: expected rank-2, got rank-{}", name, shape.len()));
    }
    let floats = as_f32(tv.data(), tv.dtype())?;
    let total = floats.len();
    let td = TensorData::new(floats, [total]);
    Ok(Tensor::<B, 1>::from_data(td, device).reshape([shape[0], shape[1]]))
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
        other => Err(anyhow!("unsupported dtype {:?}", other)),
    }
}

fn f16_to_f32(bits: u16) -> f32 {
    let sign = ((bits >> 15) & 1) as u32;
    let exp  = ((bits >> 10) & 0x1f) as u32;
    let mant =  (bits & 0x3ff) as u32;
    if exp == 0 {
        let v = mant as f32 / (1 << 24) as f32;
        return if sign == 0 { v } else { -v };
    }
    if exp == 31 {
        return if mant == 0 {
            if sign == 0 { f32::INFINITY } else { f32::NEG_INFINITY }
        } else {
            f32::NAN
        };
    }
    f32::from_bits((sign << 31) | ((exp + 127 - 15) << 23) | (mant << 13))
}
