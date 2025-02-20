let mut s1 = String::from("hello");
let s2 = &mut s1; // Mutable borrow
s2.push_str(", world"); // Now s1 is "hello, world"
