fn main() {
    let triple = (0, -2, 3);
    // Tuples can be destructured in a match as follows:
    println!("Tuple: {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
                // `..` can be the used ignore the rest of the tuple
                _      => println!("It doesn't matter what they are"),
                // `_` means don't bind the value to a variable
    }
}