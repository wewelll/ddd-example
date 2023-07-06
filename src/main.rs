mod api;
mod domain;
mod repository;
mod service;

use api::create_app;
use axum::Server;
use sqlx::PgPool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let database_url = "postgres://ddd-example:ddd-example@localhost:5432/ddd-example";
    let pool = PgPool::connect(database_url)
        .await
        .expect("Failed to connect to the database");
    let product_repository = repository::ProductRepository { pool };
    let product_service = service::ProductService { product_repository };

    let app = create_app(product_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
