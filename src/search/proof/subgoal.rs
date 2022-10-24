
use crate::model::statement::{Statement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};
use crate::unpack_term::{unpack_term};


fn sub_goals_from_var(name: &String,
                      context: &[Statement],
                      inner_context: &[Statement],
                      defs: &[Definition]) -> Result<Vec<Goal>, String> {
    let output: Vec<Goal> = context.iter().chain(inner_context).filter_map(
        |stmt| if stmt.s_type.var_str() == Some(name.to_string()) {
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
        Err(format!("no way to make sub goals for: {}", name))
    }
}

fn sub_goals_from_type_abst(var: &String,
                            a_type: &CCExpression,
                            ret: &CCExpression,
                            inner_context: &[Statement]) -> Result<Vec<Goal>, String> {
    let new_stmt = Statement {
        subject: CCExpression::Var(var.to_string()),
        s_type: a_type.clone()
    };
    let subs = vec![
        Goal::Initial(
            ret.clone(),
            [inner_context, &[new_stmt]].concat())
    ];
    Ok(vec![Goal::Unpacked(CCExpression::Star,
                           CCExpression::TypeAbs(
                               var.to_string(),
                               Box::new(a_type.clone()),
                               Box::new(ret.clone())
                               ),
                           subs)])
}

fn sub_goals_from_expression(ex: &CCExpression,
                             context: &[Statement],
                             inner_context: &[Statement],
                             defs: &[Definition]) -> Result<Vec<Goal>, String> {
        match ex {
            CCExpression::Var(x) => { sub_goals_from_var(&x, context, inner_context, defs) },
            CCExpression::TypeAbs(x, a_type, ret) => { sub_goals_from_type_abst(&x, &a_type, &ret, inner_context) },
            _ => Err(format!("Can not determine subgoals for: {}", ex.to_latex()))
        }
}

pub fn unpack_goal(g1: &Goal, context: &[Statement],
               defs: &[Definition]) -> Result<(Goal, Vec<Goal>), String> {
    match g1 {
        Goal::Initial(ex, ctx) => {
            let subs = sub_goals_from_expression(&ex, context, &ctx, defs);
            match subs {
                Ok(lst) => Ok((g1.clone(), lst)),
                Err(x) => Err(x)
            }
        },
        _ => Err(format!("Can only unpack initial, not {}", g1.to_latex()))
    }
}

