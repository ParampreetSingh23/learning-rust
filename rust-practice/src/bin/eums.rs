#[derive(Debug)]
enum Status {
 Pending , 
 InProgress , 
 Completed ,
}
fn check_task_status(status : &Status){
  match status {
  Status::Pending => {
            println!("Task is pending");
        }
        Status::InProgress => {
            println!("Task is running");
        }
        Status::Completed => {
            println!("Task is completed");
        }
  }
}
fn main(){
  let status = Status::Completed;
  println!("{:?}" , status);
  check_task_status(&status);
}