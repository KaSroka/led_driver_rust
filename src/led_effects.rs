use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::watch;
use tokio::time;

use crate::led_effect_processor::{InfinityEffect, LedEffect, Color};

pub struct BlinkEffect {
    on_time: time::Duration,
    off_time: time::Duration,
    color: Color,
}

impl BlinkEffect {
    pub fn new(on_time: time::Duration, off_time: time::Duration, color: Color) -> Box<Self> {
        Box::new(Self { on_time, off_time, color })
    }
}

#[async_trait]
impl LedEffect for BlinkEffect {
    async fn run(&self, out: Arc<watch::Sender<Color>>) {
        out.send(self.color.with_intensity(1.0)).expect("Output channel closed");
        time::sleep(self.on_time).await;
        out.send(self.color.with_intensity(0.0)).expect("Output channel closed");
        time::sleep(self.off_time).await;
    }
}

impl InfinityEffect for BlinkEffect {}

pub struct PulseEffect {
    on_time: time::Duration,
    off_time: time::Duration,
    on_ticks: usize,
    off_ticks: usize,
    refresh_interval: time::Duration,
    color: Color,
}

impl PulseEffect {
    pub fn new(
        on_time: time::Duration,
        period: time::Duration,
        refresh_interval: time::Duration,
        color: Color,
    ) -> Box<Self> {
        let off_time = period - on_time;
        let on_ticks = (on_time.as_secs_f64() / refresh_interval.as_secs_f64()).round() as usize;
        let off_ticks = (off_time.as_secs_f64() / refresh_interval.as_secs_f64()).round() as usize;

        Box::new(Self {
            on_time,
            off_time,
            on_ticks,
            off_ticks,
            refresh_interval,
            color,
        })
    }
}

#[async_trait]
impl LedEffect for PulseEffect {
    async fn run(&self, out: Arc<watch::Sender<Color>>) {
        for i in 1..=self.on_ticks {
            let val = (i as f32 * self.refresh_interval.as_secs_f32() / self.on_time.as_secs_f32())
                .clamp(0.0, 1.0);

            out.send(self.color.with_intensity(val)).expect("Output channel closed");

            time::sleep(self.refresh_interval).await;
        }

        for i in 1..=self.off_ticks {
            let val = (1.0
                - i as f32 * self.refresh_interval.as_secs_f32() / self.off_time.as_secs_f32())
            .clamp(0.0, 1.0);

            out.send(self.color.with_intensity(val)).expect("Output channel closed");

            time::sleep(self.refresh_interval).await;
        }
    }
}

impl InfinityEffect for PulseEffect {}
