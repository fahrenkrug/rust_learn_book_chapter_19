use std::ops::Add;
use std::fmt::{Display, Formatter};

pub fn run() {
    operator_overloading_example();
    traits_with_same_function_name_example();
    traits_with_same_function_name_example_no_self();
    super_traits_example();
    newtype_pattern_example();
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;

    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn operator_overloading_example() {
    let summed_up_point = Point {x: 2, y: 2} + Point { x: 1, y: 1};
    assert_eq!(summed_up_point, Point {x: 3, y: 3});
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("V1 reached! Going up now!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wingardium levio-MEEEE");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms*");
    }
}

fn traits_with_same_function_name_example() {
    let h = Human;
    h.fly();
    // How can we specifically call the other fly fn's ?
    Wizard::fly(&h);
    Pilot::fly(&h);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn traits_with_same_function_name_example_no_self() {
    // Not what we want
    // println!("A baby dog is called a {}", Dog::baby_name());

    // Compiler errror
    // println!("A baby dog is called a {}", Animal::baby_name());

    // This is called fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(4 + len));
        println!("*{}*", " ".repeat(2 + len));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(2 + len));
        println!("{}", "*".repeat(4 + len));

    }
}

impl OutlinePrint for Point {
    
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn super_traits_example() {
    let point = Point { x: 345, y: 13};
    point.outline_print();
}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn newtype_pattern_example() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}