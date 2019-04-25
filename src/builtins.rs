use crate::types::{BuiltinFunction, Scope, Type};

use im::Vector;

pub mod math {
    use super::*;

    pub fn add() -> (String, BuiltinFunction) {
        let fun = |args: Vector<Type>, _scope: &mut Scope| -> Type {
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
}
