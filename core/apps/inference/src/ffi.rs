use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use crate::engine::InferenceEngine;
use crate::engine_nnapi::NnapiInferenceEngine;

enum Backend {
    Nnapi(NnapiInferenceEngine),
    Burn(InferenceEngine),
}

impl Backend {
    fn generate<F>(&self, prompt: &str, max_tokens: usize, on_token: F) -> anyhow::Result<()>
    where
        F: FnMut(String),
    {
        match self {
            Backend::Nnapi(e) => e.generate(prompt, max_tokens, on_token),
            Backend::Burn(e)  => e.generate(prompt, max_tokens, on_token),
        }
    }
}

#[derive(uniffi::Object)]
pub struct TokenStream {
    rx: Mutex<mpsc::Receiver<Option<String>>>,
}

#[uniffi::export]
impl TokenStream {
    pub fn next_token(&self) -> Option<String> {
        self.rx.lock().ok()?.recv().unwrap_or(None)
    }
}

#[derive(uniffi::Object)]
pub struct NativeInferenceEngine {
    backend:    Arc<Mutex<Option<Backend>>>,
    load_error: Arc<Mutex<Option<String>>>,
}

#[uniffi::export]
impl NativeInferenceEngine {
    #[uniffi::constructor]
    pub fn new(model_dir: String) -> Arc<Self> {
        let mut last_err: Option<String> = None;

        // Try NNAPI first; on failure capture error and try Burn CPU.
        let backend = NnapiInferenceEngine::load(&model_dir)
            .map(Backend::Nnapi)
            .or_else(|e_nnapi| {
                last_err = Some(format!("{:#}", e_nnapi));
                InferenceEngine::load(&model_dir)
                    .map(Backend::Burn)
                    .map_err(|e_burn| {
                        // Burn error is more user-readable (tensor names etc.)
                        last_err = Some(format!("{:#}", e_burn));
                        e_burn
                    })
            })
            .ok();

        if backend.is_some() {
            last_err = None;
        }

        Arc::new(Self {
            backend:    Arc::new(Mutex::new(backend)),
            load_error: Arc::new(Mutex::new(last_err)),
        })
    }

    pub fn is_ready(&self) -> bool {
        self.backend.lock().map(|g| g.is_some()).unwrap_or(false)
    }

    /// Returns the Rust error string when is_ready() == false.
    pub fn load_error(&self) -> Option<String> {
        self.load_error.lock().ok()?.clone()
    }

    pub fn generate_stream(&self, prompt: String, max_tokens: u32) -> Arc<TokenStream> {
        let (tx, rx) = mpsc::channel::<Option<String>>();
        let backend  = Arc::clone(&self.backend);

        std::thread::spawn(move || {
            let guard = match backend.lock() {
                Ok(g)  => g,
                Err(_) => { let _ = tx.send(None); return; }
            };
            if let Some(b) = guard.as_ref() {
                let _ = b.generate(&prompt, max_tokens as usize, |token| {
                    let _ = tx.send(Some(token));
                });
            }
            let _ = tx.send(None);
        });

        Arc::new(TokenStream { rx: Mutex::new(rx) })
    }
}
