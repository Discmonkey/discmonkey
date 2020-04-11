use crate::exec::env::Scope;
use crate::types::ast::LispValue;
use crate::reader::tokenizer::Token;


struct Closure {
    scope: Scope,
    expr: LispValue,
    args: Vec<Token>
}