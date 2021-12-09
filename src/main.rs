use std::io::{stdin, Error};
use std::process;

#[derive(PartialEq)]
pub enum MetaCommand {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub enum StatementType {
    StatementTypeDefault,
    StatementTypeInsert,
    StatementTypeSelect,
}

#[derive(PartialEq)]
pub enum Prepare {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

#[derive(Debug)]
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
    loop {
        let mut input_string_buffer = InputStringBuffer::new();
        print_prompt();
        match read_from_stdin() {
            Ok((line, bytes_read)) => {
                input_string_buffer.buffer = line.to_string();
                input_string_buffer.input_string_length = (bytes_read - 1) as isize;
            },
            Err(error) => panic!("Problem reading from stdin: {:?}", error),
        }

        if input_string_buffer.buffer.starts_with(".") {
            if let Ok(meta_command) = handle_meta_command(&input_string_buffer) {
                if meta_command == MetaCommand::MetaCommandSuccess {
                    continue;
                } else if meta_command == MetaCommand::MetaCommandUnrecognizedCommand {
                    println!("Unrecognized command '{}'", input_string_buffer.buffer);
                    continue;
                }
            }
        }

        let mut s = StatementType::StatementTypeDefault;
        if let Ok((prepare, statement_type)) = handle_prepare_statement(&input_string_buffer) {
            s = statement_type;
            if prepare == Prepare::PrepareSuccess {
                // TODO:
            } else if prepare == Prepare::PrepareUnrecognizedStatement {
                println!("Unrecognized keyword at start of '{}'", input_string_buffer.buffer);
                continue;
            }
        }

        execute_statement(s);

        println!("Executed.");
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

fn handle_meta_command<'a>(input_string_buffer: &'a InputStringBuffer)
    -> Result<MetaCommand, ()> {
    match input_string_buffer.buffer {
        _ if
            input_string_buffer.buffer == ".exit" ||
            input_string_buffer.buffer == ".quit" => {
                println!("Exit.");
                drop(input_string_buffer);
                process::exit(0);
            },
        _ => Ok(MetaCommand::MetaCommandUnrecognizedCommand),
    }
}

fn handle_prepare_statement<'a>(input_string_buffer: &'a InputStringBuffer)
    -> Result<(Prepare, StatementType), ()> {
    let start_with_insert = input_string_buffer.buffer.starts_with("insert");
    let is_select = input_string_buffer.buffer == "select";
    match input_string_buffer.buffer {
        _ if start_with_insert => Ok((
            Prepare::PrepareSuccess,
            StatementType::StatementTypeInsert
        )),
        _ if is_select => Ok((
            Prepare::PrepareSuccess,
            StatementType::StatementTypeSelect
        )),
        _ => Ok((
            Prepare::PrepareUnrecognizedStatement,
            StatementType::StatementTypeDefault
        )),
    }
}

fn execute_statement(statement_type: StatementType) {
    match statement_type {
        StatementType::StatementTypeInsert => {
            println!("exec insert");
        },
        StatementType::StatementTypeSelect => {
            println!("exec select");
        },
        _ => ()
    }
}
