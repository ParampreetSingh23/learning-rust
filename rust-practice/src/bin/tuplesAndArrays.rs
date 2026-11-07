fn main() {
 //Tuples
 let student = ("Param" , 21 , 5.10);
 println!("{:?}" , student);
 //getting by index
  println!("Name = {}" , student.0);
  println!("Age = {}" , student.1);
  println!("height = {}" , student.2);
 //Declaring tuple types explicitly
 let person: (&str , i32 , bool) = ("pampi" , 25 , true);

println!("{:?}" , person);

//Destructing tuple 
 let (name, age, marks) = student;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Marks: {}", marks);



    //Arrays

    let marks = [80,90,100,91];
    println!("{:?}",marks);
    //indexing
       println!("{}",marks[0]);
       println!("{}",marks[1]);

    //Declaring an array with a type
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    
}