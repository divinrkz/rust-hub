fn main() {
    // `n` will takes the values: 1, 2 ,,, 100 
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Alternatively, a..=b can be used for a range that is inclusive on both ends. The above can be written as:
    // for n in 1..=100

    let names = vec!["Bob", "Frank", "Ferris"];
    
    for name in names.iter() {
        match name {
            &"Ferris" => println!("I am a rustacean."),
            _ => println!("Hello {}", name),
            //
        }
        println!("names: {:?}", names);
    }




    fn main() {
        let names = vec!["Bob", "Frank", "Ferris"];
    
        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        
        // println!("names: {:?}", names);
        // FIXME ^ Comment out this line
    }


    // into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    // iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}