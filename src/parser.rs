use crate::lexer::{Lexer, Token};
use crate::types::Type;
use im::Vector;

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
enum ValOrCtrl {
    LParen,
    RParen,
    Quote,
    Val(Type),
}

/// Pop an sexpression from the top of the stack
fn pop_sexpr(stack: &mut LinkedList<ValOrCtrl>) -> Type {
    stack.pop_back();
    let mut tokens: LinkedList<Type> = LinkedList::new();

    let mut top = stack.back();

    while let Some(ValOrCtrl::Val(val)) = top {
        tokens.push_front(val.clone());

        stack.pop_back();
        top = stack.back();
    }

    if stack.is_empty() {
        panic!("Too many right parens");
    }

    stack.pop_back();

    if tokens.len() == 0 {
        return Type::Nil;
    }

    let list = Vector::from_iter(tokens.into_iter());

    Type::SExpr(list)
}

/// Handle any number of quotes
fn handle_quotes(stack: &mut LinkedList<ValOrCtrl>, sexpr: Type) -> Type {
    let mut sexpr = sexpr;

    if stack.is_empty() {
        return sexpr;
    }

    let mut top = stack.back();

    while let Some(ValOrCtrl::Quote) = top {
        sexpr = Type::Quoted(Box::new(sexpr));

        stack.pop_back();
        top = stack.back();
    }

    sexpr
}

pub fn parse<'a>(lex: &'a mut Lexer<'a>) -> impl Iterator<Item = Type> + 'a {
    let mut stack: LinkedList<ValOrCtrl> = LinkedList::new();
    let mut paren_count = 0;

    std::iter::from_fn(move || loop {
        if let Some(tok) = lex.next() {
            match tok {
                Token::LParen => {
                    paren_count += 1;
                    stack.push_back(ValOrCtrl::LParen);
                }
                Token::Quote => stack.push_back(ValOrCtrl::Quote),
                Token::RParen => {
                    paren_count -= 1;

                    if paren_count < 0 {
                        panic!("too many closing parens");
                    }

                    if let Some(ValOrCtrl::Quote) = stack.back() {
                        panic!("invalid quotation");
                    }

                    stack.push_back(ValOrCtrl::RParen);

                    let mut new_sexpr = pop_sexpr(&mut stack);

                    if stack.is_empty() {
                        return Some(new_sexpr);
                    }

                    new_sexpr = handle_quotes(&mut stack, new_sexpr);

                    if stack.is_empty() {
                        return Some(new_sexpr);
                    }

                    stack.push_back(ValOrCtrl::Val(new_sexpr));
                }
                Token::Comment(_) => {}
                _ => {
                    let mut val = Type::from_tok(tok);
                    val = handle_quotes(&mut stack, val);

                    if stack.is_empty() {
                        return Some(val);
                    }

                    stack.push_back(ValOrCtrl::Val(val));
                }
            }
        } else {
            return None;
        }
    })
}
