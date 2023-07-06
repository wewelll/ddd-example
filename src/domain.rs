use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Product {
    id: Uuid,
    name: String,
    price: f32,
    created_at: NaiveDateTime,
}

impl Product {
    pub fn new(name: String, price: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            price,
            created_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> f32 {
        self.price
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product() {
        let product = Product::new("test".to_string(), 1.0);
        assert_eq!(product.name(), "test");
        assert_eq!(product.price(), 1.0);
    }
}
