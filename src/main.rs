use std::time::Instant;

use backend::{
    app::App,
    google_scholar::process,
};

pub mod backend;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let mut app = App::default();

    println!("---------------------------");
    println!("Scholar count: {}", app.pre_scholars.len());
    println!("---------------------------");

    process(&mut app).await;

    let duration = start.elapsed();
    println!("Time taken: {:.2?} seconds", duration.as_secs_f64());
}
