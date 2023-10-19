use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use anyhow::Result;
use crate::storage::ModelController;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Thing,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductForCreate {
    pub name: String,
}

impl ModelController {
    pub async fn create_product(&self, product: ProductForCreate) -> Result<Product> {
        let pushed_product: Vec<Product> = self.db()
            .create("product")
            .content(product).await?;
        Ok(pushed_product[0].clone())
    }
    pub async fn list_products(&self) -> Result<Vec<Product>> {
        let products = self.db().select("product").await?;
        Ok(products)
    }

    pub async fn delete_product(&self, id: Id) -> Result<Product> {
        let deleted_product: Option<Product> = self.db().delete(("product", id)).await?;
        Ok(deleted_product.unwrap())
    }
}
