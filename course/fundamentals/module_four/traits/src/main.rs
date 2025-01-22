#![allow(dead_code, unused_variables)]
use std::{any::Any, fmt};

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

trait Animal: fmt::Debug {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Shiiiiiiz)");
    }
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("snoopy sais...");
    }
}

fn speak(animal: &impl Animal) {
    animal.speak();
    println!("{:?}", animal);
}

fn make_animal() -> impl Animal {
    Cat
}

trait DowncastableAnimal {
    fn speak(&self) {
        println!("meh")
    }
    fn as_any(&self) -> &dyn Any;
}

struct Chachi;

impl DowncastableAnimal for Chachi {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", Point { x: 1, y: 2 });

    let cat = Cat;
    speak(&cat);

    let dog = Dog;

    speak(&dog);

    let animal = make_animal();

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];

    animals.iter().for_each(|a| {
        a.speak();
    });

    let downcastable_animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Chachi)];

    downcastable_animals.iter().for_each(|a| {
        if let Some(t) = a.as_any().downcast_ref::<Chachi>() {
            println!("it's chachi!")
        }
    });
}
