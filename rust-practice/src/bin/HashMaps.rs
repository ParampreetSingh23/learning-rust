use std::{collections::HashMap, ptr::hash};
fn restock(
    stock: &mut HashMap<&str, i32>,
    product: &'static str,
    quantity: i32,
) {
    let count = stock.entry(product).or_insert(0);
    *count += quantity;
}
fn main() {
 let mut map = HashMap::new();
 map.insert(1, "param");
 map.insert(2, "preet");

 
let name = map.get(&1);
if let Some(val) = name {
 println!("Name: {}",val);
} 
let mut stock = HashMap::new();
stock.insert("Laptop", 3);
restock(&mut stock, "Laptop", 3);
restock(&mut stock, "Webcam", 2);

}