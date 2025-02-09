use std::sync::Mutex;

use once_cell::sync::Lazy;
use tracing::{info, level_filters::LevelFilter, Level};
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry};


static LOG_GUARD: Lazy<Mutex<Option<tracing_appender::non_blocking::WorkerGuard>>> = Lazy::new(|| Mutex::new(None));

pub fn start_log() {
    let file_appender = rolling::daily("logs", "scholar_scraper");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    *LOG_GUARD.lock().unwrap() = Some(guard);

    let file_layer = fmt::Layer::<Registry>::default()
    .with_writer(non_blocking) // Log to file
    .with_target(true)         // Show module names
    .with_level(true)          // Show log level
    .with_thread_ids(true)     // Include thread ID
    .with_ansi(false)          // Disable colors for file logs
    .compact();                // Use compact log format

    let filter = LevelFilter::from_level(Level::INFO);
    tracing_subscriber::registry()
    .with(file_layer)
    .with(filter)
    .init();

    info!("Log started");
}