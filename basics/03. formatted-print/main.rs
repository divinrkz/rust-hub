fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 4);

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}, {2}", "Alice", "Bob", "Divin");


    // As can named arguments.
    println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over me");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=3);

    
    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:d>width$}", number=1, width=6);


    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

     // Create a structure named `Structure` which contains an `i32`.
     #[allow(dead_code)]
     struct Structure(i32);
     
     // However, custom types such as this structure require more complicated
    // handling. This will not work.

    // println!("This struct `{}` won't print...", Structure(3));

}