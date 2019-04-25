use crate::eval::eval;
use crate::lexer::Token;

use im::Vector;

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;
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

    pub fn as_key(&self) -> String {
        match self {
            Type::Symbol(ref string) => string.clone(),
            _ => panic!("only symbols are keys"),
        }
    }
}

pub trait Call {
    fn call(&self, args: Vector<Type>, stg: &mut Scope) -> Type;
}

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

pub trait BuiltinCall {
    fn call_builtin(&mut self, args: Vector<Type>, scope: &mut Scope) -> Type;
}

impl BuiltinCall for BuiltinFunction {
    fn call_builtin(&mut self, args: Vector<Type>, scope: &mut Scope) -> Type {
        (self.inner)(args, scope)
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

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    params: Vector<Type>,
    body: Vector<Type>,
    scope: Scope,
}

impl Call for Macro {
    fn call(&self, args: Vector<Type>, stg: &mut Scope) -> Type {
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
