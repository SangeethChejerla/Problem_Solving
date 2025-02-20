let mut s1 = String::from("hello");
let s2 = &mut s1; // Mutable borrow
s2.push_str(", world"); // Now s1 is "hello, world"


// mutable references in function
/*program starts execution in the main function where x is initialized to 10. 
It then calls modify_value with a mutable reference to x, 
allowing the function to change x to 30. Upon returning to main,
 the updated value of x is printed, resulting in the output Value of x """
 after modification: 30. */
fn modify_value(x: &mut i32) {
    *x = 30;
}

fn main() {
    let mut x = 10;
    modify_value(&mut x);
    println!("Value of x after modification: {}", x);
}