fn print_10_times(name:&str){
 for _ in 0..4 {
  println!("{}" , name)
 }
}
fn sum_of_array(arr:&[i32]) -> i32 {
 let mut sum = 0;
 let len = arr.len();
 for i in 0..len {
   sum = sum + arr[i];
 }
 sum 
}
fn main(){
 print_10_times("Param");
 let arr  = [1,2,3,4,5];
 println!("{}" ,sum_of_array(&arr));
}