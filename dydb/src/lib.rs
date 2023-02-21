use aws_sdk_dynamodb::{Client, Error, model::{ PutRequest, WriteRequest, AttributeValue }};
use std::collections::HashMap;
use lambda_http::{ Response, Body, Error as LambdaError };
use async_trait::async_trait;

pub struct DyDbClient {
    pub cli: Client
}

impl DyDbClient {
    pub async fn new() -> Self {

        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);

        DyDbClient {
            cli: client
        }
    }

    pub async fn write_items(&self, table: &str, map: HashMap<String, AttributeValue>) -> Result<(), Error> {

        self
        .cli
        .batch_write_item()
        .request_items(
            table,
            vec![
                WriteRequest::builder()
                .put_request(
                    PutRequest::builder()
                    .set_item(Some(HashMap::from(map)))
                    .build()
                )
                .build()
            ]
        )
        .send()
        .await?;

        Ok(())
    }
}

#[async_trait]
pub trait DyDbAction: Send + Sync + 'static {
    async fn add_item(s: &str, c: &DyDbClient) -> Result<Response<Body>, LambdaError> where Self: Sized;
}