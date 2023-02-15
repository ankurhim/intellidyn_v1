pub mod handler;

use lambda_http::{run, service_fn, Error, Request};
use dydb::DyDbClient;
use crate::handler::handle_request;



#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
    .with_ansi(false)
    .without_time()
    .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
    .init();

    let db_client = DyDbClient::new().await;

    run(service_fn(|event: Request| async {
        handle_request(&db_client, event).await
    }))
    .await
}