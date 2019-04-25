use crate::types::{BuiltinCall, Call, Scope, Type};

use im::Vector;

use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::rc::Rc;

// macro_rules! generate_env {
//     () => {

//     };
// }

pub fn create_env() -> Scope {
    let mut funcs: HashMap<String, Type> = HashMap::new();

    let (add, add_fn) = crate::builtins::math::add();

    funcs.insert(add, Type::Builtin(Rc::new(RefCell::new(add_fn))));

    Scope::new(funcs)
}

pub fn eval(expr: Type, mut stg: &mut Scope) -> Type {
    match expr {
        Type::Nil => Type::Nil,
        Type::Quoted(box Type::SExpr(list)) => {
            let new_list = Vector::from_iter(
                list.into_iter()
                    .map(|e| Type::Quoted(Box::new(e)))
                    .map(|s| eval(s, stg)),
            );
            Type::ConsList(new_list)
        }
        Type::Quoted(box elem) => elem,
        Type::Symbol(sym) => stg[&sym].clone(),
        Type::SExpr(sexpr) => {
            let car_eval = eval(sexpr.head().unwrap().clone(), stg);

            match car_eval {
                Type::Macro(mac) => eval(mac.call(sexpr.skip(1), stg), stg),
                Type::Function(fun) => {
                    let args = sexpr
                        .skip(1)
                        .into_iter()
                        .map(|v| eval(v, stg))
                        .collect::<Vector<Type>>();

                    fun.call(args, stg)
                }
                Type::Builtin(builtin) => {
                    for arg in sexpr.skip(1).iter_mut() {
                        *arg = eval(arg.clone(), stg);
                    }

                    builtin.borrow_mut().call_builtin(sexpr.skip(1), &mut stg)
                }
                _ => panic!("{:?} is not callable", car_eval),
            }
        }
        _ => return expr,
    }
}
