use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use crate::error::{Result, Error};
use crate::token::AUTH_TOKEN;
pub(crate) mod product;
#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - {payload:#?} api_login", "HANDLER");

    // TODO: Implement real db/auth logic

    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }
    // FIXME: Implement real auth-token generation/signature
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}