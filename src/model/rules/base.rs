
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement, Statement};


pub fn next_unused_var(context: &[Statement]) -> String {
    let used: Vec<String> = context.iter().filter_map(|stmt| {
        match &stmt.subject {
            CCExpression::Var(x) => Some(x.clone()),
            _ =>  None
        }
    }).collect();
    for ch in 'a'..'z' {
        if !used.contains(&ch.to_string()) {
            return ch.to_string();
        }
    }
    return String::from("x");
}


pub trait DerRule {
    fn apply(&self, lhs: Option<Judgement>, rhs: Option<Judgement>) -> Option<Judgement>;
    fn name(&self) -> String;
}

