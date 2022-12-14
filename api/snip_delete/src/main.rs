use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use dotenv::dotenv;
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use postgrest::Postgrest;
use serde_json::Value;
use std::env;

use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    dotenv().ok();

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn handler(
    event: ApiGatewayProxyRequest,
    _ctx: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let mut response_headers: HeaderMap = HeaderMap::new();
    response_headers.insert("Content-Type", "application/json".parse().unwrap());

    if event.http_method.as_str() != "DELETE" {
        let resp = ApiGatewayProxyResponse {
            status_code: 400,
            headers: response_headers,
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(
                r#"{ "statusCode": 400, "message": "Wrong HTTP method!" }"#.to_owned(),
            )),
            is_base64_encoded: Some(false),
        };
        return Ok(resp);
    }

    let snip_id: &str;

    let mut request_user_id: String = "".to_owned();
    let mut snip_user_id: String = "".to_owned();

    let request_body: Value;

    match event.body {
        Some(value) => request_body = serde_json::from_str(&value)?,
        None => {
            let resp = ApiGatewayProxyResponse {
                status_code: 400,
                headers: response_headers,
                multi_value_headers: HeaderMap::new(),
                body: Some(Body::Text(
                    r#"{ "statusCode": 400, "message": "No body provided in request." }"#
                        .to_owned(),
                )),
                is_base64_encoded: Some(false),
            };
            return Ok(resp);
        }
    }

    match request_body["id"].as_str() {
        Some(value) => snip_id = value,
        None => {
            let resp = ApiGatewayProxyResponse {
                status_code: 400,
                headers: response_headers,
                multi_value_headers: HeaderMap::new(),
                body: Some(Body::Text(
                    r#"{ "statusCode": 400, "message": "Missing required body key [id]!" }"#
                        .to_owned(),
                )),
                is_base64_encoded: Some(false),
            };
            return Ok(resp);
        }
    }

    let client = Postgrest::new("https://araasnleificjyjflqml.supabase.co/rest/v1/")
        .insert_header("apikey", &env::var("SUPABASE_PUBLIC_ANON_KEY").unwrap());

    let resp = client
        .from("snips")
        .select("*")
        .eq("id", snip_id)
        .execute()
        .await?;

    let body = resp.text().await?;
    let reponse_body_json: Value = serde_json::from_str(&body)?;

    if event.headers.contains_key("Authorization") {
        match event.headers["Authorization"].to_str() {
            Ok(value) => request_user_id = value.to_owned(),
            Err(error) => {
                println!("{}", error)
            }
        }
    } else {
        let resp = ApiGatewayProxyResponse {
            status_code: 400,
            headers: response_headers,
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(
                r#"{ "statusCode": 400, "message": "Missing required header [Authorization]!" }"#
                    .to_owned(),
            )),
            is_base64_encoded: Some(false),
        };
        return Ok(resp);
    }

    if !request_user_id.contains("Bearer") {
        let resp = ApiGatewayProxyResponse {
            status_code: 401,
            headers: response_headers,
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(
                r#"{ "statusCode": 401, "message": "Wrong authorization scheme" }"#.to_owned(),
            )),
            is_base64_encoded: Some(false),
        };
        return Ok(resp);
    } else {
        request_user_id = request_user_id.chars().skip("Bearer ".len()).collect();
    }

    match reponse_body_json[0]["user_id"].as_str() {
        Some(value) => {
            snip_user_id = value.to_owned();
        }
        None => {}
    }

    if snip_user_id.trim().is_empty() {
        let resp = ApiGatewayProxyResponse {
            status_code: 403,
            headers: response_headers,
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(
                r#"{ "statusCode": 403, "message": "Paste is not deleteable!" }"#.to_owned(),
            )),
            is_base64_encoded: Some(false),
        };

        return Ok(resp);
    }

    if snip_user_id == request_user_id && !snip_user_id.is_empty() {
        client
            .from("snips")
            .select("*")
            .eq("id", snip_id)
            .delete()
            .execute()
            .await?;
    } else {
        let resp = ApiGatewayProxyResponse {
            status_code: 401,
            headers: response_headers,
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(
                r#"{ "statusCode": 401, "message": "Authorization token invalid!" }"#.to_owned(),
            )),
            is_base64_encoded: Some(false),
        };

        return Ok(resp);
    }

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: response_headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(
            r#"{ "statusCode": 200, "message": "Snip deleted successfully!" }"#.to_owned(),
        )),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}
