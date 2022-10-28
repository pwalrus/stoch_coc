

use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal, PartialSol};


fn final_goal_from_subs(inst: &CCExpression, ex: &CCExpression,
                        context: &[Statement],
                        incontext: &[Statement], subs: &[Goal],
                        _: &[Definition]) -> Result<Goal, String> {
    let last_ex: Vec<&Judgement> = subs.iter().filter_map(
        |g| match g {
            Goal::Final(jdgs) => jdgs.last(),
            _ => None
        }).collect();
    if last_ex.len() != subs.len() {
        return Err("Not all goals were finalized.".to_string());
    }
    let mut term = inst.clone();

    for (idx, ex) in last_ex.iter().enumerate() {
        term = term.substitute(&format!("sub_{{{}}}", idx), &ex.statement.subject);
    }
    let jdgs: Vec<Judgement> = subs.iter().filter_map(
        |g| match g {
            Goal::Final(x) => Some(x.to_vec()),
            _ => None
        }).flatten().collect();
    let last_line = Judgement {
        defs: jdgs.last().unwrap().defs.to_vec(),
        context: [context, incontext].concat(),
        statement: Statement {
            subject: term,
            s_type: ex.clone()
        }
    };

    return Ok(Goal::Final([jdgs, vec![last_line]].concat()));
}

fn recursive_finalize_g(g1: &Goal, context: &[Statement],
             defs: &[Definition]) -> Result<Goal, String> {
    match g1 {
        Goal::Final(jdgs) => Ok(Goal::Final(jdgs.to_vec())),
        Goal::Initial(_, _) => Err(format!("cannot finalize initial: {}", g1.to_latex())),
        Goal::Unpacked(inst, ex, subs, incontext) => {
            let rec: Vec<Result<Goal, String>> = subs.iter().map(
                |g2| recursive_finalize_g(g2, context, defs)
                ).collect();
            let err: Option<&String> = rec.iter().find_map(
                |r| if let Err(msg) = r { Some(msg) } else { None });
            if let Some(msg) = err {
                return Err(msg.to_string());
            }
            let new_subs: Vec<Goal> = rec.iter().map(|x| x.clone().unwrap()).collect();

            final_goal_from_subs(&inst, &ex,
                                 context, incontext,
                                 &new_subs,
                                 defs)
        }
    }
}

pub fn recursive_finalize(partial: &PartialSol,
             defs: &[Definition]) -> Result<PartialSol, String> {
    let out_goals: Vec<Result<Goal, String>> = partial.goals.iter().map(
        |g| recursive_finalize_g(g, &partial.context, defs)
        ).collect();
    let err = out_goals.iter().find_map(
        |r| if let Err(msg) = r { Some(msg) } else { None });
    if let Some(msg) = err {
        return Err(msg.to_string());
    }
    Ok(PartialSol {
        context: partial.context.to_vec(),
        goals: out_goals.iter().map(|x| x.clone().unwrap()).collect()
    })
}

