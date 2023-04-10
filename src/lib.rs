mod config;
mod output;
mod tolgee;

pub mod cli {
    use crate::{config, output, tolgee};
    use clap::{Parser, ValueHint};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        #[arg(long)]
        pub api_key: Option<String>,

        #[arg(long, help("Defaults to the Tolgee API"))]
        pub api_url: Option<String>,

        #[arg(long, required(true), value_delimiter = ',', help("Comma separated"))]
        pub lng: Vec<String>,

        #[arg(long, required(true), value_delimiter = ',', help("Comma separated"))]
        pub ns: Vec<String>,

        #[arg(long, required(true), value_hint=ValueHint::FilePath)]
        pub output_path: String,

        #[arg(long)]
        pub split: bool,
    }

    pub async fn initialize() -> Result<(), String> {
        let args = Args::parse();

        let config = match config::get_config(args) {
            Ok(config) => config,
            Err(error) => return Err(error),
        };

        let config::Config {
            api_key,
            api_url,
            lng,
            ns,
            split,
            output_path,
        } = config;

        // Fetches the resources from the Tolgee API. Could panic if something goes wrong.
        let resources_result = tolgee::fetch_resources(api_url, api_key, lng, ns).await;

        let resources = match resources_result {
            Ok(resources) => resources,
            Err(e) => return Err(e),
        };

        // Converts the resources into TypeScript and writes to the file of the `output_path`.
        let write_result = output::write_resources_to_file(resources, output_path, split);

        if write_result.is_err() {
            return Err(write_result.unwrap_err());
        }

        Ok(())
    }
}
