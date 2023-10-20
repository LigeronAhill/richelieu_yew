use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use anyhow::Result;
use crate::ctx::Ctx;
use crate::storage::ModelController;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Thing,
    pub name: String,
    pub cid: u64, // creator user_id
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductForCreate {
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProductWithCtx {
    name: String,
    cid: u64,
}


impl ModelController {
    pub async fn create_product(&self, ctx: Ctx, product_from_user: ProductForCreate) -> Result<Product> {
        let product = ProductWithCtx {
            name: product_from_user.name,
            cid: ctx.user_id(),
        };
        let pushed_product: Vec<Product> = self.db()
            .create("product")
            .content(product)
            .await?;
        Ok(pushed_product[0].clone())
    }
    pub async fn list_products(&self, _ctx: Ctx) -> Result<Vec<Product>> {
        let products = self.db().select("product").await?;
        Ok(products)
    }

    pub async fn delete_product(&self, _ctx: Ctx, id: Id) -> Result<Product> {
        let deleted_product: Option<Product> = self.db().delete(("product", id)).await?;
        Ok(deleted_product.unwrap())
    }
}
