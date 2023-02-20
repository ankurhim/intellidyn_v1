use lambda_http::{Body, Error, Request, Response, http::Method};
use dydb::DyDbClient;
use define_company::Company;
use serde_json;

pub struct CustomEvent<'a> {
    http_path: &'a str,
    http_method: Method
}

pub async fn handle_request(db_client: &DyDbClient, event: Request) -> Result<Response<Body>, Error> {

    let h_event = CustomEvent {
        http_path: event.uri().path(),
        http_method: event.method().into(),
    };

    let body = event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");
        
    match h_event {
        CustomEvent {
            http_path: "/new_company",
            http_method: Method::POST,
        } => {
            let item = match serde_json::from_str::<Company>(s) {
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
    
            let result = item.add_company(db_client).await?;
        
            let j = serde_json::to_string(&result.clone())?;
        
            let resp = Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(j.into())
            .map_err(Box::new)?;
            Ok(resp)
        },
        _ => {
            let resp = Response::builder()
            .status(500)
            .header("content-type", "application/json")
            .body(Body::Text("Method not allowed".to_string()))
            .map_err(Box::new)?;
            Ok(resp)
        }
    }
}