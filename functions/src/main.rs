fn main() {
    /* for i in 1..6 {
        say_hi()
    } */
    let mut name = "John";
    let greeting = say_hello(&mut name);
    println!("{}", greeting);
}

fn say_hello(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    return greeting;
}

// fn say_hi() {
//     println!("Hello there");
// }
