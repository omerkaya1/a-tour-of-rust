/// This program demonstrates the use of mutable and immutable variables
fn main() {
    // mutable variable: declaring it as mut allows it to be changed
    let mut x = 5;
    println!("The value of x is: {}", x);
    println!("{x}"); // {} is a placeholder for the value of x
    x = 6;
    println!("The value of x is: {}", x);

    // unitype
    println!("unitype");
    let y: i32 = 5;
    let y: () = {
        let y: i32 = 3;
    };
    println!("{:?}", y); // () is a unit type, it has no value

    // function calls
    println!("function calls: {}", double(5));

    // duble or zero
    println!("duble or zero");
    println!("The value of x is: {}", double_or_zero(5));
    println!("The value of x is: {}", double_or_zero(0));

    // option type
    println!("option type");
    let option: Option<i32> = Some(5);
    println!("The value of option is: {:?}", double_or_option(option));
    let option: Option<i32> = None;
    println!("The value of option is: {:?}", double_or_option(option));

    // owwnership concepts
    println!("owwnership concepts");
    let s1 = String::from("hello");
    ownership(s1.clone());
    ownership(s1);
    // ownership(s1); <- won't work as the ownership of s1 has been moved to ownership function
    // ownership(s1.clone()); // won't work either; the cloning should be done before the ownership transfer

    // borrowing
    println!("borrowing");
    let s2 = String::from("hello");
    ownership_borrow(&s2);

    // mutable borrowing
    println!("mutable borrowing");
    let mut s3 = String::from("hello");
    ownership_borrow_mutable(&mut s3);
}

fn ownership(s: String) {
    // s is a reference
    println!("{}", s);
}

fn ownership_borrow(s: &String) {
    // s is a reference
    // s is a reference
    println!("{}", s);
}

fn ownership_borrow_mutable(s: &mut String) {
    // s is a reference
    *s = format!("Some {}", s);
    println!("{}", s);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn double_or_zero(x: i32) -> i32 {
    if x > 0 {
        return x * 2;
    }
    0 // naked return
}

fn double_or_option(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x * 2),
        None => None,
    }
}
