use aws_sdk_dynamodb::{model::AttributeValue, Error};
use serde::{ Serialize, Deserialize };
use lambda_http::{ Response, Body, Error as LambdaError };
use serde_json;
use dydb::{DyDbClient, DyDbAction, SubTrait};
use std::collections::HashMap;
use async_trait::async_trait;
use uuid::Uuid;

use user::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomizationRequest {
    pub customization_request_pk: String,
    pub customization_request: String,
    pub description: String,
    pub created_by: User
}

impl CustomizationRequest {

    pub fn init(
        customization_request: String,
        description: String,
        created_by: User
    ) -> Self {
        CustomizationRequest {
            customization_request_pk: Uuid::new_v4().to_string(),
            customization_request,
            description,
            created_by
        }
    }
    
    pub async fn new(&self, client: &DyDbClient) -> Result<(), Error> {
        let customization_request_pk_av = AttributeValue::S(self.clone().customization_request_pk);
        let customization_request_av = AttributeValue::S(self.clone().customization_request);
        let description_av = AttributeValue::S(self.clone().description);
        let created_by_av = AttributeValue::S(self.clone().created_by.user_pk);

        let map = vec![
            ("customization_request_pk".to_string(), customization_request_pk_av),
            ("customization_request".to_string(), customization_request_av),
            ("description".to_string(), description_av),
            ("created_by".to_string(), created_by_av),
        ];

        let hashmap: HashMap<_, _> = map.into_iter().collect();

        let _resp = client.write_items("customization_request", hashmap);
    
        Ok(())
    }

    pub async fn fetch(&self, client: &DyDbClient) -> Result<(), Error> {

        let map = vec![
            ("customization_request".to_string(), AttributeValue::S(self.clone().customization_request))
        ];

        let hashmap: HashMap<_, _> = map.into_iter().collect();

        let _resp = client.get_items("customization_request", hashmap);

        Ok(())
    }
}

#[async_trait]
impl DyDbAction for CustomizationRequest {
    async fn add_item(s: &str, db_client: &DyDbClient) -> Result<Response<Body>, LambdaError> where Self: Sized {
        let item: CustomizationRequest = CustomizationRequest::read_s(s).await?;

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
        let item: CustomizationRequest = CustomizationRequest::read_s(s).await?;

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

impl<T> SubTrait<T> for CustomizationRequest {}