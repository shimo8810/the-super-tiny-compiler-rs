use the_super_tiny_compiler::{parser::parser, tokenizer::tokenizer};

fn main() {
    let s = "(add 2 (subtract 4 2))";
    let tokens = tokenizer(s).unwrap();
    println!("{:?}", tokens);
    let ast = parser(tokens).unwrap();
    println!("{:?}", ast);
}
