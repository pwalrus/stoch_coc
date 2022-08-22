
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement, Statement};
use crate::model::rules::base::{DerRule};

pub struct ConvRule {}

impl DerRule for ConvRule {
    fn apply(&self, lhs: Option<Judgement>, rhs: Option<Judgement>) -> Option<Judgement> {
        if let Some(orig_judge) = lhs {
            if let Some(other_judge) = rhs {
                if !other_judge.statement.s_type.is_sort() { return None; }
                if orig_judge.statement.s_type == other_judge.statement.subject { return None; }
                if !orig_judge.statement.s_type.beta_equiv(&other_judge.statement.subject) { return None; }

                let stmt = Statement {
                    subject: orig_judge.statement.subject,
                    s_type: other_judge.statement.subject
                };
                return Some(Judgement {
                    context: orig_judge.context.clone(),
                    statement: stmt
                })
            }
        }
        return None;
    }

    fn name(&self) -> String {
        return String::from("conv");
    }
    
    fn sig_size(&self) -> u32 { return 2; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_conv() {
        let t1 = CCExpression::TypeAbs(
            String::from("x"),
            Box::new(CCExpression::Var(String::from("A"))),
            Box::new(CCExpression::Var(String::from("x")))
            );
        let t2 = CCExpression::TypeAbs(
            String::from("y"),
            Box::new(CCExpression::Var(String::from("A"))),
            Box::new(CCExpression::Var(String::from("y")))
            );
        let rule = ConvRule {};
        let stmt1 = Statement {
            subject: CCExpression::Var(String::from("x")),
            s_type: t1.clone()
        };
        let judg1 = Judgement {
            context: vec![],
            statement: stmt1
        };
        let stmt2 = Statement {
            subject: t2.clone(),
            s_type: CCExpression::Star
        };
        let judg2 = Judgement {
            context: vec![],
            statement: stmt2
        };
        let stmt3 = Statement {
            subject: CCExpression::Var(String::from("x")),
            s_type: t2.clone()
        };
        let judg3 = Judgement {
            context: vec![],
            statement: stmt3
        };

        assert_eq!(judg1.to_latex(), "\\vdash x : \\prod x : A . x");
        assert_eq!(judg2.to_latex(), "\\vdash \\prod y : A . y : \\ast");
        assert_eq!(judg3.to_latex(), "\\vdash x : \\prod y : A . y");
        let output = rule.apply(Some(judg1), Some(judg2));
        assert_eq!(rule.name(), "conv");
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(judge) = output {
            assert_eq!(judge.to_latex(), judg3.to_latex());
        } else {
            panic!();
        }
    }
}
