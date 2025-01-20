


#[derive(Debug)]
pub struct Scholar {
    google_id: String,
    name: String,
    affiliation: String,
    // document_count: u32,
    
    cited_by: String,
    cited_5_years: String,
    
    h_index: String,
    h_index_5_years: String,
    
    i10_index: String,
    i10_index_5_years: String,
}

impl Scholar {
    pub fn new(google_id: String, name: String, affiliation: String, cited_by: String, cited_5_years: String, h_index: String, h_index_5_years: String, i10_index: String, i10_index_5_years: String) -> Self {
        Self {
            google_id,
            name,
            affiliation,
            // document_count: 0,
            cited_by,
            cited_5_years,
            h_index,
            h_index_5_years,
            i10_index,
            i10_index_5_years,
        }
    }
}