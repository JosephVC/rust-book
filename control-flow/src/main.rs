fn main() {

    // straight up loops
    let mut counter = 0;

    let result = loop {
        counter +=1; 

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {}", result);

    // while loops
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("asfafaff")

    // for loops

    // the below will really just loop forever
    // loop {
    //     println!("I will run forever!");
    //     break;
    // }

    // let condition  = true;
    // let number = if condition {
    //     5
    // } else {
    //     6
    // };

    // println!("the value of theh number is {}", number);


    // let number = 6;
    // if number %4 == 0 {
    //     println!("number divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number divisible by 2");
    // }
    // let number = 6;

    // if number < 5 {
    //     println!("yay true");
    // } else {
    //     println!("uh false");
    // }
}