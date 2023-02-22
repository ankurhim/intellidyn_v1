use aws_sdk_dynamodb::{model::AttributeValue, Error};
use serde::{ Serialize, Deserialize };
use lambda_http::{ Response, Body, Error as LambdaError };
use serde_json;
use dydb::{DyDbClient, DyDbAction};
use std::collections::HashMap;
use async_trait::async_trait;
use uuid::Uuid;
use bcrypt::{ hash, verify, DEFAULT_COST};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub user_pk: String,
    pub username: String,
    pub password: String
}

impl User {

    pub fn init(
        username: String,
        password: String
    ) -> Self {
        User {
            user_pk: Uuid::new_v4().to_string(),
            username,
            password: hash(password, DEFAULT_COST).expect("Hashing Failed").to_string()
        }
    }
    
    pub async fn new(&self, client: &DyDbClient) -> Result<(), Error> {
        let user_pk_av = AttributeValue::S(self.clone().user_pk);
        let username_av = AttributeValue::S(self.clone().username);
        let password_av = AttributeValue::S(self.clone().password);

        let map = vec![
            ("user_pk".to_string(), user_pk_av),
            ("username".to_string(), username_av),
            ("password".to_string(), password_av)
        ];

        let hashmap: HashMap::<_, _> = map.into_iter().collect();

        let _resp = client.write_items("user", hashmap);
    
        Ok(())
    }

    pub async fn fetch(&self, client: &DyDbClient) -> Result<(), Error> {

        let map = vec![
            ("username".to_string(), AttributeValue::S(self.clone().username)),
            ("password".to_string(), AttributeValue::S(self.clone().password))
        ];

        let hashmap: HashMap<_, _> = map.into_iter().collect();

        let _resp = client.get_items("user", hashmap);

        Ok(())
    }
}

#[async_trait]
impl DyDbAction for User {
    async fn add_item(s: &str, db_client: &DyDbClient) -> Result<Response<Body>, LambdaError> where Self: Sized {
        let item: User = User::read_s(s).await?;

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
        let item: User = User::read_s(s).await?;

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

impl<T> SubTrait<T> for User {}