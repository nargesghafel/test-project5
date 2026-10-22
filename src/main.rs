use std::io;
fn main() {
    let mut name = String::new();
    io::stdin().read_line(& mut name).expect("Faild to read line");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let age: i32 = match input.trim().parse() {
        Ok(n) if n >= 1 && n <= 100 => n,
        _ => {
            println!("Invalid age");
            return;
        }
    };

    print!("Hello! My name is {} and I'm {} years old.", name.trim() , age);
    
}
