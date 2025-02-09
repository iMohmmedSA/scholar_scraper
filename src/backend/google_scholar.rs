use std::collections::HashMap;
use std::sync::Arc;

use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;
use tokio::sync::Semaphore;
use tokio::task::{self, JoinHandle};
use tracing::{info, error};

use crate::backend::csv::export::export_data;
use crate::prelude::*;

use crate::backend::scholar_model::Scholar;

use super::app::App;
use super::scholar_model::Publication;

pub async fn process(app: &mut App) {
    info!("Start processing list");
    let semaphore = Arc::new(Semaphore::new(app.thread_count));

    let tasks: Vec<JoinHandle<()>> = app
        .pre_scholars
        .clone()
        .into_iter()
        .map(|task| {
            let semaphore = semaphore.clone();
            let scholar_list = app.scholars.clone();
            task::spawn(async move {
                if let Ok(_permit) = semaphore.acquire().await {
                    let scholar = extract(task.google_id).await.ok();
                    
                    if let Some(scholar) = scholar {
                        App::add_scholar(scholar_list.clone(), scholar).await;
                    }
                } else {
                    eprintln!("Failed to acquire semaphore permit.");
                }
            })
        })
        .collect();

    for task in tasks {
        let _ = task.await;
    }

    let scholars: Vec<Scholar> = app.scholars.lock().await.iter().cloned().collect();
    export_data(scholars).await;

    println!("Scholar List: {:?}", app.scholars.lock().await.len())
}

async fn extract(id: String) -> Result<Scholar> {
    println!("Processing: {id}");

    let result = match fetch(&id).await {
        Ok(o) => o,
        Err(err) => {return Err(err);},
    };

    let publications = match process_publication(&id).await {
        Ok(o) => o,
        Err(err) => {return Err(err);},
    };
    let document_count = publications.len();

    let html = Html::parse_document(&result);

    let name = extract_content(&html, "div#gsc_prf_in");
    let affiliation = extract_content(&html, "div.gsc_prf_il > a.gsc_prf_ila");
    let (cited_by, cited_5_years, h_index_all, h_index_5, i10_index_all, i10_index_5) =
        extract_citations_table(&html);

    let scholar: Scholar = Scholar::new(
        id.clone(),
        name,
        affiliation,
        document_count,
        cited_by,
        cited_5_years,
        h_index_all,
        h_index_5,
        i10_index_all,
        i10_index_5,
        publications
    );

    let info = format!("Extracting scholar data with id:{:?} successed", id);
    info!(info);
    println!("{}", info);
    Ok(scholar)
}

fn extract_content(html: &Html, selecter: &str) -> String {
    let selector = match Selector::parse(selecter) {
        Ok(s) => s,
        Err(_) => return String::from("Unknown"),
    };

    if let Some(element) = html.select(&selector).next() {
        let content = element.text().collect::<Vec<&str>>().concat();
        return content.trim().to_string();
    }

    String::from("Unknown")
}

fn extract_element(row: &scraper::ElementRef, selector: &str) -> String {
    let selector: Selector = match Selector::parse(selector) {
        Ok(s) => s,
        Err(_) => return String::from("Unknown"),
    };

    if let Some(element) = row.select(&selector).next() {
        let content = element.text().collect::<Vec<&str>>().concat();
        return content.trim().to_string();
    }

    String::from("Unknown")
}

fn extract_citations_table(html: &Html) -> (String, String, String, String, String, String) {
    let mut results = vec![String::from("Unknown"); 6];

    let parent_selector = Selector::parse("tbody:has(> tr > .gsc_rsb_std)").ok();
    let row_selector = Selector::parse("tr").ok();
    let cell_selector = Selector::parse("td.gsc_rsb_std").ok();
    let header_selector = Selector::parse("td.gsc_rsb_sc1").ok();

    if let (Some(parent_sel), Some(row_sel), Some(cell_sel), Some(header_sel)) = (
        parent_selector,
        row_selector,
        cell_selector,
        header_selector,
    ) {
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

async fn process_publication(id: &String) -> Result<Vec<Publication>> {
    let mut start_at: usize = 0;
    let mut publication: Vec<Publication> = vec![];

    let row_selector = Selector::parse("tr").expect("I made mistake at process_publication");

    loop {

        let mut result = match fetch_publication(id, start_at).await {
            Ok(o) => o,
            Err(_) => {
                println!("fetch_publication failed for {id}");
                return Err(Error::FailedProcess);
            },
        };

        result = format!("<table>{result}</table>");

        let html = Html::parse_document(&result);

        let mut temp_pub: Vec<Publication> = vec![];
        
        for row in html.select(&row_selector) {
            let title = extract_element(&row, "td.gsc_a_t a.gsc_a_at");

            let journal = extract_element(&row, "td.gsc_a_t div.gs_gray:nth-of-type(2)");

            let cited_by = extract_element(&row, "td.gsc_a_c a.gsc_a_ac");

            let year = extract_element(&row, "td.gsc_a_y span.gsc_a_h");

            temp_pub.push(Publication::new(title, journal, year, cited_by));
        }

        let size = temp_pub.len();
        publication.extend(temp_pub);
        if size < 100 { break; }


        start_at += 100;

    }

    Ok(publication)
}

async fn fetch(id: &String) -> Result<String> {
    let url = format!("https://scholar.google.com/citations?user={id}&hl=en");
    let client = Client::new();
    let response = match client.get(&url).send().await {
        Ok(ok) => ok,
        Err(_) => {
            error!("{}", Error::FailedFetch(url.clone()));
            return Err(Error::FailedFetch(url.clone()));
        },
    };

    if !response.status().is_success() {
        error!("{}", Error::HttpError(response.status()));
        return Err(Error::HttpError(response.status()));
    }

    match response.text().await {
        Ok(ok) => Ok(ok),
        Err(_) => {
            error!("{}", Error::ConvertText(id.clone()));
            Err(Error::ConvertText(id.clone()))
        }
    }
}

async fn fetch_publication(id: &String, start_at: usize) -> Result<String> {
    let mut form = HashMap::new();
    form.insert("json", "1");

    let url = format!(
        "https://scholar.google.com/citations?user={id}&hl=en&cstart={start_at}&pagesize=100"
    );
    let client = Client::new();
    let request = client.post(&url).form(&form);
    let response = match request.send().await {
        Ok(ok) => ok,
        Err(_) => {
            error!("{}", Error::FailedFetch(url.clone()));
            return Err(Error::FailedFetch(url.clone()));
        }
    };

    if !response.status().is_success() {
        error!("{}", Error::HttpError(response.status()));
        return Err(Error::HttpError(response.status()));
    }
    
    let response = match response.text().await {
        Ok(ok) => ok,
        Err(_) => {
            error!("{}", Error::ConvertText(id.clone()));
            return Err(Error::ConvertText(id.clone()));
        }
    };

    let parsed: Value = match serde_json::from_str(&response) {
        Ok(ok) => ok,
        Err(_) => {
            error!("{}", Error::ConvertJson);
            return Err(Error::ConvertJson);
        }
    };

    let b = parsed
        .get("B")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            error!("{}", Error::ReadJson);
            Error::ReadJson
        })?;

    Ok(b.to_string())
}
