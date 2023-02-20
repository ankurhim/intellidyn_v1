use aws_sdk_dynamodb::{model::AttributeValue, Error};
use serde::{ Serialize, Deserialize };
use dydb::DyDbClient;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Company {
    pub p_type: String,
    pub age: String,
    pub username: String,
    pub first: String,
    pub last: String,
}

impl Company {

    pub fn new(
        p_type: String,
        age: String,
        username: String,
        first: String,
        last: String,
    ) -> Self {
        Company {
            p_type,
            age,
            username,
            first,
            last,
        }
    }
    
    pub async fn add_company(&self, client: &DyDbClient) -> Result<(), Error> {
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

        let _resp = client.write_items("company", hashmap);
    
        Ok(())
    }
}