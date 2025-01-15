mod scanner;
mod token;
mod token_type;
mod expr;
mod ast_printer;
mod parser;

use crate::scanner::Scanner;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::ast_printer::AstPrinter;
use crate::parser::Parser;

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

pub struct Rox
{
    had_error: bool,
}

impl Rox
{
    pub fn new() -> Self
    {
        Rox { had_error: false }
    }

    fn main()
    {
        let args: Vec<String> = env::args().collect();
        let mut rox = Rox::new();

        match args.len()
        {
            1 => rox.run_prompt(),
            2 => rox.run_file(&args[1]),
            _ =>
            {
                eprintln!("Usage: rox [script]");
                std::process::exit(64);
            }
        }
    }

    fn run_file(&mut self, path: &str)
    {
        match fs::read_to_string(path)
        {
            Ok(content) =>
            {
                self.run(&content);
                if self.had_error
                {
                    std::process::exit(65);
                }
            }
            Err(e) =>
            {
                eprintln!("Error reading file: {}", e);
                std::process::exit(66);
            }
        }
    }

    fn run_prompt(&mut self)
    {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut input = String::new();

        loop
        {
            print!("> ");
            stdout.flush().unwrap();

            input.clear();
            if stdin.lock().read_line(&mut input).is_err() || input.is_empty()
            {
                break;
            }

            self.run(input.trim());
            self.had_error = false;
        }
    }

    fn run(&mut self, source: &str)
    {
        let mut scanner = Scanner::new(source.to_string(), self);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(&tokens);
        let expression = parser.parse();

        // Stop if there was a syntax error
        if self.had_error
        {
            return;
        }

        let mut ast_printer = AstPrinter;

        // Dereference expr
        match expression
        {
            Some(ref expr) =>
            {
                println!("{}", ast_printer.print(expr));
            }
            None =>
            {
                println!("No expression to print");
            }
        }
    }

    fn error(&mut self, line: usize, message: &str)
    {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str)
    {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }

    pub fn error_token(&mut self, token: &Token, message: &str)
    {
        if token.token_type == TokenType::Eof
        {
            self.report(token.line, " at end", message);
        }
        else
        {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }
}

fn main() {
    Rox::main();
}
