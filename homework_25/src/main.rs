use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Debug)]
enum Pet {
    Dog(String),
    Cat(Cat),
}

impl From<Cat> for Pet {
    fn from(cat: Cat) -> Pet {
        Pet::Cat(cat)
    }
}

#[derive(Clone, Debug)]
struct Cat {
    age: u32,
    name: String,
}

impl Cat {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cat: {}, Age: {}", self.name, self.age)
    }
}

impl TryFrom<Pet> for Cat {
    type Error = &'static str;

    fn try_from(pet: Pet) -> Result<Self, Self::Error> {
        match pet {
            Pet::Cat(cat) => Ok(cat),
            Pet::Dog(_) => Err("Cannot convert a Dog into a Cat!"),
        }
    }
}

impl Add<u32> for Cat {
    type Output = Cat;

    fn add(self, rhs: u32) -> Cat {
        Cat {
            name: self.name,
            age: self.age + rhs,
        }
    }
}

impl AddAssign<u32> for Cat {
    fn add_assign(&mut self, rhs: u32) {
        self.age += rhs;
    }
}

fn main() {
    let mut my_cat = Cat {
        name: String::from("Vasya"),
        age: 3,
    };
    println!("Cat's name is: {}", my_cat.get_name());

    let cloned_cat = my_cat.clone();
    println!("Original Cat: {}", my_cat);
    println!("Cloned Cat: {}", cloned_cat);

    let pet: Pet = my_cat.clone().into();
    println!("Pet (from Cat): {:?}", pet);

    let pet = Pet::from(my_cat.clone());
    println!("Yet another pet (from Cat): {:?}", pet);

    match Cat::try_from(pet) {
        Ok(cat) => println!("Converted Pet back to Cat: {}", cat),
        Err(e) => println!("Failed to convert Pet to Cat: {}", e),
    }

    match Cat::try_from(Pet::Dog("Bobik".to_string())) {
        Ok(cat) => println!("Converted Pet back to Cat: {}", cat),
        Err(e) => println!("Failed to convert Pet to Cat: {}", e),
    }

    let older_cat = my_cat.clone() + 2;
    println!("Older Cat: {}", older_cat);

    my_cat += 1;
    println!("Cat after adding 1 year: {}", my_cat);
}
