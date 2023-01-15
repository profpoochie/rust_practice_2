fn main() {
    /* for i in 1..6 {
        say_hi()
    } */
    let mut name = "John";
    say_hello(&mut name);
    println!("{}", name);
}

fn say_hello(name: &mut &str) {
    println!("Hello {}", name);
    *name = "Alex";
}

// fn say_hi() {
//     println!("Hello there");
// }
