use super::{csv::{load::{csv_load, handle_csv_error}, model::PreScholar}, scholar_model::Scholar};




pub struct App {
    pub pre_scholars: Vec<PreScholar>,
    pub scholars: Vec<Scholar>
}

impl Default for App {
    fn default() -> Self {
        let pre_scholars = match csv_load() {
            Ok(s) => s,
            Err(e) => {handle_csv_error(e); std::process::exit(1);},
        };

        Self {
            pre_scholars,
            scholars: Default::default()
        }
    }
}

