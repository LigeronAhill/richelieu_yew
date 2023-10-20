use anyhow::Result;
use serde_json::json;


#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    // hc.do_get("/hello?name=Mike").await?.print().await?;
    // hc.do_get("/hello2/Mike").await?.print().await?;


    let req_login = hc.do_post(
        "/api/login",
        json!({
			"username": "demo1",
			"password": "welcome"
		}),
    );
    req_login.await?.print().await?;

    let req_create_product = hc.do_post(
         "/api/products",
         json!({
	 		"name": "Product from Rust - 2"
	 	}),
     );
    req_create_product.await?.print().await?;

    hc.do_get("/api/products").await?.print().await?;

    Ok(())
}