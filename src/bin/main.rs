use rslisp::{
    lexer::Lexer,
    parser::parse,
    types::Type,
};

fn main() {
    let code = "(+ (- 6 5) 4)";
    let mut lexer = Lexer::new(code);
    
    let sexprs = parse(&mut lexer).collect::<Vec<Type>>();

    println!("{:#?}", sexprs);
}