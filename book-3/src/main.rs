fn main() {
    // Book section 3.1
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is now {x}");
    //
    // const THREE_HOURS_IN_SECONDS: u32 = 3*60*60;
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x + 3;
    //     println!("The value of x in this scope is {x}");
    // }
    // println!("The value of x in this scope is {x}");

    // Book section 3.2
    // Just a review of the common or basic data types in Rust, not much in the way of code to show here

    // Book section 3.3
    // An overview of functions, example shown commented out below (indents and nesting are an issue for this section)
    // fn main() {
    //     let x = plus_one(5);
    //
    //     println!("The value of x is: {x}");
    // }
    //
    // fn plus_one(x: i32) -> i32 {
    //     x + 1
    // }

    // Book section 3.4
    // Shows how to do comments, already figured that one out on my own

    // Book section 3.5
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let new_number = if condition { 5 } else { 6 };

    println!("The value of number is: {new_number}");  // IDE doesn't like this way, but it DOES work

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");
}
