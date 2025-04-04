use anyhow::{bail, Result};
use reqwest::blocking::get;
use std::fs;
use std::io::Write;

static DOWNLOAD_URL: &str = "https://raw.githubusercontent.com/rasbt/LLMs-from-scratch/main/ch02/01_main-chapter-code/the-verdict.txt";

pub fn load_sample_text() -> Result<String> {
    let filename = "the-verdict.txt";

    let tmp_dir = std::env::temp_dir();
    let file_path = tmp_dir.join(filename);

    if !file_path.exists() {
        let response = get(DOWNLOAD_URL)?;
        if !response.status().is_success() {
            bail!("Failed to download file");
        }
        let mut file = fs::File::create(&file_path)?;
        file.write_all(response.text()?.as_bytes())?;
    }
    let content = fs::read_to_string(&file_path)?;

    Ok(content)
}
