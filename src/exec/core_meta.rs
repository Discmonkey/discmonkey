use crate::types::list::List;
use crate::types::ast::LispValue;
use crate::types::env::Scope;

pub fn apply_quote(list: &List, _env: &mut Scope) -> LispValue {
    if list.len() > 2 {
        LispValue::Error("quoting is only supported for a single argument".to_string())
    } else {
        list[1].clone()
    }
}