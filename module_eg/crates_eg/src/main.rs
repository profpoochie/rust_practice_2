//use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;
use rand::{Rng, SeedableRng};

mod archive;

fn main() {
    arc("somefile.text");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
}
