use serde::Serialize;




#[derive(Debug, Clone, Serialize)]
pub struct Scholar {
    pub google_id: String,
    pub name: String,
    pub affiliation: String,
    pub document_count: usize,
    
    pub cited_by: String,
    pub cited_5_years: String,
    
    pub h_index: String,
    pub h_index_5_years: String,
    
    pub i10_index: String,
    pub i10_index_5_years: String,

    pub publication: Vec<Publication>
}

impl Scholar {
    pub fn new(google_id: String, name: String, affiliation: String, document_count: usize, cited_by: String, cited_5_years: String, h_index: String, h_index_5_years: String, i10_index: String, i10_index_5_years: String, publication: Vec<Publication>) -> Self {
        Self {
            google_id,
            name,
            affiliation,
            document_count,
            cited_by,
            cited_5_years,
            h_index,
            h_index_5_years,
            i10_index,
            i10_index_5_years,
            publication
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct Publication {
    pub title: String,
    pub journal: String,
    pub year: String,
    pub cited_by: String
}

impl Publication {
    pub fn new(title: String, journal: String, year: String, cited_by: String) -> Self {
        Self { title, journal, year, cited_by }
    }
}