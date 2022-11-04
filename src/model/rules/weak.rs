
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::rules::base::{DerRule};

pub struct WeakRule {}

impl DerRule for WeakRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        if let Some(judge) = lhs {
            if let Some(t_judge) = rhs {
                if t_judge.statement.s_type.is_sort() {
                    let var = Statement::next_unused_var(&judge.context);
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

    fn validate(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>,
                    result: &Judgement) -> bool {
        if let Some(lex) = lhs {
            if let Some(rex) = rhs {
                if !rex.statement.s_type.is_sort() { return false; }
                if lex.statement != result.statement { return false; }
                if result.context.len() == 0 || result.context.last().unwrap().s_type != rex.statement.subject {
                    return false;
                }
                if !lex.context.iter().all(|stmt| result.context.contains(stmt)) { return false; }
                return true;
            }
        }
        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_weak() {
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

    #[test]
    fn bigger_weak_test() {
        let rule = WeakRule {};
        let stmt0 = Statement {
            subject: CCExpression::Star,
            s_type: CCExpression::Sq
        };
        let stmt1 = Statement {
            subject: CCExpression::Var(String::from("a")),
            s_type: CCExpression::Var(String::from("A"))
        };
        let stmt2 = Statement {
            subject: CCExpression::Var(String::from("C")),
            s_type: CCExpression::Star
        };
        let stmt3 = Statement {
            subject: CCExpression::Var(String::from("A")),
            s_type: CCExpression::Star
        };
        let stmt4 = Statement {
            subject: CCExpression::Var(String::from("B")),
            s_type: CCExpression::Star
        };
        let jdg1 = Judgement {
            defs: vec![],
            context: vec![stmt3.clone(), stmt1.clone()],
            statement: stmt1.clone()
        };
        let jdg2 = Judgement {
            defs: vec![],
            context: vec![stmt3.clone(), stmt1.clone(), stmt2.clone(), stmt4.clone()],
            statement: stmt1.clone()
        };
        let jdg3 = Judgement {
            defs: vec![],
            context: vec![],
            statement: stmt0.clone()
        };

        assert!(rule.validate(Some(&jdg1), Some(&jdg3), &jdg2));
    }
}
