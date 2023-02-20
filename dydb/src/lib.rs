use aws_sdk_dynamodb::{Client, Error, model::{ PutRequest, WriteRequest, AttributeValue }};
use std::collections::HashMap;

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