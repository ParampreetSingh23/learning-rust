fn add_ten(number: &mut i32) {
     *number = *number +10;
}

fn main() {
    let mut x = 20;

    add_ten(&mut x);

    println!("{}", x);
}