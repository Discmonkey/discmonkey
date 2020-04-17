use crate::types::list::List;
use crate::types::ast::{LispValue, build_ast};
use crate::types::env::Scope;
use crate::exec::eval::eval_ast;
use crate::reader::tokenizer::Tokenizer;
use crate::reader::parser::Parser;

pub fn apply_list(list: &List, _env: &mut Scope) -> LispValue {
    let mut copied = List::new();

    list.items().iter().skip(1).for_each(|item| {
        copied.push(item.clone())
    });

    LispValue::List(copied)
}

pub fn apply_eval(list: &List, env: &mut Scope) -> LispValue {
    if list.len() != 2 {
        LispValue::Error("eval can only be called on a single item".to_string())
    } else {
        // why do we need to eval twice?
        // well we first need to evalute the list item
        // then we eval again to evaluate that item as code
        let first_result = eval_ast(&list[1], env);

        let new_env = env.root();

        match new_env {
            None => LispValue::Error("could not find root environment for eval".to_string()),
            Some( mut e) => eval_ast(&first_result, &mut e)
        }
    }
}

pub fn apply_str(list: &List, env: &mut Scope) -> LispValue {

    let mut out = list.items().iter().skip(1).map(|item| {
        let converted = eval_ast(item, env);
        format!("{} ", converted)
    }).fold("".to_string(), |mut total, next | {
        total.push_str(next.as_str());

        total
    });

    out.pop();

    LispValue::String(out)
}

pub fn read_string(tokenizer: &Tokenizer, line: String) -> LispValue {
    let result = tokenizer.tokenize(line);

    match result {
        Err(error) => LispValue::Error(error.to_string()),
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);

            build_ast(&mut parser)
        }
    }
}

pub fn apply_read_string(list: &List, env: &mut Scope) -> LispValue {
    if list.len() != 2 {
        return LispValue::Error("read_string takes a single argument".to_string());
    }

    let second = eval_ast(&list[1], env);

    if let LispValue::String(s) = second {
        let t = Tokenizer::new();
        read_string(&t, s)
    } else {
        LispValue::Error("read string needs a string argument".to_string())
    }
}

pub fn apply_prn(list: &List, env: &mut Scope) -> LispValue {
    if list.len() != 2 {
        LispValue::Error("prn takes a single argument".to_string())
    } else {
        println!("{}", eval_ast(&list[1], env));

        LispValue::Nil
    }
}