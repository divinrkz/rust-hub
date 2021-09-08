// Custom

// Some conditionals like target_os are implicitly provided by rustc,
//  but custom conditionals must be passed to rustc using the --cfg flag.


#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}






// $ rustc --cfg some_condition custom.rs && ./custom
// condition met!