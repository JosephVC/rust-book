fn main() {

//     println!("hello from the main function");
//     another_function();

//     argument_function(5, 6);
// }

// fn another_function() {
//     println!("another function");
// }

// fn argument_function(x: i32, y: i32) {
//     println!("{}", x);
//     println!("{}", y);

let foo = plus_one(5);

println!("The vlaue of foo is {}", foo);

 let y = {
     let x = 3;
     x + 1
 };

 println!("teh value of y is {}", y);
    
let x = five();

println!("the value of x is {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// no
// this won't run as it does not return a value
// in other languages, the very assignment returns the value of the assignment

// let x = (let y = 6);
// println!("{}", x);
