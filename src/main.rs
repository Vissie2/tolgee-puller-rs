use dotenvy::dotenv;
use tolgee_puller_rs::cli;

mod logger;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let result = cli::initialize().await;

    match result {
        Ok(_) => logger::success("Succesfully pulled resources!"),
        Err(e) => logger::error(&e),
    };
}
