use crate::reader::tokenizer::Token;
use crate::types::list::List;
use crate::exec::eval::{eval_ast};
use std::rc::Rc;
use std::borrow::BorrowMut;
use crate::types::env::Scope;
use crate::types::ast::{LispValue, Lambda};

pub fn create_closure(tokens: Vec<Token>, expr: LispValue)
    -> Lambda {

    return Rc::new(move |args: &List, env: &mut Scope| {

        let mut function_scope = env.new_scope();

        args.items()
            .iter()
            .skip(1)
            .map(|x| {eval_ast(x, env)})
            .enumerate()
            .for_each(|(i, result) | {
                function_scope.borrow_mut().set(tokens[i].get_text().clone(), result)
            });

        eval_ast(&expr, &mut function_scope)
    })
}