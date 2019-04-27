use im::Vector;
use rslisp::types::{BuiltinFunction, BuiltinMacro, Scope, Type};
use rslisp_derive::builtin;

#[builtin(fn)]
pub fn add(args: Vector<Type>, _scope: &mut Scope) -> Type {
    let mut sum_int: i64 = 0;
    let mut sum_float: f64 = 0.0;

    for value in args {
        match value {
            Type::Int(i) => sum_int += i,
            Type::Float(f) => sum_float += f,
            t => panic!("invalid argument for `add`: {:?}", t),
        }
    }

    if sum_float > 0.0 {
        Type::Float(sum_int as f64 + sum_float)
    } else {
        Type::Int(sum_int)
    }
}

#[test]
fn it_works() {
    let add_fn = add_builtin();

    assert_eq!(add_fn.name(), "add");
    assert_eq!(
        add(
            Vector::new(),
            &mut Scope::new(std::collections::HashMap::new())
        ),
        Type::Int(0)
    );
}
