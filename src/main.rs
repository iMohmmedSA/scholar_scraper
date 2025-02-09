use std::{io, time::Instant};

use backend::{
    app::App,
    google_scholar::process, log::start_log,
};

pub mod backend;
pub mod error;
pub mod prelude;

#[tokio::main]
async fn main() {
    start_log();

    let start = Instant::now();

    let mut app = App::default();

    println!("Enter the number of tasks to use (default 10): ");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    app.thread_count = input.trim().parse().unwrap_or(10);

    println!("---------------------------");
    println!("Scholar count: {}", app.pre_scholars.len());
    println!("Number of thread: {}", app.thread_count);
    println!("---------------------------");

    process(&mut app).await;

    let duration = start.elapsed();
    println!("Time taken: {:.2?} seconds", duration.as_secs_f64());
}