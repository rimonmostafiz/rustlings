fn main() {
    say_hello();
    another_function(5);
    sum_function(5, 6);

    let y = {
        let x = 3;
        x + 1 // expression
    };
    println!("y = {}", y);
}

fn say_hello() {
    println!("Hello!");
}

//- Statements are instructions that perform some action and do not return a value.
//- Expressions evaluate to a resultant value.
fn another_function(x: i32) {
    println!("The value of x is: {}", x); // this is a statement
}

fn sum_function(x: i32, y: i32) -> i32 {
    x + y // this is an expression,
}