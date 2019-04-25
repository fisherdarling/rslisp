use crate::types::{BuiltinFunction, BuiltinMacro, Scope, Type, Function};
use crate::eval::eval;


use im::Vector;


pub fn define() -> (String, BuiltinMacro) {
    let fun = |args: Vector<Type>, scope: &mut Scope| -> Type {
        if let Type::Symbol(sym) = args[0].clone() {
            scope.put(sym, eval(args[1].clone(), &mut scope.clone()));
        
            return Type::Nil;
        } else {
            let name_and_args = &args[0];
            
            let (name, params): (Type, Vector<Type>) = match name_and_args {
                Type::SExpr(sexpr) => (sexpr[0].clone(), sexpr.skip(1)),
                _ => panic!("no name for define"),
            };

            let name = name.as_key();
            let body = args.skip(1);
            let environ = scope.fork();
            let func = Type::Function(Function::new(params, body, environ));

            scope.put(name, func);
        
            return Type::Nil;
        }
    };

    ("define".into(), BuiltinMacro::new("define".into(), fun))
}



pub mod math {
    use super::*;

    pub fn add() -> (String, BuiltinFunction) {
        let fun = |args: Vector<Type>, _scope: &mut Scope| -> Type {
            // println!("add call: {:?}", args);
            
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
        };

        ("add".into(), BuiltinFunction::new("add".into(), fun))
    }

    pub fn mul() -> (String, BuiltinFunction) {
        let fun = |args: Vector<Type>, _scope: &mut Scope| -> Type {
            // println!("mul call: {:?}", args);
            
            let mut mul_int: i64 = 1;
            let mut mul_float: f64 = 1.0;

            for value in args {
                match value {
                    Type::Int(i) => mul_int *= i,
                    Type::Float(f) => mul_float *= f,
                    t => panic!("invalid argument for `mul`: {:?}", t),
                }
            }

            if mul_float > 1.0 {
                Type::Float(mul_int as f64 * mul_float)
            } else {
                Type::Int(mul_int)
            }
        };

        ("mul".into(), BuiltinFunction::new("mul".into(), fun))
    }
}
