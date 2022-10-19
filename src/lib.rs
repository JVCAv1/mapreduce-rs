///Person struct with 'name' and 'age'.
pub struct Person{
    name: String,
    age: i32,
}

impl Person{
    ///Takes &str as 'name' and i32 as 'age'  
    /// and returns the struct Person with aformentioned elements populated.
    pub fn from(name: &str, age: i32) -> Person {
        Person{name: name.to_string(), age}
    }

    ///Sets the name element of Person.
    pub fn set_name(&mut self, name: &str) -> &mut Self{
        self.name = name.to_string();
        self
    }

    ///Sets the age element of Person.
    pub fn set_age(&mut self, age: i32) -> &mut Self{
        self.age = age;
        self
    }

    ///Returns value of 'name'.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    ///Returns value of 'age'.
    pub fn age(&self) -> i32 {
        self.age
    }
}

pub struct Mapper<F, I>{
    f: F,
    pub(crate) iter: I,
}

impl<F, I> Mapper<F, I>{

    pub fn map(f: F, i: I) where F: Fn()
    {
        
    }
}