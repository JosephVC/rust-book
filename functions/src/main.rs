fn main() {

    println!("hello from the main function");
    another_function();

    argument_function(5, 6);
}

fn another_function() {
    println!("another function");
}

fn argument_function(x: i32, y: i32) {
    println!("{}", x);
    println!("{}", y);
}
