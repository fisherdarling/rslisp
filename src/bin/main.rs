use rslisp::{
    eval::{create_env, eval},
    lexer::Lexer,
    parser::parse,
    types::Type,
};

fn main() {
    let code = "(define (add-new x y) (+ x y)) (add-new 1 1)";
    // let code = "(+ 1 1)";

    let mut lexer = Lexer::new(code);

    let mut sexprs = parse(&mut lexer).collect::<Vec<Type>>().into_iter();

    let mut env = create_env();

    // println!("code: {}", code);
    // println!("Sexprs: {:?}", sexprs);
    // println!("Env: {:?}", env);

    while let Some(t) = sexprs.next() {
        let ans = eval(t, &mut env);

        println!("eval: {:?}", ans);
    }

    // println!("{:?}", env);

    // let answer = eval(sexprs[0], &mut env);

    // println!("eval: {:?}", answer)
}
