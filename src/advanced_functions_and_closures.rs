pub fn run() {
    function_pointer_example();
    second_function_pointer_example();
}

fn function_pointer_example() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn second_function_pointer_example() {
    // First version with closure
    let list_of_numbers = vec![1,2,3];
    let list_of_strings = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // Second version with function pointer
    let list_of_numbers = vec![1,2,3];
    // The passed in function needs to be written as fully qualified syntax
    let list_of_strings = list_of_numbers.iter().map(ToString::to_string).collect();

    //Another example. The tuple enum value initialization can be used here as a function in the map call.
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

enum Status {
    Value(u32),
    Stop,
}

fn returning_closures_example() {

}

/*
fn this_wont_compile() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
 */

// But this
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}