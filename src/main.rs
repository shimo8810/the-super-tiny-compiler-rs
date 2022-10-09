use the_super_tiny_compiler::tokenizer::tokenizer;

fn main() {
    let s = "(add 2 (subtract 4 2))";
    let tokens = tokenizer(s);
    println!("{:?}", tokens);
}
