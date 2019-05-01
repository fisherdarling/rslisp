use crate::functions::{BuiltinFunction, BuiltinMacro, Function};
use crate::lexer::Token;

use im::Vector;

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Float(f64),
    Int(i64),
    StrLit(String),
    Symbol(String),
    Cons(Vector<Type>),
    ConsList(Vector<Type>),
    SExpr(Vector<Type>),
    Quoted(Box<Type>),
    Function(Function),
    Builtin(Rc<RefCell<BuiltinFunction>>),
    Macro(Rc<RefCell<BuiltinMacro>>),
    Nil,
}

impl Type {
    pub fn from_tok<'a>(token: Token<'a>) -> Type {
        match token {
            Token::Float(flo) => Type::Float(flo.parse().unwrap()),
            Token::Int(int) => Type::Int(int.parse().unwrap()),
            Token::StrLit(lit) => Type::StrLit(lit.into()),
            Token::Symbol(sym) => Type::Symbol(sym.into()),
            _ => panic!("cannot convert from {:?} to a Type", token),
        }
    }

    pub fn as_key(&self) -> String {
        match self {
            Type::Symbol(ref string) => string.clone(),
            _ => panic!("only symbols are keys"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    environ: HashMap<String, Type>,
    local: HashMap<String, Type>,
}

impl Scope {
    pub fn new(environ: HashMap<String, Type>) -> Scope {
        Scope {
            environ,
            local: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: Type) {
        self.local.insert(key, value);
    }

    pub fn fork(&self) -> HashMap<String, Type> {
        let mut environ = self.environ.clone();
        environ.extend(self.local.clone());

        environ
    }
}

impl Index<&String> for Scope {
    type Output = Type;

    #[inline]
    fn index(&self, key: &String) -> &Type {
        if self.local.contains_key(key) {
            self.local.get(key).unwrap()
        } else if self.environ.contains_key(key) {
            self.environ.get(key).unwrap()
        } else {
            panic!("key not found: {}", key)
        }
    }
}
