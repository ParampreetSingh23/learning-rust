fn main() {
 let age : Option<i32> = Some(21);
 let missing_age : Option<i32> = None;
 println!("{:#?}",age);
 println!("{:?}",missing_age);
 let balance : Option<i32> =  Some(2100);
check_balance(balance);
check_balance_if_let(balance);

}

fn check_balance(bal : Option<i32>) {
   match bal {
     Some(value) =>{
      println!("Balance {}",value);
     }
     None => {
      println!("Account Empty");
     }
   }
}
//if let 
fn check_balance_if_let(bal : Option<i32>) {
   if let Some(value) = bal {
      println!("Balance {}",value);
   } else {
      println!("Account Empty");
   } 
}