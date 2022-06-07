use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::watch;
use tokio::task;

use crate::color::Color;

pub struct LedEffectProcessor {
    handle: task::JoinHandle<()>,
    sender: Arc<watch::Sender<Color>>,
}

pub struct InfinityLedEffect<T>
where
    T: LedEffect + Sync + Send,
{
    effect: Box<T>,
}

pub trait InfinityEffect
where
    Self: LedEffect + Sized,
{
    fn as_infinity(self: Box<Self>) -> Box<InfinityLedEffect<Self>> {
        Box::new(InfinityLedEffect { effect: self })
    }
}

impl LedEffectProcessor {
    pub fn new(sender: watch::Sender<Color>) -> Self {
        Self {
            handle: tokio::spawn(async {}),
            sender: Arc::new(sender),
        }
    }

    fn start_effect(&mut self, effect: Box<dyn LedEffect>) {
        self.handle.abort();
        let sender = self.sender.clone();
        self.handle = tokio::spawn(async move {
            effect.run(sender).await;
        });
    }
}

#[async_trait]
pub trait LedEffect: Sync + Send {
    async fn run(&self, out: Arc<watch::Sender<Color>>);

    fn run_in(self: Box<Self>, processor: &mut LedEffectProcessor)
    where
        Self: 'static + Sized,
    {
        processor.start_effect(self as Box<dyn LedEffect>);
    }
}

#[async_trait]
impl<T> LedEffect for InfinityLedEffect<T>
where
    T: LedEffect + Sync + Send,
{
    async fn run(&self, out: Arc<watch::Sender<Color>>) {
        loop {
            self.effect.run(out.clone()).await;
        }
    }
}
