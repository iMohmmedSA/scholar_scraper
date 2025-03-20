use super::{csv::{load::csv_load, model::PreScholar}, scholar_model::Scholar};
use std::sync::Arc;
use tokio::sync::Mutex;



pub struct App {
    pub pre_scholars: Vec<PreScholar>,
    pub scholars: Arc<Mutex<Vec<Scholar>>>,
    pub thread_count: usize,
}

impl Default for App {
    fn default() -> Self {
        let pre_scholars = csv_load().expect("Make sure authorlist.csv exist, with 'google_id' as header");

        Self {
            pre_scholars,
            scholars: Default::default(),
            thread_count: 10,
        }
    }
}


impl App {
    pub async fn add_scholar(s: Arc<Mutex<Vec<Scholar>>>, new_scholar: Scholar) {
        let mut scholars = s.lock().await;
        scholars.push(new_scholar);
    }    
}
