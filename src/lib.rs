mod config;
mod output;
mod tolgee;

pub mod cli {
    use crate::{
        config, output,
        tolgee::{self, Resources},
    };
    use clap::{Parser, ValueHint};
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum CliError {
        #[error("{0}")]
        Config(#[from] config::ConfigError),
        #[error("{0}")]
        Tolgee(#[from] tolgee::TolgeeError),
        #[error("{0}")]
        Output(#[from] output::OutputError),
    }

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

    pub async fn initialize() -> Result<(), CliError> {
        let args = Args::parse();
        let config = config::get_config(args)?;

        let config::Config {
            api_key,
            api_url,
            lng,
            ns,
            split,
            output_path,
        } = config;

        // Fetches the resources from the Tolgee API. Could panic if something goes wrong.
        let resources: Resources = tolgee::fetch_resources(api_url, api_key, lng, ns).await?;

        // Converts the resources into TypeScript and writes to the file of the `output_path`.
        output::write_resources_to_file(resources, output_path, split)?;

        Ok(())
    }
}
