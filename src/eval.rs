use crate::types::{Scope, Type};
use rslisp_functions::functions::{BuiltinFunction, BuiltinMacro};

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
    let (mul, mul_fn) = crate::builtins::math::mul();
    let (define, define_fn) = crate::builtins::define();

    let add_fn = Type::Builtin(Rc::new(RefCell::new(add_fn)));
    let mul_fn = Type::Builtin(Rc::new(RefCell::new(mul_fn)));
    let define_fn = Type::Macro(Rc::new(RefCell::new(define_fn)));

    funcs.insert(define, define_fn);
    funcs.insert(add, add_fn.clone());
    funcs.insert(mul, mul_fn.clone());
    funcs.insert("+".into(), add_fn);
    funcs.insert("*".into(), mul_fn);

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
                Type::Macro(mac) => eval(mac.borrow_mut().call_builtin(sexpr.skip(1), stg), stg),
                Type::Function(fun) => {
                    let args = sexpr
                        .skip(1)
                        .into_iter()
                        .map(|v| eval(v, stg))
                        .collect::<Vector<Type>>();

                    fun.call(args, stg)
                }
                Type::Builtin(builtin) => {
                    // println!("BUILTIN: {:?} {:?}", sexpr.take(1), sexpr.skip(1));

                    let mut args = sexpr.skip(1);

                    for arg in args.iter_mut() {
                        *arg = eval(arg.clone(), stg);

                        // println!("New arg: {:?}", *arg);
                    }

                    // println!("FINAL ARGS: {:?}", args);

                    builtin.borrow_mut().call_builtin(args, &mut stg)
                }
                _ => panic!("{:?} is not callable", car_eval),
            }
        }
        _ => return expr,
    }
}


pub trait Call {
    fn call(&self, args: Vector<Type>, stg: &mut Scope) -> Type;
}

pub trait BuiltinCall {
    fn call_builtin(&mut self, args: Vector<Type>, scope: &mut Scope) -> Type;
}

impl Call for Function {
    fn call(&self, args: Vector<Type>, stg: &mut Scope) -> Type {
        if args.len() != self.params.len() {
            panic!("invalid number of arguments");
        }

        let mut bounded_storage = Scope::new(self.environ.clone());

        for (key, value) in self.params.iter().zip(args.into_iter()) {
            bounded_storage.put(key.as_key(), value);
        }

        let mut values: Vec<Type> = Vec::new();

        for expr in &self.body {
            values.push(eval(expr.clone(), &mut bounded_storage));
        }

        values.last().cloned().unwrap_or(Type::Nil)
    }
}

impl BuiltinCall for BuiltinFunction {
    fn call_builtin(&mut self, args: Vector<Type>, scope: &mut Scope) -> Type {
        (self.inner)(args, scope)
    }
}


impl BuiltinCall for BuiltinMacro {
    fn call_builtin(&mut self, args: Vector<Type>, scope: &mut Scope) -> Type {
        (self.inner)(args, scope)
    }
}
