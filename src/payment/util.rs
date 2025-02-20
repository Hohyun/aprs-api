use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::File;
use crate::payment::common::Payment;

pub fn save_csv_file(settleco: &str, sales_date: &str, payments: Vec<Payment>) -> anyhow::Result<()> {
    let filename = format!("./data/_{}_SP_{}.csv", settleco, sales_date);
    let path = Path::new(filename.as_str());
    let mut wtr = csv::Writer::from_path(path)?;
    for p in payments {
        wtr.serialize(p)?;
    }
    wtr.flush()?;
    Ok(())
}

pub async fn get_data_from_url(url: &str) -> anyhow::Result<()> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    println!("content: {}", content);
    Ok(())
}

pub async fn download_file_from_url(url:& str) -> anyhow::Result<()> {
    let target_dir = PathBuf::new();
    let target_dir = target_dir.join("./data");
    let response = reqwest::get(url).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: {}", fname);
        let fname = target_dir.join(fname);
        println!("will be located under: {:?}", fname);
        File::create(fname)?
    };
    let content =  response.bytes().await?;
    dest.write_all(&content)?;
    Ok(())
}