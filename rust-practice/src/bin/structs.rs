#[derive(Debug)]
struct Book {
  title: String , 
  pages : u32 , 
   available : bool , 
}
fn check_availability(book:&Book) {
  if book.available {
    println!("Available");
  }
  else {
    println!("Unavailable");
  }
}
impl Book{
  fn display(&self){
    println!("{}",self.title);
    println!("{}",self.pages);

  }
  fn new(title:String , pages:u32) -> Self{
    Self {
      title ,
      pages , 
      available:true,
    }
  }
}

fn main() {
 let mut book = Book{
   title: String::from("Rust Programming"),
   pages: 300,
   available: true,
 };

 println!("Title : {}" ,book.title);
 println!("Pages : {}" ,book.pages);
 book.available = false;
  println!("{:?}" ,book);

check_availability(&book);
book.display();

}