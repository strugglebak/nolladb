use std::io::{self,stdin,Error,ErrorKind};
use std::process;
use std::io::Write; // <--- bring flush() into scope

extern crate arrayvec;
use arrayvec::ArrayString;

#[macro_use] extern crate scan_fmt;
macro_rules! field_size {
    ($t:ident :: $field:ident) => {{
        let m = core::mem::MaybeUninit::<$t>::uninit();
        let p = unsafe {
            core::ptr::addr_of!((*(&m as *const _ as *const $t)).$field)
        };

        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        size_of_raw(p)
    }};
}

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;
type Username = ArrayString::<COLUMN_USERNAME_SIZE>;
type Email = ArrayString::<COLUMN_EMAIL_SIZE>;

const SIZE_OF_ID: usize = field_size!(Row::id);
const SIZE_OF_USERNAME: usize = field_size!(Row::username);
const SIZE_OF_EMAIL: usize = field_size!(Row::email);

const OFFSET_OF_ID: usize = 0;
const OFFSET_OF_USERNAME: usize = SIZE_OF_ID + OFFSET_OF_ID;
const OFFSET_OF_EMAIL: usize = SIZE_OF_USERNAME + OFFSET_OF_USERNAME;

const SIZE_OF_ROW: usize = SIZE_OF_ID + SIZE_OF_USERNAME + SIZE_OF_EMAIL;


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

#[derive(Debug)]
pub struct Row {
    pub id: u32,
    pub username: Username,
    pub email: Email,
}
impl Row {
    fn new() -> Self {
        Self {
            id: 0,
            username: Username::new(),
            email: Email::new(),
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
        if let Ok((prepare, statement_type, row_data)) = handle_prepare_statement(&input_string_buffer) {
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
    print!("db > ");
    io::stdout().flush().unwrap();
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
    -> Result<(Prepare, StatementType, Row), String> {
    let start_with_insert = input_string_buffer.buffer.starts_with("insert");
    let is_select = input_string_buffer.buffer == "select";
    let mut row_data = Row::new();
    match input_string_buffer.buffer {
        _ if start_with_insert => {
            if let (Some(id), Some(username), Some(email)) = scan_fmt_some!(
                &input_string_buffer.buffer,
                // TODO: here use regex
                "insert {} {} {}",
                u32,
                String,
                String
            ) {
                row_data.id = id;
                row_data.username.push_str(&username);
                row_data.email.push_str(&email);
            } else {
                // TODO: handle error
                return Err(String::from("Error: insert error"));
            }
            Ok((
                Prepare::PrepareSuccess,
                StatementType::StatementTypeInsert,
                row_data,
            ))
        },
        _ if is_select => Ok((
            Prepare::PrepareSuccess,
            StatementType::StatementTypeSelect,
            row_data,
        )),
        _ => Ok((
            Prepare::PrepareUnrecognizedStatement,
            StatementType::StatementTypeDefault,
            row_data,
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
