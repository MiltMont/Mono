use colored::Colorize;
use text_io::read;

use crate::{lexer::Lexer, token::TokenType};

pub fn start() {
    loop {
        print!("{} ", ">>".yellow());
        let input: String = read!("{}\n");

        let mut lexer = Lexer::new(input);

        loop {
            let token = lexer.next_token();

            if token.typ != TokenType::EOF {
                println!("{:?}", token);
            } else {
                break;
            }
        }
    }
}
