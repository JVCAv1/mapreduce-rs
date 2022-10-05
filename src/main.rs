
struct Person{
    name: String,
    age: i32,
}

impl Person{
    fn from(name: &str, age: i32) -> Person {
        Person{name: name.to_string(), age}
    }

    fn set_name(&mut self, name: &str) -> &mut Self{
        self.name = name.to_string();
        self
    }

    fn set_age(&mut self, age: i32) -> &mut Self{
        self.age = age;
        self
    }
}

fn main() {
    let mut p = Person::from("pedro", 33);
    println!("name: {} age: {}", p.name, p.age);
    p.set_name("joao").set_age(55);
    println!("name: {} age: {}", p.name, p.age);
}