use crate::types::{
    Type,
    Scope,
    Call,
};

use im::ConsList;

use std::iter::FromIterator;


pub fn eval(expr: Type, stg: &Scope) -> Type {
    match expr {
        Type::Nil => Type::Nil,
        Type::Quoted(box Type::SExpr(list)) => {
            let list = ConsList::from_iter(list.iter().map(|e| Type::Quoted(Box::new(*e))).map(|s| eval(s, stg)));
            Type::ConsList(list)
        },
        Type::Quoted(box elem) => {
            elem
        },
        Type::Symbol(sym) => stg[&sym].clone(),
        Type::SExpr(sexpr) => {
            let car_eval = eval(*sexpr.head().unwrap(), stg);

            match car_eval {
                Type::Macro(mac) => eval(mac.call(&sexpr.tail().unwrap(), stg), stg),
                Type::Function(fun) => {
                    let args = sexpr.tail().unwrap().iter().map(|v| eval(*v, stg)).collect::<ConsList<Type>>();
                    
                    fun.call(&args, stg)
                }
                _ => {
                    panic!("{:?} is not callable", car_eval)
                }
            }
        }


    };

    Type::Nil
}