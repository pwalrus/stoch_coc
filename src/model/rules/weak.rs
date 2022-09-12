
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::rules::base::{DerRule, next_unused_var};

pub struct WeakRule {}

impl DerRule for WeakRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        if let Some(judge) = lhs {
            if let Some(t_judge) = rhs {
                if t_judge.statement.s_type.is_sort() {
                    let var = next_unused_var(&judge.context);
                    let stmt = Statement {
                        subject: CCExpression::Var(var),
                        s_type: t_judge.statement.subject.clone()
                    };
                    return Some(Judgement {
                        defs: judge.defs.clone(),
                        context: [judge.context.clone(), vec![stmt]].concat(),
                        statement: judge.statement.clone()
                    });
                }
            }
        }
        return None;
    }

    fn name(&self) -> String {
        return String::from("weak");
    }
    
    fn sig_size(&self) -> u32 { return 2; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_sort() {
        let rule = WeakRule {};
        let stmt1 = Statement {
            subject: CCExpression::Var(String::from("A")),
            s_type: CCExpression::Var(String::from("B"))
        };
        let stmt2 = Statement {
            subject: CCExpression::Var(String::from("C")),
            s_type: CCExpression::Star
        };
        let output = rule.apply(Some(&Judgement {
            defs: vec![],
            context: vec![],
            statement: stmt1.clone()
        }), Some(&Judgement {
            defs: vec![],
            context: vec![],
            statement: stmt2
        }));
        assert_eq!(rule.name(), "weak");
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(judge) = output {
            assert_eq!(judge.statement, stmt1);
            assert_eq!(judge.to_latex(), "a : C \\vdash A : B");
        } else {
            panic!();
        }
    }
}
