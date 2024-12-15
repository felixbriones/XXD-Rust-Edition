// import std::io for stdin and stdout.
// import self: import allows us to refer to the module itself
// import BufRead: import allows us read from buffered source
use std::io::{self, BufRead};
use hex::encode;

fn main()
{
    // 'let' keyword allows us to declare an immuatable variable
    let stdin = io::stdin();

    // Create a locked version/handle of the standard input, preventing other parts of program 
    // from using stdin at the same time possibly preventing race conditions.
    // Locking stdin also lets us read larger chunks of data at once instead of char at a time
    let handle = stdin.lock();

    // Handles new line delimiters. Buffers input for efficient reading. Goes line by line
    for line in handle.lines()
    {
        // unwrap extracts the string value if operation succeeds
        let line = line.unwrap();
        let hex = encode(line);
        println!("Ouput: {}", hex);
    }
}
