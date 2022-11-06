
use super::expression::{CCExpression};
use crate::util::{*};

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct Statement {
    pub subject: CCExpression,
    pub s_type: CCExpression
}

impl Statement {

    pub fn subject_in_context(ex: &CCExpression, context: &[Statement]) -> bool {
        context.iter().any(|stmt| stmt.subject.alpha_equiv(ex))
    }

    pub fn ctx_str(context: &[Statement]) -> String {
       context.iter().map(|x| x.to_latex()).collect::<Vec<String>>().join(", ")
    }

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

    pub fn abstractions(ex: &CCExpression) -> Vec<Statement> {
        match ex {
            CCExpression::Application(lhs, rhs) => {
                [Statement::abstractions(&lhs), Statement::abstractions(&rhs)].concat()
            },
            CCExpression::Abs(arg, a_type, ret) => {
                [
                    vec![Statement {subject: CCExpression::Var(arg.to_string()), s_type: *a_type.clone()}],
                    Statement::abstractions(a_type),
                    Statement::abstractions(ret)
                ].concat()
            },
            CCExpression::TypeAbs(arg, a_type, ret) => {
                [
                    vec![Statement {subject: CCExpression::Var(arg.to_string()), s_type: *a_type.clone()}],
                    Statement::abstractions(a_type),
                    Statement::abstractions(ret)
                ].concat()
            },
            _ => vec![]
        }
    }

    pub fn to_latex(&self) -> String {
        return self.subject.to_latex() + " : " + &self.s_type.to_latex()
    }

    pub fn alpha_equiv(&self, rhs: &Statement) -> bool {
        return self.subject.alpha_equiv(&rhs.subject) && self.s_type.alpha_equiv(&rhs.s_type);
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

    #[test]
    fn test_abstractions() {
        let expr1 = CCExpression::Var(String::from("A"));
        let expr2 = CCExpression::Var(String::from("x"));
        let expr3 = CCExpression::Abs("x".to_string(), Box::new(expr1.clone()), Box::new(expr2.clone()));
        let expr4 = CCExpression::TypeAbs("x".to_string(), Box::new(expr1.clone()), Box::new(expr1.clone()));
        let expr5 = CCExpression::Application(Box::new(expr3.clone()), Box::new(expr2.clone()));

        let stmt = Statement {
            subject: expr2.clone(),
            s_type: expr1.clone()
        };

        assert_eq!(Statement::abstractions(&expr1), []);
        assert_eq!(Statement::abstractions(&CCExpression::Star), []);
        assert_eq!(Statement::abstractions(&CCExpression::Sq), []);
        assert_eq!(Statement::abstractions(&expr3), [stmt.clone()]);
        assert_eq!(Statement::abstractions(&expr4), [stmt.clone()]);
        assert_eq!(Statement::abstractions(&expr5), [stmt.clone()]);
    }
}
