use aws_sdk_dynamodb::{model::AttributeValue, Error};
use serde::{ Serialize, Deserialize };
use lambda_http::{ Response, Body, Error as LambdaError };
use serde_json;
use api_dydb::{DyDbClient, DyDbAction, SubTrait};
use std::collections::HashMap;
use async_trait::async_trait;
use uuid::Uuid;

use api_user::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Segment {
    pub segment_pk: String,
    pub segment: String,
    pub description: String,
    pub created_by: User
}

impl Segment {

    pub fn init(
        segment: String,
        description: String,
        created_by: User
    ) -> Self {
        Segment {
            segment_pk: Uuid::new_v4().to_string(),
            segment,
            description,
            created_by
        }
    }
    
    pub async fn new(&self, client: &DyDbClient) -> Result<(), Error> {
        let segment_pk_av = AttributeValue::S(self.clone().segment_pk);
        let segment_av = AttributeValue::S(self.clone().segment);
        let description_av = AttributeValue::S(self.clone().description);
        let created_by_av = AttributeValue::S(self.clone().created_by.user_pk);

        let map = vec![
            ("segment_pk".to_string(), segment_pk_av),
            ("segment".to_string(), segment_av),
            ("description".to_string(), description_av),
            ("created_by".to_string(), created_by_av),
        ];

        let hashmap: HashMap<_, _> = map.into_iter().collect();

        let _resp = client.write_items("segment", hashmap);
    
        Ok(())
    }

    pub async fn fetch(&self, client: &DyDbClient) -> Result<(), Error> {

        let map = vec![
            ("segment".to_string(), AttributeValue::S(self.clone().segment))
        ];

        let hashmap: HashMap<_, _> = map.into_iter().collect();

        let _resp = client.get_items("segment", hashmap);

        Ok(())
    }
}

#[async_trait]
impl DyDbAction for Segment {
    async fn add_item(s: &str, db_client: &DyDbClient) -> Result<Response<Body>, LambdaError> where Self: Sized {
        let item: Segment = Segment::read_s(s).await?;

        let result = item.new(db_client).await?;
    
        let j = serde_json::to_string(&result.clone())?;
    
        let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(j.into())
        .map_err(Box::new)?;
        Ok(resp)
    }

    async fn get_item(s: &str, client: &DyDbClient) -> Result<Response<Body>, LambdaError> where Self: Sized {
        let item: Segment = Segment::read_s(s).await?;

        let result = item.fetch(client).await?;

        let j = serde_json::to_string(&result.clone())?;
    
        let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(j.into())
        .map_err(Box::new)?;
        Ok(resp)
    }
}

impl<T> SubTrait<T> for Segment {}