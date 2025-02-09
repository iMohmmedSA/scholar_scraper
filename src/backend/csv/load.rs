use csv::Reader;
use log::info;
use tracing::error;
use crate::prelude::*;

use crate::backend::utils::retrieve_list_from_input;

use super::model::PreScholar;

pub fn csv_load() -> Result<Vec<PreScholar>>{
    let path = match retrieve_list_from_input() {
        Ok(ok) => ok,
        Err(_) => "authorlist.csv".to_string(),
    };

    info!("File path: {path}");

    let mut reader = match Reader::from_path(path) {
        Ok(r) => r,
        Err(_) => {
              error!("Failed to read the file");
              return Err(Error::ReadFile);
        },
    };
    
    let result: Vec<PreScholar> = reader.deserialize().filter_map(csv::Result::ok).collect();
    Ok(result)
}