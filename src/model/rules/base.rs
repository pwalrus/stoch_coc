
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

pub fn next_unused_type(context: &[Statement]) -> String {
    let used_var: Vec<String> = context.iter().filter_map(|stmt| {
        match &stmt.subject {
            CCExpression::Var(x) => Some(x.clone()),
            _ =>  None
        }}).collect();
    let used_type: Vec<String> = context.iter().filter_map(|stmt| {
        match &stmt.s_type {
            CCExpression::Var(x) => Some(x.clone()),
            _ =>  None
        }}).collect();
    let used: Vec<String> = [used_var, used_type].concat();
    for ch in 'A'..'Z' {
        if !used.contains(&ch.to_string()) {
            return ch.to_string();
        }
    }
    return String::from("x");
}

pub trait DerRule {
    fn apply(&self, lhs: Option<Judgement>, rhs: Option<Judgement>) -> Option<Judgement>;
    fn name(&self) -> String;
    fn sig_size(&self) -> u32;
}

