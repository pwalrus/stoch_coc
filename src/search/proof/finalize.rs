

use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal, PartialSol};


fn final_goal_from_var(name: &String, subs: &[Goal],
                       _: &[Definition]) -> Result<Goal, String> {
    let term = subs.iter().find(
        |g| if let Goal::Final(jdgs) = g {
            jdgs.len() > 0 && jdgs.last().unwrap().statement.s_type.var_str() == Some(name.to_string())
        } else { false });
    match term {
        Some(g) => Ok(g.clone()),
        None => Err(format!("var: No subgoal matches {}", name))
    }
}

fn final_goal_from_type_abs(name: &String,
                            a_type: &CCExpression,
                            ret: &CCExpression,
                            subs: &[Goal],
                            _: &[Definition]) -> Result<Goal, String> {
    let lines = subs.iter().find_map(
        |g| if let Goal::Final(jdgs) = g {
            if jdgs.len() > 0 && jdgs.last().unwrap().statement.s_type == *ret{
                Some(jdgs)
            } else { None }
        } else { None });

    match lines {
        Some(jdgs) => {
            let last = jdgs.last().unwrap();
            let stmt1 = Statement {
                subject: CCExpression::Abs( name.to_string(), Box::new(a_type.clone()), Box::new(last.statement.subject.clone())),
                s_type: CCExpression::TypeAbs( name.to_string(), Box::new(a_type.clone()), Box::new(ret.clone()))
            };
            let stmt2 = Statement {
                subject: stmt1.s_type.clone(),
                s_type: CCExpression::Star
            };
            let stmt3 = Statement {
                subject: ret.clone(),
                s_type: CCExpression::Star
            };
            let output = Goal::Final(
                [jdgs.clone(), vec![Judgement{
                    defs: last.defs.clone(),
                    context: last.context.to_vec(),
                    statement: stmt3
                }, Judgement{
                    defs: last.defs.clone(),
                    context: last.context[..last.context.len() - 1].to_vec(),
                    statement: stmt2
                }, Judgement{
                    defs: last.defs.clone(),
                    context: last.context[..last.context.len() - 1].to_vec(),
                    statement: stmt1
                }]].concat());
            Ok(output)
        },
        None => Err(format!("type_abs: No subgoal matches {}", ret.to_latex()))
    }
}

fn final_goal_from_subs(ex: &CCExpression, subs: &[Goal],
                        defs: &[Definition]) -> Result<Goal, String> {
    match ex {
        CCExpression::Var(x) => final_goal_from_var(x, subs, defs),
        CCExpression::TypeAbs(x, a_type, ret) => final_goal_from_type_abs(x, &a_type, &ret, subs, defs),
        _ => Err(format!("failed to find final for {}", ex.to_latex()))
    }

}

fn recursive_finalize_g(g1: &Goal, context: &[Statement],
             defs: &[Definition]) -> Result<Goal, String> {
    match g1 {
        Goal::Final(jdgs) => Ok(Goal::Final(jdgs.to_vec())),
        Goal::Initial(_, _) => Err(format!("cannot finalize initial: {}", g1.to_latex())),
        Goal::Unpacked(ex, subs) => {
            let rec: Vec<Result<Goal, String>> = subs.iter().map(
                |g2| recursive_finalize_g(g2, context, defs)
                ).collect();
            let err: Option<&String> = rec.iter().find_map(
                |r| if let Err(msg) = r { Some(msg) } else { None });
            if let Some(msg) = err {
                return Err(msg.to_string());
            }
            let new_subs: Vec<Goal> = rec.iter().map(|x| x.clone().unwrap()).collect();

            final_goal_from_subs(&ex, &new_subs, defs)
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

