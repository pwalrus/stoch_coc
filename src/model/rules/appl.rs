
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::rules::base::{DerRule};

pub struct ApplRule {}

impl DerRule for ApplRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        if let Some(f_jdg) = lhs {
            if let Some(a_jdg) = rhs {
                if let CCExpression::TypeAbs(ph, a_type, r_type) =
                    &f_jdg.statement.s_type {
                    if !a_type.alpha_equiv(&a_jdg.statement.s_type) {
                        return None;
                    }
                    let stmt = Statement {
                        subject: CCExpression::Application(
                                     Box::new(f_jdg.statement.subject.clone()),
                                     Box::new(a_jdg.statement.subject.clone())
                                     ),
                        s_type: r_type.substitute(ph, &a_jdg.statement.subject)
                    };
                    return Some(Judgement {
                        defs: f_jdg.defs.clone(),
                        context: f_jdg.context.clone(),
                        statement: stmt
                    });
                }
            }
        }
        return None;
    }

    fn name(&self) -> String {
        return String::from("appl");
    }

    fn sig_size(&self) -> u32 { return 2; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_appl() {
        let rule = ApplRule {};
        let stmt1 = Statement {
            subject: CCExpression::Var(String::from("M")),
            s_type: CCExpression::TypeAbs(
                String::from("x"),
                Box::new(CCExpression::Var(String::from("A"))),
                Box::new(CCExpression::Var(String::from("B")))
                )
        };
        let jdg1 = Judgement {
            defs: vec![],
            statement: stmt1,
            context: vec![]
        };
        let stmt2 = Statement {
            subject: CCExpression::Var(String::from("N")),
            s_type: CCExpression::Var(String::from("A"))
        };
        let jdg2 = Judgement {
            defs: vec![],
            statement: stmt2,
            context: vec![]
        };
        let stmt3 = Statement {
            subject: CCExpression::Application(
                Box::new(CCExpression::Var(String::from("M"))),
                Box::new(CCExpression::Var(String::from("N")))
             ),
            s_type: CCExpression::Var(String::from("B"))
        };
        let jdg3 = Judgement {
            defs: vec![],
            statement: stmt3,
            context: vec![]
        };
        assert_eq!(&jdg1.to_latex(), "\\vdash M : A \\to B");
        assert_eq!(&jdg2.to_latex(), "\\vdash N : A");
        let output = rule.apply(Some(&jdg1), Some(&jdg2));
        assert_eq!(rule.name(), "appl");
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(judge) = output {
            assert_eq!(judge.to_latex(), jdg3.to_latex());
        } else {
            panic!();
        }
    }
}
