fn main() {
    /* for i in 1..6 {
        say_hi()
    } */
    let mut name = "John";
    say_hello(name);
    println!("{}", name);
}

fn say_hello(name: &str) {
    println!("Hello {}", name);
}

// fn say_hi() {
//     println!("Hello there");
// }
