
use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};


pub trait ProofStrat {
    fn sub_goals(&self, ex: &CCExpression,
                 context: &[Statement],
                 inner_context: &[Statement],
                 concs: &[Judgement],
                 defs: &[Definition]) -> Result<Vec<Goal>, String>;

    fn full_context(&self,
                    context: &[Statement],
                    inner_context: &[Statement],
        ) -> Vec<Statement> {
        context.iter().chain(inner_context).map(|x| x.clone()).collect()
    }

    fn usable_conc(&self,
                   full_context: &[Statement],
                   concs: &[Judgement],
        ) -> Vec<Statement> {
        concs.iter().filter(
            |j| Statement::weaker_eq(&full_context, &j.context)
            ).map(|j| j.statement.clone()).collect()
    }
}

