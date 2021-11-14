pub fn run() {
    type_synonym_example();
    type_synonym_bad_example_to_reduce_repitition();
    type_synonym_good_example_to_reduce_repitition();
    never_type_example();
}

type Kilometers = i32;

fn type_synonym_example() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x = {}, y = {}", x, y);
}

fn type_synonym_bad_example_to_reduce_repitition() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        println!("Doing something with it");
    }

    fn return_long_type() -> Box<dyn Fn() + Send + 'static>{
        Box::new(|| println!("hi"))
    }
}

fn type_synonym_good_example_to_reduce_repitition() {
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        println!("Doing something with it");
    }
    fn return_long_type() -> Thunk {
        Box::new(|| println!("hi"))
    }
}

fn never_type_example() {
    // never();
}

// Never cannot compile successfully
// fn never() -> ! {
// }

fn guess() {
    let input = "5 ";
    let guess: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

}