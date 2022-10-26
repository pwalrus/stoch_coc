
use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};
use crate::unpack_term::{unpack_term};
use super::base::{ProofStrat};

pub struct InContext {}

impl ProofStrat for InContext {
    fn sub_goals(&self, ex: &CCExpression,
                 context: &[Statement],
                 inner_context: &[Statement],
                 _: &[Judgement],
                 defs: &[Definition]) -> Result<Vec<Goal>, String> {

        let output: Vec<Goal> = context.iter().chain(inner_context).filter_map(
            |stmt| if stmt.s_type == *ex {
                Some(stmt.clone())
            } else {
                None
            }
            ).map(|stmt| {
            let jdgs = unpack_term(&stmt.subject, &[context, inner_context].concat(), defs);
            Goal::Final(jdgs)
        }).collect();

        if output.len() > 0 {
            Ok(output)
        } else {
            Err(format!("not in context: {}", ex.to_latex()))
        }
    }
}

