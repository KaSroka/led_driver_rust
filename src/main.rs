mod color;
mod led_effect_processor;
mod led_effect;

use color::Color;
use colored::Colorize;
use led_effect_processor::LedEffectProcessor;
use led_effect::{LedEffect, InfinityEffect, BlinkEffect, PulseEffect};
use tokio::sync::watch;
use tokio::time;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (sender, mut receiver) = watch::channel(Color::WHITE);

    tokio::spawn(async move {
        while receiver.changed().await.is_ok() {
            let color = receiver.borrow();
            let rect = "  ".on_truecolor(color.r, color.g, color.b);
            println!("New color {}", rect);
        }
    });

    let mut proc = LedEffectProcessor::new(sender);

    BlinkEffect::new(
        time::Duration::from_millis(1000),
        time::Duration::from_millis(2000),
        Color::RED,
    )
    .run_in(&mut proc);

    time::sleep(time::Duration::from_secs(5)).await;

    PulseEffect::new(
        time::Duration::from_millis(1000),
        time::Duration::from_millis(2000),
        time::Duration::from_millis(100),
        Color::GREEN,
    )
    .as_infinity()
    .run_in(&mut proc);

    time::sleep(time::Duration::from_secs(5)).await;

    BlinkEffect::new(
        time::Duration::from_millis(100),
        time::Duration::from_millis(200),
        Color::BLUE,
    )
    .run_in(&mut proc);

    time::sleep(time::Duration::from_secs(1)).await;

    PulseEffect::new(
        time::Duration::from_millis(1000),
        time::Duration::from_millis(1000),
        time::Duration::from_millis(100),
        Color::from_rgb(255, 255, 0),
    )
    .run_in(&mut proc);

    time::sleep(time::Duration::from_secs(5)).await;

    PulseEffect::new(
        time::Duration::from_millis(0),
        time::Duration::from_millis(1000),
        time::Duration::from_millis(100),
        Color::from_rgb(255, 0, 255),
    )
    .run_in(&mut proc);

    time::sleep(time::Duration::from_secs(5)).await;
}
