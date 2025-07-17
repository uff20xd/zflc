mod tokenizer;
use tokenizer::tokenizer::Lexer;
use tokenizer::tokens::Token;
use std::{
    env,
    fs,
    io::BufRead,
    io::Read,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //gets args
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 { panic!("Please provide a file!") }

    let mut raw_file = fs::File::open(&args[1])?;
    let mut raw_file_contents = String::new();
    raw_file.read_to_string(&mut raw_file_contents)?;

    let source_code_in_lines = raw_file_contents.lines();

    let lexer = Lexer::new(raw_file_contents)?;
    let tokens = lexer.lex();

    println!("{:?}", raw_file_contents);

    Ok(())
}
