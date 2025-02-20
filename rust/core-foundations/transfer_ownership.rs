let mut opt = Some(String::from("hello"));
let s = opt.take();
println!("{:?}", s); // Some("hello")
println!("{:?}", opt); // None


// Chaining Operations
//take() can be used in method chains to move the value out of an Option and pass it to another function or closure.
let mut opt = Some(10);
opt.take().map(|x| x * 2); // Returns Some(20)