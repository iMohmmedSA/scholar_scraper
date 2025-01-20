use csv::{Error, Reader};

use super::model::PreScholar;

pub fn csv_load() -> Result<Vec<PreScholar>, Error>{
    let path = "authorlist.csv";
    let mut reader = Reader::from_path(path)?;
    let result: Vec<PreScholar> = reader.deserialize().filter_map(Result::ok).collect();
    Ok(result)
}

pub fn handle_csv_error(e: Error) {
    panic!("Error: {:?}", e);
}