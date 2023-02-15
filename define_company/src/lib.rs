use aws_sdk_dynamodb::{model::AttributeValue, Error};
use serde::{ Serialize, Deserialize };
use dydb::DyDbClient;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub p_type: String,
    pub age: String,
    pub username: String,
    pub first: String,
    pub last: String,
}

impl Item {

    pub fn new(
        p_type: String,
        age: String,
        username: String,
        first: String,
        last: String,
    ) -> Self {
        Item {
            p_type,
            age,
            username,
            first,
            last,
        }
    }
    
    pub async fn add_item(&self, client: &DyDbClient) -> Result<(), Error> {
        let user_av = AttributeValue::S(self.clone().username);
        let type_av = AttributeValue::S(self.clone().p_type);
        let age_av = AttributeValue::S(self.clone().age);
        let first_av = AttributeValue::S(self.clone().first);
        let last_av = AttributeValue::S(self.clone().last);
    
        let request = client
        .cli
        .put_item()
        .table_name("item")
        .item("username", user_av)
        .item("account_type", type_av)
        .item("age", age_av)
        .item("first_name", first_av)
        .item("last_name", last_av);
    
        let _resp = request.send().await?;
    
        Ok(())
    }
}