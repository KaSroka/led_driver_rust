use std::sync::Arc;

use tokio::sync::watch;
use tokio::task;

use crate::color::Color;
use crate::led_effect::LedEffect;

pub struct LedEffectProcessor {
    handle: task::JoinHandle<()>,
    sender: Arc<watch::Sender<Color>>,
}

impl LedEffectProcessor {
    pub fn new(sender: watch::Sender<Color>) -> Self {
        Self {
            handle: tokio::spawn(async {}),
            sender: Arc::new(sender),
        }
    }

    pub fn start_effect(&mut self, effect: Box<dyn LedEffect>) {
        self.handle.abort();
        let sender = self.sender.clone();
        self.handle = tokio::spawn(async move {
            effect.run(sender).await;
        });
    }
}
