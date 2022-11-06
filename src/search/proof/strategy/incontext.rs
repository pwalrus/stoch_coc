
use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};
use super::base::{ProofStrat};

pub struct InContext {}

impl ProofStrat for InContext {
    fn sub_goals(&self, ex: &CCExpression,
                 context: &[Statement],
                 inner_context: &[Statement],
                 _: &[Judgement],
                 _: &[Definition]) -> Result<Vec<Goal>, String> {

        let output: Vec<Result<Goal, String>> = context.iter().chain(inner_context).filter_map(
            |stmt| if stmt.s_type == *ex {
                Some(stmt.clone())
            } else {
                None
            }
            ).map(|stmt| {
            let jdgs = vec![
                Judgement {
                    statement: stmt.clone(),
                    context: [context, inner_context].concat(),
                    defs: vec![]
                }
            ];
            Ok(Goal::Final(jdgs))
        }).collect();

        if output.len() > 0 && !output.iter().any(|x| x.is_err()) {
            Ok(output.iter().map(|x| x.as_ref().unwrap().clone()).collect())
        } else {
            Err(format!("not in context: {}", ex.to_latex()))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};

    #[test]
    fn test_in_context_strat() {
        let jdg: Judgement = parse_judgement("A:\\ast, x:A \\vdash y : A").unwrap();
        let strat = InContext {};
        let ex = &jdg.statement.s_type;
        let context = &jdg.context;
        let res = strat.sub_goals(ex, context, &[], &[], &[]);

        match res {
            Ok(lst) => {
                assert_eq!(lst.len(), 1);
                if let Goal::Final(jdgs) = &lst[0] {
                    assert_eq!(jdgs.last().unwrap().statement.to_latex(), "x : A");
                } else { panic!(); }
            },
            Err(_) => { panic!(); }
        }
    }
}

