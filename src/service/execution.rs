use std::str::FromStr;

use anyhow::Result;
use axum::http::{HeaderName, HeaderValue};
use chrono::Local;
use hyper::Method;
use reqwest::Url;
use serde_json::Value;
use sqlx::types::uuid::fmt::Hyphenated;
use sqlx_crud::Crud;
use uuid::Uuid;

use crate::{
    api::{error::Error, resp::ExpectRowsAffected},
    db,
    entity::{
        execution::{Execution, RawHttpRequest, RawHttpResponse},
        request::Request,
    },
};

async fn make_response(resp: reqwest::Response) -> Result<RawHttpResponse> {
    let is_json = resp
        .headers()
        .get("Content-Type")
        .unwrap()
        .to_str()
        .unwrap()
        .contains("application/json");
    let headers = resp
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                Value::String(v.to_str().unwrap().to_string()),
            )
        })
        .collect::<Value>();
    let version = format!("{:?}", resp.version());
    let status_code = resp.status().as_u16();
    let status_message = resp.status().canonical_reason().unwrap().to_string();
    Ok(RawHttpResponse {
        id: 0,
        version,
        status_code,
        status_message,
        headers,
        body: if is_json {
            let json = resp.json().await?;
            json
        } else {
            let text = resp.text().await?;
            Value::String(text)
        },
    })
}

async fn make_request(request_id: Hyphenated) -> Result<RawHttpRequest> {
    let request = Request::by_id(db::db_pool(), request_id)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let mut request = prepare_request(&request)?;
    let request_id = request
        .clone()
        .create(db::db_pool())
        .await
        .map_err(|e| Error::CreateFailed(e.to_string()))?
        .last_insert_id();
    request.id = request_id;
    Ok(request)
}

async fn make_request_builder(request: &RawHttpRequest) -> Result<reqwest::RequestBuilder> {
    let builder = reqwest::Client::new()
        .request(
            Method::from_str(&request.method).unwrap(),
            Url::parse(&request.url).unwrap(),
        )
        .headers(
            request
                .headers
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| {
                    (
                        HeaderName::from_str(&k.to_string()).unwrap(),
                        HeaderValue::from_str(&v.to_string()).unwrap(),
                    )
                })
                .collect(),
        );
    let builder = if request.body.is_none() {
        builder
    } else {
        builder.json(&request.body.clone().unwrap())
    };
    Ok(builder)
}

pub(crate) async fn execute_request(request_id: Hyphenated) -> Result<Execution> {
    let request = make_request(request_id).await?;
    let builder = make_request_builder(&request).await?;
    let request_time = Local::now();
    tracing::info!("send request at {}", request_time);
    let resp = builder.send().await?;
    let response_time = Local::now();
    tracing::info!("get response at {}", response_time);
    let /* mut */ response = make_response(resp).await?;
    let resp_id = response
        .clone()
        .create(db::db_pool())
        .await
        .map_err(|e| Error::CreateFailed(e.to_string()))?
        .last_insert_id();
    // response.id = resp_id;
    let execution = Execution {
        id: Uuid::new_v4().hyphenated(),
        request: request.id,
        request_time,
        response_time,
        response: resp_id,
    };
    execution
        .clone()
        .create(db::db_pool())
        .await?
        .rows_affected()
        .expect(1)?;
    Ok(execution)
    // let request = reqwest::Request {
    //     method: Method::from_str(&request.method).unwrap(),
    //     url: Url::parse(&request.url).unwrap(),
    //     headers: request
    //         .headers
    //         .as_object()
    //         .unwrap()
    //         .iter()
    //         .map(|(k, v)| {
    //             (
    //                 HeaderName::from_str(&k.to_string()).unwrap(),
    //                 HeaderValue::from_str(&v.to_string()).unwrap(),
    //             )
    //         })
    //         .collect(),
    //     body: match request.body {
    //         Some(body) => Body::empty(),
    //         None => None,
    //     },
    //     timeout: 10000,
    //     version: hyper::Version::HTTP_11,
    // };
    // let request = reqwest::Request::new(
    //     Method::from_str(&request.method).unwrap(),
    //     Url::parse(&request.url).unwrap(),
    // );
    // let uri = request.uri().to_string();
    // let headers = request.headers();
    // let send_time = Local::now();
    // let response = Client::new().request(request).await?;
    // let respond_time = Local::now();
    // let is_json = response
    //     .headers()
    //     .get("Content-Type")
    //     .unwrap()
    //     .to_str()
    //     .unwrap()
    //     .contains("application/json");
    // let headers = response
    //     .headers()
    //     .iter()
    //     .map(|(k, v)| {
    //         (
    //             k.to_string(),
    //             Value::String(v.to_str().unwrap().to_string()),
    //         )
    //     })
    //     .collect();
    // let status = response.status().as_u16();
    // let response = if is_json {
    //     let body = hyper::body::to_bytes(response.into_body()).await?;
    //     let resp = serde_json::from_slice(&body).unwrap();
    //     Response {
    //         id: Uuid::new_v4().hyphenated(),
    //         status,
    //         header: Value::Object(headers),
    //         body: Some(resp),
    //         message: None,
    //         created_at: None,
    //         updated_at: None,
    //         deleted_at: None,
    //     }
    // } else {
    //     let msg =
    //         String::from_utf8(hyper::body::to_bytes(response.into_body()).await?.to_vec()).unwrap();
    //     Response {
    //         id: Uuid::new_v4().hyphenated(),
    //         status,
    //         header: Value::Object(headers),
    //         body: None,
    //         message: Some(msg),
    //         created_at: None,
    //         updated_at: None,
    //         deleted_at: None,
    //     }
    // };
    // response
    //     .clone()
    //     .create(db::db_pool())
    //     .await
    //     .map_err(|e| Error::CreateFailed(e.to_string()))?
    //     .rows_affected()
    //     .expect(1)?;
    // Ok(response)
}

fn prepare_url(r: &Request) -> Result<String> {
    let obj = r.query.as_object().unwrap();
    let uri = if obj.is_empty() {
        format!("{}{}", r.host, r.path)
    } else {
        let x = obj
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        format!("{}{}?{}", r.host, r.path, x)
    };
    Ok(uri)
}

fn prepare_request(request: &Request) -> Result<RawHttpRequest> {
    Ok(RawHttpRequest {
        id: 0,
        method: request.method.clone(),
        url: prepare_url(&request)?,
        version: "HTTP/1.1".to_string(),
        headers: request.headers.clone(),
        body: request.body.clone(),
    })
}
