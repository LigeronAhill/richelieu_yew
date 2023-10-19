use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use crate::error::{Error, Result};
use crate::token::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>
) -> impl IntoResponse {
    let auth_token = cookies.get(AUTH_TOKEN)
        .map(|c| c.value().to_string());
    // TODO: Real auth-token parsing & validation

 //   let (user_id, exp, sign) = auth_token;



    match auth_token {
        Some(token) => {
            match parse_token(token) {
                Ok((user_id, exp, sign)) => {
                    // req.extensions_mut().insert(user_id);
                    // req.extensions_mut().insert(exp);
                    // req.extensions_mut().insert(sign);
                    next.run(req).await
                }
                Err(_) => {
                    Error::AuthFailTokenWrongFormat.into_response()
                }
            }
        }
        None => {
            Error::AuthFailCtxNotInRequestExt.into_response()
        }
    }
   // next.run(req).await
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    )
        .ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}