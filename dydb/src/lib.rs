use aws_sdk_dynamodb::{Client, Error, model::{ PutRequest, WriteRequest, AttributeValue, KeysAndAttributes }};
use std::collections::HashMap;
use lambda_http::{ Response, Body, Error as LambdaError };
use async_trait::async_trait;
use serde_json;
use serde::{ Serialize, Deserialize };

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

    pub async fn get_items(&self, table: &str, map: HashMap<String, AttributeValue>) -> Result<(), Error> {
        self
        .cli
        .batch_get_item()
        .request_items(
            table,
            KeysAndAttributes::builder()
                .keys(map)
                .build(),
        )
        .send()
        .await?;

        Ok(())
    }
}

#[async_trait]
pub trait DyDbAction: Send + Sync + 'static {
    async fn add_item(s: &str, c: &DyDbClient) -> Result<Response<Body>, LambdaError> where Self: Sized;
    async fn get_item(s: &str, c: &DyDbClient) -> Result<Response<Body>, LambdaError> where Self: Sized;
}

#[async_trait]
pub trait SubTrait<T> {
    async fn read_s(s: &str) -> Result<T, serde_json::Error>
    where
    Self: Sized,
    T: Clone + std::fmt::Debug + for <'de>Deserialize<'de> + Serialize
    {
        serde_json::from_str::<T>(s)
    }
}