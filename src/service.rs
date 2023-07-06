use crate::domain::Product;
use crate::repository::ProductRepository;

pub struct ProductService {
    pub product_repository: ProductRepository,
}

impl ProductService {
    pub async fn list_products(&self) -> Result<Vec<Product>, String> {
        Err("Not implemented".into())
    }

    pub async fn create_product(&self, name: String, price: f32) -> Result<Product, sqlx::Error> {
        let product = Product::new(name, price);
        self.product_repository.create(&product).await?;
        Ok(product)
    }
}
