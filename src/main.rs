use std::io::BufRead;
use std::io::Write;

fn main() {
    loop {
        let spaces: usize = match console_input("Spaces between each char").parse() {
            Ok(spaces) => spaces,
            Err(err) => {
                println!("Failed to parse input: {}", err);
                continue;
            }
        };

        let out = console_input("Text to be spaced").chars().map(|c| c.to_string() + &" ".repeat(spaces)).collect::<String>();

        println!("{}", out);

        println!("----------");
    }
}

fn console_input(prompt: &str) -> String {
    print!("{}: ", prompt);
    
    let mut stdout = std::io::stdout();
    stdout.flush().unwrap();

    let reader = std::io::stdin();
    let reader_lock = reader.lock();
    let out = reader_lock.lines().next().unwrap().unwrap();
    out
}
