use crate::lexer::Token;

use im::conslist::ConsList;
pub use im::conslist::cons;

use std::ops::Index;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Float(f64),
    Int(i64),
    StrLit(String),
    Symbol(String),
    Cons(ConsList<Type>),
    ConsList(ConsList<Type>),
    SExpr(ConsList<Type>),
    Quoted(Box<Type>),
    Function(Function),
    Macro(Macro),
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
}

pub trait Call {
    fn call(&self, args: &ConsList<Type>, stg: &Scope) -> Type;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    params: ConsList<Type>,
    body: ConsList<Type>,
    scope: Scope
}

impl Call for Function {
    fn call(&self, args: &ConsList<Type>, stg: &Scope) -> Type {
        Type::Nil
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    params: ConsList<Type>,
    body: ConsList<Type>,
    scope: Scope
}

impl Call for Macro {
    fn call(&self, args: &ConsList<Type>, stg: &Scope) -> Type {
        Type::Nil
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

    pub fn fork(&self) -> Scope {
        let mut environ = self.environ.clone();
        environ.extend(self.local.clone());

        Scope::new(environ)
    }
}

impl Index<&String> for Scope
{
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

