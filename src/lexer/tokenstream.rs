use super::token::Token;
use crate::element::ElementPosition;


/// list of tokens and their respective position generated from a program file (.slo)
#[derive(Clone)]
pub struct TokenStream {
    pub filename: String,
    pub tokens: Vec<(Token, ElementPosition)>,
    nb_tokens: usize,
    current: usize,
}


impl TokenStream {

    pub fn new(filename: String, tokens: Vec<(Token, ElementPosition)>, nb_tokens: usize, current: usize) -> TokenStream {
        TokenStream { filename, tokens, nb_tokens, current }
    }



    /// return the nth value of the iterator
    pub fn nth(&self, index: usize) -> Option<(Token, ElementPosition)> {
        if index >= self.nb_tokens {None}
        else {Some(self.tokens[index].clone())}
    }


    /// return current value of the iterator
    pub fn current(&self) -> Option<(Token, ElementPosition)> {
        self.nth(self.current)
    }


    /// switch to the next value of the iterator and returns it
    pub fn next(&mut self) -> Option<(Token, ElementPosition)> {
        self.current += 1;
        self.nth(self.current)
    }


    /// return the nth next value without switching to it
    pub fn peek(&mut self, nth: isize) -> Option<(Token, ElementPosition)> {
        if nth < 0 {self.nth(self.current - (-nth as usize))}
        else {self.nth(self.current + nth as usize)}
    }

    /// Print to the console the list of tokens
    pub fn print_tokens(&self) {
        for i in 0..self.tokens.len() {
            println!("{:<10}{:40}{:40}", i+1, self.tokens[i].0.to_string_formatted(), self.tokens[i].1.to_string());
        }
    }
}