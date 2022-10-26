
use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{PartialSol, Goal, WithConc};
use super::strategy::stratset::{standard_strategy};


fn sub_goals_from_expression(ex: &CCExpression,
                             context: &[Statement],
                             inner_context: &[Statement],
                             concs: &[Judgement],
                             defs: &[Definition]) -> Result<Vec<Goal>, String> {

    let strategies = standard_strategy();

    let output: Vec<Goal> = strategies.iter().map(
        |strat| strat.sub_goals(ex, context, inner_context, concs, defs)
        ).filter_map(
            |block| match block {
                Ok(lst) => Some(lst),
                Err(_) => None
            }).flatten().collect();
    if output.len() > 0 {
        Ok(output)
    } else {
        Err(format!("no strategies returned paths for {}", ex.to_latex()))
    }
}

fn unpack_goal(g1: &WithConc, context: &[Statement],
               defs: &[Definition]) -> Result<(Goal, Vec<Goal>), String> {
    match &g1.goal {
        Goal::Initial(ex, ctx) => {
            let subs = sub_goals_from_expression(&ex, context, &ctx, &g1.conc, defs);
            match subs {
                Ok(lst) => Ok((g1.goal.clone(), lst)),
                Err(x) => Err(x)
            }
        },
        _ => Err(format!("Can only unpack initial, not {}", g1.goal.to_latex()))
    }
}

pub fn next_sol_from_sol(partial: &PartialSol,
                         defs: &[Definition]) -> Result<Vec<PartialSol>, String> {
    let active = partial.active();
    if active.len() == 0 {
        return Err("sol has no path forward".to_string());
    }
    let goal_subs: Vec<(Goal, Vec<Goal>)> = active.iter().filter_map(
            |g| match unpack_goal(g, &partial.context, defs) {
                Ok(x) => Some(x),
                _ => None
            }).collect();
    let output: Vec<PartialSol> = goal_subs.iter().map(
            |(old_g, g_lst)| g_lst.iter().map(
                move |new_g| partial.replace(&old_g, new_g)
                )
            ).flatten().collect();
    return Ok(output);
}

