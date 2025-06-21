mod product;
mod search;

use product::Product;
use search::ProductIndex;

fn main() {
    let mut index = ProductIndex::new();

    // Produtos de exemplo
    index.add_product(Product::new(1, "Notebook Lenovo", "Lenovo", "Eletrônicos"));
    index.add_product(Product::new(2, "Camisa Polo", "Nike", "Vestuário"));
    index.add_product(Product::new(3, "Smartphone Galaxy", "Samsung", "Eletrônicos"));

    let results = index.search_by_name("notebook");
    println!("Resultados da busca: {:?}", results);
}