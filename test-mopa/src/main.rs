#[macro_use]
extern crate mopa;

use std::fmt;

use rand::Rng;

#[derive(Debug)]
struct Sheep {}

#[derive(Debug)]
struct Cow {}

trait Animal: mopa::Any {
    fn noise(&self) -> &'static str;
}

mopafy!(Animal);

impl fmt::Debug for Box<dyn Animal> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if self.is::<Sheep>() {
            writeln!(f, "Sheep")
        } else if self.is::<Cow>() {
            writeln!(f, "Cow")
        } else {
            panic!()
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen();
    let animal = random_animal(random_number);
    dbg!(&animal);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
