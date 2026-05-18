use std::collections::HashMap;
use anyhow::{Context, Result};
use fancy_regex::Regex;

pub struct Tokenizer {
    vocab: HashMap<String, u32>,
    id_to_token: Vec<String>,
    merge_rank: HashMap<(String, String), usize>,
    byte_encoder: HashMap<u8, char>,
    byte_decoder: HashMap<char, u8>,
    pattern: Regex,
}

impl Tokenizer {
    pub fn from_files(vocab_path: &str, merges_path: &str) -> Result<Self> {
        let vocab_str = std::fs::read_to_string(vocab_path)
            .with_context(|| format!("reading vocab: {}", vocab_path))?;
        let vocab: HashMap<String, u32> = serde_json::from_str(&vocab_str)?;

        let vocab_size = vocab.len();
        let mut id_to_token = vec![String::new(); vocab_size];
        for (token, id) in &vocab {
            if (*id as usize) < vocab_size {
                id_to_token[*id as usize] = token.clone();
            }
        }

        let merges_str = std::fs::read_to_string(merges_path)
            .with_context(|| format!("reading merges: {}", merges_path))?;
        let merge_rank: HashMap<(String, String), usize> = merges_str
            .lines()
            .skip(1)
            .filter(|l| !l.is_empty())
            .enumerate()
            .map(|(i, l)| {
                let mut parts = l.splitn(2, ' ');
                let a = parts.next().unwrap_or("").to_string();
                let b = parts.next().unwrap_or("").to_string();
                ((a, b), i)
            })
            .collect();

        let byte_encoder = build_byte_encoder();
        let byte_decoder = byte_encoder.iter().map(|(&k, &v)| (v, k)).collect();

        let pattern = Regex::new(
            r"'(?:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+"
        )?;

        Ok(Self {
            vocab,
            id_to_token,
            merge_rank,
            byte_encoder,
            byte_decoder,
            pattern,
        })
    }

    pub fn encode(&self, text: &str) -> Vec<u32> {
        let mut token_ids = Vec::new();
        for piece in self.pattern.find_iter(text).flatten() {
            let word = piece.as_str();
            let byte_word: String = word
                .bytes()
                .map(|b| self.byte_encoder[&b])
                .collect();
            let bpe_tokens = self.bpe(&byte_word);
            for t in bpe_tokens {
                if let Some(&id) = self.vocab.get(&t) {
                    token_ids.push(id);
                }
            }
        }
        token_ids
    }

    pub fn decode(&self, ids: &[u32]) -> String {
        let byte_string: String = ids
            .iter()
            .filter_map(|&id| self.id_to_token.get(id as usize))
            .cloned()
            .collect();
        let bytes: Vec<u8> = byte_string
            .chars()
            .filter_map(|c| self.byte_decoder.get(&c).copied())
            .collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    fn bpe(&self, word: &str) -> Vec<String> {
        let mut symbols: Vec<String> = word.chars().map(|c| c.to_string()).collect();
        if symbols.len() <= 1 {
            return symbols;
        }

        loop {
            let mut best_rank = usize::MAX;
            let mut best_pair: Option<(usize, usize)> = None;

            for i in 0..symbols.len() - 1 {
                let pair = (symbols[i].clone(), symbols[i + 1].clone());
                if let Some(&rank) = self.merge_rank.get(&pair) {
                    if rank < best_rank {
                        best_rank = rank;
                        best_pair = Some((i, i + 1));
                    }
                }
            }

            match best_pair {
                None => break,
                Some((i, j)) => {
                    let merged = format!("{}{}", symbols[i], symbols[j]);
                    symbols[i] = merged;
                    symbols.remove(j);
                }
            }
        }

        symbols
    }
}

fn build_byte_encoder() -> HashMap<u8, char> {
    let mut bs: Vec<u8> = Vec::new();
    let mut cs: Vec<u32> = Vec::new();

    for b in b'!'..=b'~' {
        bs.push(b);
        cs.push(b as u32);
    }
    for b in 0xA1u8..=0xAC {
        bs.push(b);
        cs.push(b as u32);
    }
    for b in 0xAEu8..=0xFF {
        bs.push(b);
        cs.push(b as u32);
    }

    let mut n = 0u32;
    for b in 0u8..=255 {
        if !bs.contains(&b) {
            bs.push(b);
            cs.push(256 + n);
            n += 1;
        }
    }

    bs.into_iter()
        .zip(cs.into_iter().filter_map(char::from_u32))
        .collect()
}
