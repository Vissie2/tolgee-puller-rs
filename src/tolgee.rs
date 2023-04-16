use bytes::Bytes;
use reqwest::Url;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use thiserror::Error;
use url::ParseError;
use zip::{result::ZipError, ZipArchive};

pub type Resources = HashMap<Language, Namespace>;
type Language = String;
type Namespace = HashMap<String, Value>;

#[derive(Debug, Error)]
pub enum TolgeeError {
    #[error("Failed parsing query params.")]
    QueryParse(#[from] ParseError),
    #[error("Failed fetching from Tolgee API.")]
    Failed(#[from] reqwest::Error),
    #[error("Access denied, verify your credentials.")]
    Forbidden,
    #[error("Failed reading archive.")]
    ReadArchive(#[from] ZipError),
    #[error("Failed reading file.")]
    ReadFile(#[from] std::io::Error),
    #[error("Failed reading JSON.")]
    ReadJSON(#[from] serde_json::Error),
    #[error("Something went wrong.")]
    Unknown,
}

type Archive = ZipArchive<Cursor<Bytes>>;

/// Parses the URL with its needed query params.
fn parse_url_with_query_params(
    api_url: String,
    lng: Vec<String>,
    ns: Vec<String>,
) -> Result<Url, TolgeeError> {
    let mut params = vec![("languages", lng.join(","))];

    for namespace in ns.iter() {
        params.push(("filterNamespace", namespace.to_string()))
    }

    let base_url = &format!("{api_url}/v2/projects/export");
    let url = reqwest::Url::parse_with_params(base_url, params)?;

    Ok(url)
}

/// Fetches the zip archive from the Tolgee API.
async fn fetch_zip(url: Url, api_key: String) -> Result<Archive, TolgeeError> {
    let response = reqwest::Client::new()
        .get(url)
        .header("X-API-Key", api_key)
        .send()
        .await?;

    if response.status() == 403 {
        return Err(TolgeeError::Forbidden);
    } else if response.status() != 200 {
        return Err(TolgeeError::Unknown);
    }

    let bytes = response.bytes().await?;
    let cursor = Cursor::new(bytes);
    let zip = ZipArchive::new(cursor)?;

    Ok(zip)
}

/// Splits a string into a vector of strings.
fn split_str(str: &str, char: &str) -> Vec<String> {
    str.split(char).map(String::from).collect::<Vec<String>>()
}

/// Extracts the resources from the zip archive. Honest story, I spend ~6 hours trying to achieve this.
fn extract_resources_from_zip(mut archive: Archive) -> Result<Resources, TolgeeError> {
    let mut resources = Resources::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        if file.name().ends_with(".json") {
            // Read the file into a string.
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            if let [ns, filename] = &split_str(file.name(), "/")[..] {
                let language = &split_str(filename, ".")[0];

                // Covert the string into a JSON value and write to resources.
                let value: Value = serde_json::from_str(&contents)?;
                resources.insert(
                    Language::from(language),
                    Namespace::from([(ns.to_owned(), value)]),
                );
            }
        }
    }

    Ok(resources)
}

/// Fetches the zip archive and extracts its resources.
pub async fn fetch_resources(
    api_url: String,
    api_key: String,
    lng: Vec<String>,
    ns: Vec<String>,
) -> Result<Resources, TolgeeError> {
    let url = parse_url_with_query_params(api_url, lng, ns)?;
    let archive = fetch_zip(url, api_key).await?;
    let resources = extract_resources_from_zip(archive)?;

    Ok(resources)
}
