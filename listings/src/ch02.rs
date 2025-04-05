use anyhow::{bail, Result};
use fancy_regex::Regex;
use reqwest::blocking::get;
use std::collections::BTreeMap;
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

pub fn create_sample_vocab() -> Result<BTreeMap<String, i32>> {
    let mut vocab: BTreeMap<String, i32> = BTreeMap::new();
    let content = load_sample_text()?;
    let pattern = Regex::new(r#"([,.:;?_!"()']|--|\s)"#)?;

    let mut tokens: Vec<String> = pattern
        .split(&content)
        .filter_map(|s| {
            let trimmed = s.ok()?.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        })
        .chain(pattern.find_iter(&content).filter_map(|m| {
            let delim = m.ok()?.as_str().trim();
            (!delim.is_empty()).then(|| delim.to_string())
        }))
        .collect();

    tokens.sort();

    for (idx, token) in tokens.into_iter().enumerate() {
        vocab.entry(token).or_insert(idx as i32);
    }

    Ok(vocab)
}
