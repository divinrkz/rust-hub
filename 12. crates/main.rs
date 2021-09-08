// A crate is a compilation unit in Rust. Whenever rustc some_file.rs is called,
//  some_file.rs is treated as the crate file. 
// If some_file.rs has mod declarations in it, 
// then the contents of the module files would be inserted in places
//  where mod declarations in the crate file are found,
//  before running the compiler over it. 
// In other words, modules do not get compiled individually,
//  only crates get compiled.


// To link a crate to this new library you may use rustc's --extern flag.
//  All of its items will then be imported under a module named the same as the library.
//  This module generally behaves the same way as any other module.


// extern crate rary; // May be required for Rust 2015 edition or earlier

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}