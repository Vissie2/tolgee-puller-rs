use crate::cli::Args;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct EnvVars {
    tolgee_api_key: Option<String>,
    tolgee_api_url: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub api_url: String,
    pub lng: Vec<String>,
    pub ns: Vec<String>,
    pub output_path: String,
    pub split: bool,
}

const DEFAULT_API_URL: &str = "https://app.tolgee.io";

/// Attempts to read the env variables, panics if not possible.
fn get_env() -> EnvVars {
    envy::from_env::<EnvVars>().expect("Something went wrong while trying to read env variables.")
}

/// Gets the config based on command arguments and env variables.
pub fn get_config(args: Args) -> Result<Config, String> {
    let env = get_env();

    // We prioritize env variables over the command args here.
    let api_key_with_fallback = env.tolgee_api_key.or(args.api_key);
    let api_url_with_fallback = env.tolgee_api_url.or(args.api_url);

    let api_key = match api_key_with_fallback {
        Some(api_key) => api_key,
        None => return Err("No API key specified.".to_string()),
    };

    let api_url = match api_url_with_fallback {
        Some(api_url) => api_url,
        None => DEFAULT_API_URL.to_owned(),
    };

    let config = Config {
        api_key,
        api_url,
        lng: args.lng,
        ns: args.ns,
        output_path: args.output_path,
        split: args.split,
    };

    Ok(config)
}
