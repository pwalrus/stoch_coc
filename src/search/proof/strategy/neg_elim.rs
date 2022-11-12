
use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};
use super::base::{ProofStrat};

pub struct NegElim {}

fn find_matches(context: &[Statement]) -> Vec<(Statement, Statement)> {
    context.iter().filter_map(
        |stmt_neg| {
            if let Some(t1) = stmt_neg.s_type.is_neg() {
                Some(context.iter().filter_map(
                    |stmt_base| if stmt_base.s_type.alpha_equiv(&t1) {
                        Some((stmt_neg.clone(), stmt_base.clone()))
                    } else {
                        None
                    }
                    ).collect::<Vec<(Statement, Statement)>>())
            } else {
                None
            }
        }
        ).flatten().collect()
}


fn make_goal(stmt_neg: &Statement,
             stmt_base: &Statement,
             new_type: &CCExpression,
             context: &[Statement],
             inner_context: &[Statement],
             ) -> Goal {
    let new_term1 = CCExpression::Application(
        Box::new(stmt_neg.subject.clone()),
        Box::new(stmt_base.subject.clone()));
    let new_term2 = CCExpression::Application(
        Box::new(new_term1),
        Box::new(new_type.clone()));
    let new_stmt = Statement {
        subject: new_term2,
        s_type: new_type.clone()
    };
    let g_fin = Goal::Final(
        vec![Judgement {
            defs: vec![],
            context: [context, inner_context].concat(),
            statement: new_stmt
        }]);
    g_fin
}


impl ProofStrat for NegElim {
    fn sub_goals(&self, ex: &CCExpression,
                 context: &[Statement],
                 inner_context: &[Statement],
                 concs: &[Judgement],
                 _: &[Definition]) -> Result<Vec<Goal>, String> {

        let full_context: Vec<Statement> = self.full_context(context, inner_context);
        let usable_conc: Vec<Statement> = self.usable_conc(&full_context, concs);
        let all_known: Vec<Statement> = [full_context, usable_conc].concat();
        let matches = find_matches(&all_known);
        let goal_o: Option<Goal> = matches.iter().map(
            |(stmt_neg, stmt_base)|
                make_goal(stmt_neg, stmt_base, ex, context, inner_context)
            ).next();

        match goal_o {
            Some(goal) => Ok(vec![goal]),
            None => Err("No appropriate negations to apply".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};

    #[test]
    fn test_neg_elim_strat() {
        let jdg: Judgement = parse_judgement(
            "A:\\ast,  B:\\ast, x: A, y:\\neg A \\vdash z: B"
            ).unwrap();
        let strat = NegElim {};
        let ex = &jdg.statement.s_type;
        let context = &jdg.context;
        let res = strat.sub_goals(ex, context, &[], &[], &[]);

        match res {
            Ok(lst) => {
                assert_eq!(lst.len(), 1);
                if let Goal::Final(jdgs) = &lst[0] {
                    assert_eq!(jdgs.len(), 1);
                    assert_eq!(jdgs[0].statement.to_latex(), "y x B : B");
                } else { panic!(); }
            },
            Err(_) => { panic!(); }
        }
    }
}
