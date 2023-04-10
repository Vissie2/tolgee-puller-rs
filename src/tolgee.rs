use reqwest::Url;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub type Resources = HashMap<Language, Namespace>;
type Language = String;
type Namespace = HashMap<String, Value>;

/// Parses the URL with its needed query params.
fn parse_url_with_query_params(api_url: String, lng: Vec<String>, ns: Vec<String>) -> Url {
    let mut params = vec![("languages", lng.join(","))];

    for namespace in ns.iter() {
        params.push(("filterNamespace", namespace.to_string()))
    }

    let base_url = &format!("{api_url}/v2/projects/export");

    reqwest::Url::parse_with_params(base_url, params).unwrap()
}

/// Fetches the zip archive from the Tolgee API.
async fn fetch_zip(url: Url, api_key: String) -> Result<ZipArchive<Cursor<Vec<u8>>>, String> {
    let response_result = reqwest::Client::new()
        .get(url)
        .header("X-API-Key", api_key)
        .send()
        .await;

    let bytes_result = match response_result {
        Ok(response) => {
            if response.status() == 200 {
                response.bytes().await
            } else {
                return Err("Couldn't fetch from API, verify your credentials.".to_string());
            }
        }
        Err(_) => return Err("Couldn't fetch from API.".to_string()),
    };

    let bytes = match bytes_result {
        Ok(bytes) => bytes.to_vec(),
        Err(_) => return Err("Couldn't convert response.".to_string()),
    };

    let cursor = Cursor::new(bytes);
    let zip_result = ZipArchive::new(cursor);

    let zip = match zip_result {
        Ok(zip) => zip,
        Err(_) => return Err("Couldn't read archive, maybe it's corrupt?".to_string()),
    };

    Ok(zip)
}

/// Extracts the resources from the zip archive. Honest story, I spend ~6 hours trying to achieve
/// this and failed then I asked chat-gpt which made (a big part of) this within 10 seconds.
fn extract_resources_from_zip(mut archive: ZipArchive<Cursor<Vec<u8>>>) -> Resources {
    let mut resources = Resources::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        if file.name().ends_with(".json") {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let path_parts = &file.name().split('/').collect::<Vec<&str>>()[..];

            if let [ns, filename] = path_parts.to_owned()[..] {
                let file_name_parts = &filename.split('.').collect::<Vec<&str>>()[..];
                let language = file_name_parts[0];
                let value: Value = serde_json::from_str(&contents).unwrap();

                resources.insert(
                    Language::from(language),
                    Namespace::from([(ns.to_owned(), value)]),
                );
            }
        }
    }

    resources
}

/// Fetches the zip archive and extracts its resources.
pub async fn fetch_resources(
    api_url: String,
    api_key: String,
    lng: Vec<String>,
    ns: Vec<String>,
) -> Result<Resources, String> {
    let url = parse_url_with_query_params(api_url, lng, ns);
    let archive = fetch_zip(url, api_key).await?;

    Ok(extract_resources_from_zip(archive))
}
