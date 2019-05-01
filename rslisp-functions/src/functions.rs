use rslisp::types::{Scope, Type};

use im::Vector;

use std::collections::HashMap;
use std::fmt;

pub struct BuiltinFunction {
    name: String,
    inner: Box<dyn FnMut(Vector<Type>, &mut Scope) -> Type>,
}

impl BuiltinFunction {
    pub fn new(
        name: String,
        fun: impl FnMut(Vector<Type>, &mut Scope) -> Type + 'static,
    ) -> BuiltinFunction {
        BuiltinFunction {
            name,
            inner: Box::new(fun),
        }
    }
}


impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BuiltinFunction: {}", self.name)
    }
}

impl PartialEq for BuiltinFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    params: Vector<Type>,
    body: Vector<Type>,
    environ: HashMap<String, Type>,
}

impl Function {
    pub fn new(
        params: Vector<Type>,
        body: Vector<Type>,
        environ: HashMap<String, Type>,
    ) -> Function {
        Function {
            params,
            body,
            environ,
        }
    }
}



pub struct BuiltinMacro {
    name: String,
    inner: Box<dyn FnMut(Vector<Type>, &mut Scope) -> Type>,
}

impl BuiltinMacro {
    pub fn new(
        name: String,
        fun: impl FnMut(Vector<Type>, &mut Scope) -> Type + 'static,
    ) -> BuiltinMacro {
        BuiltinMacro {
            name,
            inner: Box::new(fun),
        }
    }
}

impl fmt::Debug for BuiltinMacro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BuiltinMacro `{}`", self.name)
    }
}

impl PartialEq for BuiltinMacro {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
