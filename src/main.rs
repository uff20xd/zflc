mod tokenizer;
mod compiler;
use tokenizer::tokenizer::Lexer;
use tokenizer::tokens::Token;
use compiler::compiler::Compiler;
use parser::parser::Parser;
use std::{
    env,
    fs,
    io::BufRead,
    io::Read,
    io::Write,
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //gets args
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 { panic!("Please provide a file!") }

    let mut raw_file = fs::File::open(&args[1])?;
    let mut raw_file_contents = String::new();
    raw_file.read_to_string(&mut raw_file_contents)?;

    let source_code_in_lines_bor = raw_file_contents.lines();
    let mut source_code_in_lines = Vec::new();
    for line in source_code_in_lines_bor {
        source_code_in_lines.push(line.to_string());
    }

    let mut lexer = Lexer::new(source_code_in_lines)?;
    let tokens = lexer.lex()?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

//    let mut compiler = Compiler::new(tokens);
//    let assembly = compiler.compile();
//
//    let mut assembly_file = fs::File::create("out/out.asm")?;
//    write!(&mut assembly_file, "{}", &assembly);
//
//    _ = Command::new("fasm").args(&["./out/out.asm", "./out/out"]).status()?;
//    _ = Command::new("./out/out").status()?;

    //println!("{:?}", &assembly);


    Ok(())
}
