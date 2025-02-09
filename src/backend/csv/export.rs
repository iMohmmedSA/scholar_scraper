use chrono::Local;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};
use tracing::info;

use crate::backend::scholar_model::Scholar;

pub async fn export_data(scholars: Vec<Scholar>) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let file = File::create(format!("scholar_data_{now}.csv"))
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to create CSV writer: {:?}", err);
        });
    let mut writer = BufWriter::new(file);

    write_header(&mut writer, get_publication_size(&scholars)).await;

    process_list(&mut writer, scholars).await;

    writer.flush().await.unwrap_or_default();
}

async fn write_header(writer: &mut BufWriter<File>, size: usize) {
    let mut header = "google_id,name,affiliation,document_count,cited_by,cited_5_years,h_index,h_index_5_years,i10_index,i10_index_5_years".to_string();

    for i in 0..size {
        header.push_str(format!(",publication_title_{i},journal,year,cited_by").as_str());
    }

    header.push_str("\n");

    writer
        .write_all(header.as_bytes())
        .await
        .unwrap_or_default();
}

fn get_publication_size(scholars: &Vec<Scholar>) -> usize {
    let mut max = 0;

    for scholar in scholars {
        if max < scholar.publication.len() {
            max = scholar.publication.len();
        }
    }

    max
}

async fn process_list(writer: &mut BufWriter<File>, scholars: Vec<Scholar>) {
    info!("{:?}", scholars);
    for scholar in scholars {
        write_row(writer, scholar).await;
    }
}

async fn write_row(writer: &mut BufWriter<File>, scholar: Scholar) {
    let mut str = format!(
        "{},{},{},{},{},{},{},{},{},{}",
        scholar.google_id,
        scholar.name,
        scholar.affiliation,
        scholar.document_count,
        scholar.cited_by,
        scholar.cited_5_years,
        scholar.h_index,
        scholar.h_index_5_years,
        scholar.i10_index,
        scholar.i10_index_5_years
    );

    for publication in scholar.publication {
        str.push_str(format!(
            ",{},{},{},{}",
            escape_csv(publication.title.clone()),
            escape_csv(publication.journal.clone()),
            publication.year,
            publication.cited_by
        ).as_str());
    }

    str.push_str("\n");

    writer.write_all(str.as_bytes()).await.unwrap_or_default();
}

fn escape_csv(field: String) -> String {
    format!("\"{}\"", field)
}