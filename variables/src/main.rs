fn main() {
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // // let mut x = 5;
    
    // // println!("The value of x is: {}", x);
    // // x = 6;
    // println!("The value of x is: {}", x);

    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!("the length of spaces is {}", spaces);

    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    //subtraction
    let difference = 95.5 - 4.2;
    println!("{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    // boolean types
    let t = true;
    let f: bool = false;
    println!("{}", t);
    println!("{}", f);

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // tuple
        let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);

    // the below does not work, as the type that tup is (tuple) can't be 
    // printed like it were a single variable
    // println!("{}", tup);

    let x: (i32, f64, u8) = (500, 6.66, 1);

    // now, this is fucking cool
    let five_hundred = x.0;
    let six_six_six = x.1;
    let one = x.2;

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // here we specify an array will contain five 32 bit integers
    let a: [i32; 5] = [1,2,3,4,5];

    // again, the below won't print
    // println!("{}", a);

    //this will contain five elements, all with the value 3
    let a = [3;5];

    // standard issue creating array, then creating new variables based on indexes in that array
    let a = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];

    println!("{}", first);
}
