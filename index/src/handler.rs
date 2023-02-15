use lambda_http::{Body, Error, Request, Response, http::Method};
use dydb::DyDbClient;
use define_company::Item;

pub async fn handle_request(db_client: &DyDbClient, event: Request) -> Result<Response<Body>, Error> {
    let body = &event.body();
    let method = &event.method();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");

    match **method {
        Method::POST => {
            let item = match serde_json::from_str::<Item>(s) {
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
        
            item.add_item(db_client).await?;
        
            let j = serde_json::to_string(&item.clone())?;
        
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
            .body(Body::Text("Invalid Method".to_string()))
            .map_err(Box::new)?;
            Ok(resp)
        }
    }
    
}