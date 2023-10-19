use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Clone)]
pub struct ModelController {
    db: Surreal<Client>
}

impl ModelController {
    pub async fn new() -> Self {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.expect("Failed to connect to db");

        db.signin(Root {
            username: "root",
            password: "root",
        })
            .await.expect("Failed to sign in");

        db.use_ns("richelieu").use_db("richelieu").await.expect("Failed to use db");
        Self { db }
    }
    pub fn db(&self) -> Surreal<Client> {
        self.db.clone()
    }
}
