use lambda_http::{Body, Error, Request, Response, http::Method};
use dydb::{DyDbClient, DyDbAction};
use define_company::Company;

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
        CustomEvent {http_path: "/new_company", http_method: Method::POST} => Ok(Company::add_item(s, db_client).await?),
        // CustomEvent {http_path: "/new_company_code", http_method: Method::POST} => Ok(CompanyCode::add_item(s, db_client).await?),
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