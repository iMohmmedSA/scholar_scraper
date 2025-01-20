use std::sync::Arc;

use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use tokio::sync::Semaphore;
use tokio::task::{self, JoinHandle};

use crate::backend::scholar_model::Scholar;

use super::app::App;

pub async fn process(app: &mut App) {
    let semaphore = Arc::new(Semaphore::new(10));

    let tasks: Vec<JoinHandle<()>> = app.pre_scholars.clone()
    .into_iter().map(|task| {
        let semaphore = semaphore.clone();
        task::spawn(async move {
            if let Ok(_permit) = semaphore.acquire().await {
                extract(task.google_id).await;
            } else {
                eprintln!("Failed to acquire semaphore permit.");
            }
        })
    })
    .collect();

    for task in tasks {
        let _ = task.await;
    }
}

async fn extract(id: String) {
    println!("Processing: {id}");

    let result = match fetch(&id).await {
        Ok(o) => o,
        Err(_) => todo!("Fix the extract"),
    };

    let html = Html::parse_document(&result);

    let name = extract_content(&html, "div#gsc_prf_in");
    let affiliation = extract_content(&html, "div.gsc_prf_il > a.gsc_prf_ila");
    let (cited_by, cited_5_years, h_index_all, h_index_5, i10_index_all, i10_index_5) = extract_citations_table(&html);

    let scholar: Scholar = Scholar::new(id, 
        name, affiliation, cited_by, cited_5_years, h_index_all, h_index_5, i10_index_all, i10_index_5);

    println!("{:?}", scholar);
}

// TODO: what happen if it failed? should i note that it failed? drop the scholar?
async fn fetch(id: &String) -> Result<String, reqwest::Error> {
    let url = format!("https://scholar.google.com/citations?user={id}&hl=en");
    let client = Client::new();
    let response = client.get(url).send().await?;
    response.text().await.map_err(|e| e.into())
}

fn extract_content(html: &Html, selecter: &str) -> String {
    let selector = match Selector::parse(selecter) {
        Ok(s) => s,
        Err(_) => return String::from("Unknown"),
    };
    
    if let Some(element) = html.select(&selector).next() {
        let content = element.text().collect::<Vec<&str>>().concat();
        return content;
    }

    String::from("Unknown")
}

fn extract_citations_table(html: &Html) -> (String, String, String, String, String, String) {
    let mut results = vec![String::from("Unknown"); 6];

    let parent_selector = Selector::parse("tbody:has(> tr > .gsc_rsb_std)").ok();
    let row_selector = Selector::parse("tr").ok();
    let cell_selector = Selector::parse("td.gsc_rsb_std").ok();
    let header_selector = Selector::parse("td.gsc_rsb_sc1").ok();

    if let (Some(parent_sel), Some(row_sel), Some(cell_sel), Some(header_sel)) = 
        (parent_selector, row_selector, cell_selector, header_selector) {
        if let Some(parent) = html.select(&parent_sel).next() {
            for row in parent.select(&row_sel) {
                let cells: Vec<ElementRef<'_>> = row.select(&cell_sel).collect();
                if cells.len() == 2 {
                    if let Some(header_cell) = row.select(&header_sel).next() {
                        match header_cell.text().collect::<Vec<&str>>().concat().as_str() {
                            "Citations" => {
                                results[0] = cells[0].text().collect::<Vec<&str>>().concat();
                                results[1] = cells[1].text().collect::<Vec<&str>>().concat();
                            }
                            "h-index" => {
                                results[2] = cells[0].text().collect::<Vec<&str>>().concat();
                                results[3] = cells[1].text().collect::<Vec<&str>>().concat();
                            }
                            "i10-index" => {
                                results[4] = cells[0].text().collect::<Vec<&str>>().concat();
                                results[5] = cells[1].text().collect::<Vec<&str>>().concat();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    (
        results[0].clone(),
        results[1].clone(),
        results[2].clone(),
        results[3].clone(),
        results[4].clone(),
        results[5].clone(),
    )
}
