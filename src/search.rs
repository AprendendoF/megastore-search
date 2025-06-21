use std::collections::HashMap;
use crate::product::Product;

pub struct ProductIndex {
    pub name_index: HashMap<String, Vec<Product>>,
    pub brand_index: HashMap<String, Vec<Product>>,
    pub category_index: HashMap<String, Vec<Product>>,
}

impl ProductIndex {
    pub fn new() -> Self {
        Self {
            name_index: HashMap::new(),
            brand_index: HashMap::new(),
            category_index: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.name_index.entry(product.name.clone()).or_default().push(product.clone());
        self.brand_index.entry(product.brand.clone()).or_default().push(product.clone());
        self.category_index.entry(product.category.clone()).or_default().push(product);
    }

    pub fn search_by_name(&self, query: &str) -> Vec<&Product> {
        self.name_index.get(&query.to_lowercase()).map(|v| v.iter().collect()).unwrap_or_default()
    }

    pub fn search_by_brand(&self, query: &str) -> Vec<&Product> {
        self.brand_index.get(&query.to_lowercase()).map(|v| v.iter().collect()).unwrap_or_default()
    }

    pub fn search_by_category(&self, query: &str) -> Vec<&Product> {
        self.category_index.get(&query.to_lowercase()).map(|v| v.iter().collect()).unwrap_or_default()
    }
}