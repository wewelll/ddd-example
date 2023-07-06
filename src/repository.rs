use crate::domain::Product;
use sqlx::{Error, PgPool};

pub struct ProductRepository {
    pub pool: PgPool,
}

impl ProductRepository {
    pub async fn create(&self, product: &Product) -> Result<(), Error> {
        sqlx::query("INSERT INTO products (id, name, price, created_at) VALUES ($1, $2, $3, $4)")
            .bind(&product.id())
            .bind(&product.name())
            .bind(&product.price())
            .bind(&product.created_at())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
