
use crate::model::statement::{Statement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};


pub trait ProofStrat {
    fn sub_goals(&self, ex: &CCExpression,
                 context: &[Statement],
                 inner_context: &[Statement],
                 concs: &[Statement],
                 defs: &[Definition]) -> Result<Vec<Goal>, String>;
}

