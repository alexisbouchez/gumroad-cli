use crate::error::Result;
use crate::models::product::Product;

use super::GumroadClient;

impl GumroadClient {
    pub async fn list_products(&self) -> Result<Vec<Product>> {
        let body = self.get("/v2/products").await?;
        let products: Vec<Product> = serde_json::from_value(body["products"].clone())?;
        Ok(products)
    }

    pub async fn get_product(&self, id: &str) -> Result<Product> {
        let body = self.get(&format!("/v2/products/{id}")).await?;
        let product: Product = serde_json::from_value(body["product"].clone())?;
        Ok(product)
    }

    pub async fn create_product(&self, params: &[(&str, &str)]) -> Result<Product> {
        let body = self.post("/v2/products", params).await?;
        let product: Product = serde_json::from_value(body["product"].clone())?;
        Ok(product)
    }

    pub async fn update_product(&self, id: &str, params: &[(&str, &str)]) -> Result<Product> {
        let body = self.put(&format!("/v2/products/{id}"), params).await?;
        let product: Product = serde_json::from_value(body["product"].clone())?;
        Ok(product)
    }

    pub async fn delete_product(&self, id: &str) -> Result<()> {
        self.delete(&format!("/v2/products/{id}")).await?;
        Ok(())
    }

    pub async fn enable_product(&self, id: &str) -> Result<Product> {
        let body = self.put(&format!("/v2/products/{id}/enable"), &[]).await?;
        let product: Product = serde_json::from_value(body["product"].clone())?;
        Ok(product)
    }

    pub async fn disable_product(&self, id: &str) -> Result<Product> {
        let body = self.put(&format!("/v2/products/{id}/disable"), &[]).await?;
        let product: Product = serde_json::from_value(body["product"].clone())?;
        Ok(product)
    }
}
