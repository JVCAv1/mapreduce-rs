mod lib;

use crate::lib::*;

fn main() {
    let mut p = Person::from("pedro", 33);
    println!("name: {} age: {}", p.name(), p.age());
    p.set_name("joao").set_age(55);
    println!("name: {} age: {}", p.name(), p.age());
}