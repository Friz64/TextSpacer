extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::io::BufRead;

fn main() {
    // The string that's going to be outputted
    let mut output = String::new();

    // Used for console Input
    let reader = std::io::stdin();

    // Get the string that should be spaced from the console
    println!("Text to be spaced: ");
    let mut inputstring = reader.lock().lines().next().unwrap().unwrap();

    // Get the spaces that should be between each character from the console
    println!("Spaces between each char: ");
    let inputspaces: i32 = reader.lock().lines().next().unwrap().unwrap().parse().expect("input is not a number");

    // Iterate through all characters in the input
    // Add requested amount of spaces to each character and add it to the output
    for curchar in inputstring.split("") {
        output += &format!("{}{}", curchar, " ".repeat(inputspaces as usize))
    }

    // remove the unnecessary spaces at the beginning and end of the output string
    output = output.trim().to_string();

    // simply print it out
    println!("{}", output);

    // set it to the clipboard with the clipboard crate
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output.to_owned()).unwrap();
    println!("\nSuccessfully set the output to the clipboard!");

    // R     E     C     U     R     S     I     O     N
    println!("\n----------\n");
    main()
}
