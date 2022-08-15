
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement, Statement};
use crate::model::rules::base::{DerRule, next_unused_var};


struct VarRule {}

impl DerRule for VarRule {
    fn apply(&self, lhs: Option<Judgement>, rhs: Option<Judgement>) -> Option<Judgement> {
        if let Some(_) = rhs { return None; }
        if let Some(in_judge) = lhs {
            let stmt = &in_judge.statement;
            if let CCExpression::Star = &stmt.s_type {
                if let CCExpression::Var(_) = &stmt.subject {
                    let next = next_unused_var(&in_judge.context);
                    let new_stmt = Statement {
                        s_type: stmt.subject.clone(),
                        subject: CCExpression::Var(next) 
                    };
                    return Some(Judgement {
                        context: [
                            in_judge.context,
                            vec![new_stmt.clone()]].concat(),
                        statement: new_stmt
                    });
                }
            }
        }
        return None;
    }

    fn name(&self) -> String {
        return String::from("var");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_var() {
        let rule = VarRule {};
        let stmt = Statement {
            subject: CCExpression::Var(String::from("A")),
            s_type: CCExpression::Star
        };
        let jdg = Judgement {
            context: vec![],
            statement: stmt
        };
        let output = rule.apply(Some(jdg), None);
        assert_eq!(rule.name(), "var");
        assert_ne!(output, None);
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(x) = output {
            assert_eq!(&x.to_latex(), "a : A \\vdash a : A");
        }
    }
}
