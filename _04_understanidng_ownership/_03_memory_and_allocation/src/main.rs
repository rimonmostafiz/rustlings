fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }   // this scope is now over, and s is no longer valid

    // As x and y are scaler type,
    // we now have two variables, x and y, and both equal 5.
    // This is indeed what is happening, because integers are simple values with a known,
    // fixed size, and these two 5 values are pushed onto the stack.
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // Now this part of the code looks very similar to code above
    // Creating a String s1 and then copying value of s1 to s2
    // But this isn’t quite what happens!!!
    // To ensure memory safety, after the line let s2 = s1;
    // Rust considers s1 as no longer valid. Therefore,
    // Rust doesn’t need to free anything when s1 goes out of scope.
    // Check out what happens when you try to use s1 after s2 is created; it won’t work:
    let s1 = String::from("hello");
    let s2 = s1;
    // This line will generate error if we try to run as s1 no longer valid at this point!
    //println!("s1: {}, s2: {}", s1, s2);
    println!("s2: {}", s2);

    // So what's the way to copy value of a complex variable to another?
    // We can use the .clone() method
    let str1 = String::from("hello");
    let str2 = str1.clone();

    println!("str1 = {str1}, str2 = {str2}");
}