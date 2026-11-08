struct Product {
    id: u32,
    name: String,
    price: u32,
}
fn find_product(id : u32) ->Option<Product>{
  if id==1 {
   Some(Product {
    id:1,
    name : String::from("Laptop"),
    price : 50000
   })
  }else{
   None
  }
}
fn main() {
  let product = find_product(1);
  if let Some(value) = product {
   println!("Product: {}",value.name)
  }else{
   println!("No Product")
  }
}