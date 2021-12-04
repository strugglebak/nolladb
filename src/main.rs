use std::io::{stdin, Error};

pub struct InputStringBuffer {
    pub buffer: String,
    pub input_string_length: isize,
}

impl InputStringBuffer {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            input_string_length: 0,
        }
    }
}

fn main() {
    let mut input_string_buffer = InputStringBuffer::new();

    loop {
        print_prompt();
        match read_from_stdin() {
            Ok((line, bytes_read)) => {
                input_string_buffer.buffer = line.to_string();
                input_string_buffer.input_string_length = (bytes_read - 1) as isize;
            },
            Err(error) => panic!("Problem reading from stdin: {:?}", error),
        }

        match input_string_buffer.buffer {
            _ if
                input_string_buffer.buffer == ".exit" ||
                input_string_buffer.buffer == ".quit" => {
                    println!("db exit");
                    break;
                },
            _ => println!("Unrecognized command '{}'.\n", input_string_buffer.buffer),
        }
    }
}

fn print_prompt() {
    println!("db > ");
}

fn read_from_stdin() -> Result<(String, usize), Error> {
    let mut line = String::new();
    let bytes_read = stdin().read_line(&mut line)?;

    if bytes_read <= 0 {
        panic!("Error reading from stdin");
    }

    // ignore '\r\n'
    line.pop();

    Ok((line, bytes_read))
}
