use aws_sdk_dynamodb::{model::AttributeValue, Error};
use serde::{ Serialize, Deserialize };
use lambda_http::{ Response, Body, Error as LambdaError };
use serde_json;
use dydb::{DyDbClient, DyDbAction};
use std::collections::HashMap;
use async_trait::async_trait;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub p_type: String,
    pub age: String,
    pub username: String,
    pub first: String,
    pub last: String,
}

impl User {

    pub fn new(
        p_type: String,
        age: String,
        username: String,
        first: String,
        last: String,
    ) -> Self {
        User {
            p_type,
            age,
            username,
            first,
            last,
        }
    }
    
    pub async fn add_user(&self, client: &DyDbClient) -> Result<(), Error> {
        let user_av = AttributeValue::S(self.clone().username);
        let type_av = AttributeValue::S(self.clone().p_type);
        let age_av = AttributeValue::S(self.clone().age);
        let first_av = AttributeValue::S(self.clone().first);
        let last_av = AttributeValue::S(self.clone().last);

        let map = vec![
            ("username".to_string(), user_av),
            ("account_type".to_string(), type_av),
            ("age".to_string(), age_av),
            ("first_name".to_string(), first_av),
            ("last_name".to_string(), last_av)
        ];

        let hashmap: HashMap<_, _> = map.into_iter().collect();

        let _resp = client.write_items("user", hashmap);
    
        Ok(())
    }
}

#[async_trait]
impl DyDbAction for User {
    async fn add_item(s: &str, db_client: &DyDbClient) -> Result<Response<Body>, LambdaError> where Self: Sized {
        let item = match serde_json::from_str::<User>(s) {
            Ok(item) => item,
            Err(err) => {
                let resp = Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(err.to_string().into())
                .map_err(Box::new)?;
                return Ok(resp);
            }
        };

        let result = item.add_user(db_client).await?;
    
        let j = serde_json::to_string(&result.clone())?;
    
        let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(j.into())
        .map_err(Box::new)?;
        Ok(resp)
    }
}