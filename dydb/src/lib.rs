use aws_sdk_dynamodb::Client;

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
}