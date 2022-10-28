
use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};
use super::base::{ProofStrat};

pub struct UnpackTypeAbs {}

impl ProofStrat for UnpackTypeAbs {
    fn sub_goals(&self, ex: &CCExpression,
                 _: &[Statement],
                 inner_context: &[Statement],
                 _: &[Judgement],
                 _: &[Definition]) -> Result<Vec<Goal>, String> {
        if let CCExpression::TypeAbs(var, a_type, ret) = ex {
            let new_stmt = Statement {
                subject: CCExpression::Var(var.to_string()),
                s_type: *a_type.clone()
            };
            let subs = vec![
                Goal::Initial(
                    *ret.clone(),
                    [inner_context, &[new_stmt]].concat())
            ];
            Ok(vec![Goal::Unpacked(CCExpression::Abs(
                                       var.to_string(),
                                       Box::new(*a_type.clone()),
                                       Box::new(CCExpression::Var("sub_{0}".to_string()))),
                                   CCExpression::TypeAbs(
                                       var.to_string(),
                                       Box::new(*a_type.clone()),
                                       Box::new(*ret.clone())),
                                       subs,
                                       inner_context.to_vec())])
        } else {
            Err(format!("Not a type abstraction: {}", ex.to_latex()))
        }
    }
}

