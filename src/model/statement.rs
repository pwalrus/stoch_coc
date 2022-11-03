
use super::expression::{CCExpression};
use crate::util::{*};

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct Statement {
    pub subject: CCExpression,
    pub s_type: CCExpression
}

impl Statement {

    pub fn next_unused_var(context: &[Statement]) -> String {
        let used: Vec<String> = context.iter().filter_map(|stmt| {
            match &stmt.subject {
                CCExpression::Var(x) => Some(x.clone()),
                _ =>  None
            }
        }).collect();
        next_unused_var(&used)
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
        next_unused_cap_var(&used)
    }

    pub fn to_latex(&self) -> String {
        return self.subject.to_latex() + " : " + &self.s_type.to_latex()
    }

    pub fn alpha_equiv(&self, rhs: &Statement) -> bool {
        return self.subject.alpha_equiv(&rhs.subject);
    }

    pub fn primative(&self) -> bool {
        return self.subject.primative();
    }

    pub fn weaker_eq(lhs: &[Statement], rhs: &[Statement]) -> bool {
        return rhs.iter().all(|x| lhs.contains(x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_latex_simple_stmt() {
        let expr1 = CCExpression::Var(String::from("banana"));
        let expr2 = CCExpression::Var(String::from("A"));
        let stmt = Statement { subject: expr1, s_type: expr2 };
        assert_eq!(stmt.to_latex(), String::from("banana : A"));
        assert_eq!(stmt.primative(), false);
    }
}
