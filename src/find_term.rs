
use priority_queue::PriorityQueue;

use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::proof::{Proof};
use crate::model::partial::{Goal, PartialSol};
use crate::unpack_term::{unpack_term};
use crate::type_check::{check_proof};


fn sub_goals_from_var(name: &String,
                      context: &[Statement],
                      inner_context: &[Statement],
                      defs: &[Definition]) -> Result<Vec<Vec<Goal>>, String> {
    let output: Vec<Vec<Goal>> = context.iter().chain(inner_context).filter_map(
        |stmt| if stmt.s_type.var_str() == Some(name.to_string()) {
            Some(stmt.clone())
        } else {
            None
        }
        ).map(|stmt| {
        let jdgs = unpack_term(&stmt.subject, &[context, inner_context].concat(), defs);
        vec![Goal::Final(jdgs)]
    }).collect();

    if output.len() > 0 {
        Ok(output)
    } else {
        Err(format!("no way to make sub goals for: {}", name))
    }
}

fn sub_goals_from_type_abst(var: &String,
                            a_type: &CCExpression,
                            ret: &CCExpression,
                            inner_context: &[Statement]) -> Result<Vec<Vec<Goal>>, String> {
    let new_stmt = Statement {
        subject: CCExpression::Var(var.to_string()),
        s_type: a_type.clone()
    };
    let subs = vec![
        Goal::Initial(
            ret.clone(),
            [inner_context, &[new_stmt]].concat())
    ];
    Ok(vec![subs])
}

fn sub_goals_from_expression(ex: &CCExpression,
                             context: &[Statement],
                             inner_context: &[Statement],
                             defs: &[Definition]) -> Result<Vec<Vec<Goal>>, String> {
        match ex {
            CCExpression::Var(x) => { sub_goals_from_var(&x, context, inner_context, defs) },
            CCExpression::TypeAbs(x, a_type, ret) => { sub_goals_from_type_abst(&x, &a_type, &ret, inner_context) },
            _ => Err(format!("Can not determine subgoals for: {}", ex.to_latex()))
        }
}

fn unpack_goal(g1: &Goal, context: &[Statement],
               defs: &[Definition]) -> Result<(Goal, Vec<Goal>), String> {
    match g1 {
        Goal::Initial(ex, ctx) => {
            let subs = sub_goals_from_expression(&ex, context, &ctx, defs);
            match subs {
                Ok(lst) => Ok((g1.clone(),
                               lst.iter().map(
                                   |sub_lst| Goal::Unpacked(ex.clone(), sub_lst.clone())
                                   ).collect())),
                Err(x) => Err(x)
            }
        },
        _ => Err(format!("Can only unpack initial, not {}", g1.to_latex()))
    }
}

fn next_sol_from_sol(partial: &PartialSol,
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

fn recursive_finalize(partial: &PartialSol,
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

fn do_search(partial: &PartialSol,
             defs: &[Definition]) -> Result<PartialSol, String> {
    let mut queue = PriorityQueue::new();
    queue.push(partial.clone(), 1);
    while !queue.is_empty() {
        let (current, _) = queue.pop().unwrap();
        let new_goals = next_sol_from_sol(&current, defs);
        if let Ok(lst) = new_goals {
            let done_o = lst.iter().find(|x| x.count().i == 0);
            if let Some(done) = done_o {
                let f_done = recursive_finalize(done, defs);
                match f_done {
                    Ok(x) => return Ok(x),
                    Err(x) => return Err(x)
                }
            } else {
                for partial2 in lst {
                    queue.push(partial2.clone(), 1);
                }
            }
        }
    }

    return Err("Search failed".to_string());
}

pub fn find_term(s_type: &CCExpression, context: &[Statement], defs: &[Definition]) -> Result<Proof, String> {
    let g1 = Goal::Initial(s_type.clone(), vec![]);
    let partial = PartialSol{
        context: context.to_vec(),
        goals: vec![g1],
    };
    let res = do_search(&partial, defs);

    match res {
        Ok(out_partial) => {
            let lines_o = out_partial.goals.last().unwrap();
            if let Goal::Final(lines) = lines_o {
                let refs_o = check_proof(&[], lines);
                match refs_o {
                    Ok(refs) => Ok(Proof { lines: lines.clone(), refs: refs }),
                    Err(x) => {
                        Err(x)
                    }
                }
            } else {
                Err(format!("returned goal not final: {:?}", lines_o))
            }
        },
        Err(e) => Err(e)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};

    #[test]
    fn simple_find() {
        let jdg = parse_judgement("A : \\ast, x : A \\vdash x : A").unwrap();
        let t1 = jdg.statement.s_type.clone();
        let stmt1 = jdg.context[0].clone();
        let stmt2 = jdg.context[1].clone();
        let term = find_term(&t1, &[stmt1, stmt2], &[]);

        if let Ok(x) = term {
            assert_eq!(x.lines.last().unwrap().to_latex(), jdg.to_latex());
            let str_lines: Vec<String> = x.lines.iter().map(|x| x.to_latex()).collect();
            assert_eq!(str_lines,
                       ["\\vdash \\ast : \\square",
                       "A : \\ast \\vdash A : \\ast",
                       "A : \\ast, x : A \\vdash x : A"]);
        } else {
            println!("term not found: {:?}", term);
            panic!();
        }
    }

    #[test]
    fn find_on_easy_tautology() {
        let jdg = parse_judgement("A : \\ast \\vdash \\lambda a : A . a : A \\to A").unwrap();
        let t1 = jdg.statement.s_type.clone();
        let stmt1 = jdg.context[0].clone();
        let term = find_term(&t1, &[stmt1], &[]);

        if let Ok(x) = term {
            assert_eq!(x.lines.last().unwrap().to_latex(), jdg.to_latex());
            let str_lines: Vec<String> = x.lines.iter().map(|x| x.to_latex()).collect();
            assert_eq!(str_lines,
                       ["\\vdash \\ast : \\square",
                        "A : \\ast \\vdash A : \\ast",
                        "A : \\ast, a : A \\vdash a : A",
                        "A : \\ast, a : A \\vdash A : \\ast",
                        "A : \\ast \\vdash \\prod a : A . A : \\ast",
                        "A : \\ast \\vdash \\lambda a : A . a : \\prod a : A . A"
                       ]);
        } else {
            println!("term not found: {:?}", term);
            panic!();
        }
    }
}
