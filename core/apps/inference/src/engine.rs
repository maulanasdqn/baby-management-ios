use anyhow::Result;
use burn::backend::ndarray::{NdArray, NdArrayDevice};

use crate::loader::load_gpt2;
use crate::model::Gpt2;
use crate::tokenizer::Tokenizer;

type B = NdArray<f32>;

const SYSTEM_CONTEXT: &str =
    "The following is a conversation with Baby AI, a helpful baby-care assistant. \
     Baby AI only discusses feeding, sleep, diapers, growth, and baby milestones.\n\
     Parent:";

const EOS_TOKEN: u32 = 50256;
const MAX_CTX: usize = 128;

pub struct InferenceEngine {
    model: Gpt2<B>,
    tokenizer: Tokenizer,
}

impl InferenceEngine {
    pub fn load(model_dir: &str) -> Result<Self> {
        let device = NdArrayDevice::default();
        let model = load_gpt2::<B>(model_dir, &device)?;
        let vocab_path = format!("{}/vocab.json", model_dir);
        let merges_path = format!("{}/merges.txt", model_dir);
        let tokenizer = Tokenizer::from_files(&vocab_path, &merges_path)?;
        Ok(Self { model, tokenizer })
    }

    pub fn generate<F>(&self, user_message: &str, max_tokens: usize, mut on_token: F) -> Result<()>
    where
        F: FnMut(String),
    {
        let prompt = format!("{} {}\nBaby AI:", SYSTEM_CONTEXT, user_message.trim());
        let mut tokens = self.tokenizer.encode(&prompt);
        tokens.truncate(MAX_CTX.saturating_sub(max_tokens));

        for _ in 0..max_tokens {
            let context = if tokens.len() > MAX_CTX {
                &tokens[tokens.len() - MAX_CTX..]
            } else {
                &tokens
            };

            let logits = self.model.forward(context);
            let next = greedy_argmax(&logits);

            if next == EOS_TOKEN {
                break;
            }

            on_token(self.tokenizer.decode(&[next]));
            tokens.push(next);
        }

        Ok(())
    }
}

fn greedy_argmax(logits: &burn::tensor::Tensor<B, 1>) -> u32 {
    let data = logits.clone().to_data();
    let floats: Vec<f32> = data.to_vec::<f32>().unwrap_or_default();
    floats
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i as u32)
        .unwrap_or(0)
}
