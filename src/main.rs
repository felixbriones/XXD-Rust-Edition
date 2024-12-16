#![allow(non_snake_case)]
use std::io::Read;
use std::io;

// Equivalent of a macro in C
const BYTES_PER_LINE: usize = 16;
const HEX_BYTES_SPACING: usize = 2;

fn main()
{
    // 'let' keyword allows us to declare an immuatable variable
    // add mut keyword to make mutable
    let stdin = io::stdin();

    // Create a locked version/handle of the standard input, preventing other parts of program 
    // from using stdin at the same time possibly preventing race conditions.
    // Locking stdin also lets us read larger chunks of data at once instead of char at a time
    let mut handle = stdin.lock();

    // XXD prints 16 bytes per line / read 16 bytes at a time ([value; length]
    let mut buffer = [0 ; BYTES_PER_LINE];    
    let mut offset = 0;

    // enum Result: defined by Rust. Returns Ok if True / Err if False 
    // Contents of buffer are read from handle into buffer 
    // Number of bytes read are returned as Result's Ok value
    // match statement: if result is ok, extract value of bytesRead
    // Scope of Ok/Err: block they are introduced in (match, if let, while let)
    // Note a bytesRead == 0 does not result in Err being given a value
    while let Ok(bytesRead) = handle.read(&mut buffer)
    {
        if bytesRead == 0
        {
            println!("EOF. Exiting...");
            break;
        }
        // add Err logic in event of stream error 

        // Print the offset 
        print!("{:08x}: ", offset);

        // 0..bytesREad: array slice ranging from 0 to bytesRead
        // Print bytes in ASCII, 2 bytes (4 chars) at a time
        for i in 0..bytesRead
        {
            print!("{:02x}", buffer[i]);
            // Add a space after every 2 bytes for readability and to mimic XXD
            if i % HEX_BYTES_SPACING == 1
            {
                print!(" ");
            }
        }

        // if fewer than 16 bytes were printed on this line, use spaces to pad
        for i in bytesRead..BYTES_PER_LINE
        {
            // Add spacing for a byte
            print!("  ");
            if i % HEX_BYTES_SPACING == 1
            {
                // Account for the space added every 2 bytes
                print!(" ");
            }
        }

        // Add an extra space between the byte and ASCII columns
        print!(" ");

        // Print hex data's ASCII representation
        for &bytes in &buffer[..bytesRead]
        {
            // print byte data as a char
            print!("{}", bytes as char);
        }

        // Update offset
        offset += bytesRead;

        println!("");
    }
}
