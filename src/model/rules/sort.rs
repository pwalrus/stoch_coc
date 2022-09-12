
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::rules::base::{DerRule};

pub struct SortRule {}

impl DerRule for SortRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        if let Some(_) = lhs { return None; }
        if let Some(_) = rhs { return None; }
        let stmt = Statement {
            subject: CCExpression::Star,
            s_type: CCExpression::Sq
        };
        return Some(Judgement {
            context: vec![],
            statement: stmt
        })
    }

    fn name(&self) -> String {
        return String::from("sort");
    }
    
    fn sig_size(&self) -> u32 { return 0; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_sort() {
        let rule = SortRule {};
        let stmt = Statement {
            subject: CCExpression::Star,
            s_type: CCExpression::Sq
        };
        let output = rule.apply(None, None);
        assert_eq!(rule.name(), "sort");
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(judge) = output {
            assert_eq!(judge.statement, stmt);
        } else {
            panic!();
        }
    }
}
