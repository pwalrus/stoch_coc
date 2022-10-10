
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::def::{Definition};

fn alt_context(old_var: &String, new_var: &String, v_type: &CCExpression,
               context: &[Statement]) -> Vec<Statement> {
    return context.iter().filter_map(
        |stmt| if &stmt.s_type == v_type && stmt.subject.var_str() == Some(new_var.to_string()){
            Some(Statement {
                subject: CCExpression::Var(old_var.to_string()),
                s_type: stmt.s_type.clone()
            })
        } else {
            Some(stmt.clone())
        }
        ).collect();
}

fn alt_vars(v_type: &CCExpression, context: &[Statement]) -> Vec<String> {
    return context.iter().filter_map(
        |stmt| if &stmt.s_type == v_type {
            if let Some(x) = stmt.subject.var_str() {
                Some(x)
            } else { None }
        } else {
            None
        }).collect();
}

pub fn abst_alt_equiv(j1: &Judgement, j2: &Judgement) -> bool {
    let alts = abst_alternatives(j1);
    for alt in alts {
        if alt.alpha_equiv(j2) { return true; }
    }
    return false;
}

pub fn abst_alternatives(jdg: &Judgement) -> Vec<Judgement> {
    if let CCExpression::Abs(v, v_type, ret) = &jdg.statement.subject {
        let alts = alt_vars(&v_type, &jdg.context);
        let output: Vec<Judgement> = alts.iter().map(
            |new_var| {
                let ctx = alt_context(&v, new_var, &v_type, &jdg.context);
                let new_stmt = Statement {
                    subject: CCExpression::Abs(
                                 new_var.to_string(),
                                 Box::new(*v_type.clone()),
                                 Box::new(*ret.clone())
                                 ),
                    s_type: jdg.statement.s_type.clone()
                };
                Judgement {
                    defs: jdg.defs.clone(),
                    context: ctx,
                    statement: new_stmt
                }
            }
            ).collect();
        return output;
    } else {
        return vec![];
    }
}

pub fn next_unused_var(context: &[Statement]) -> String {
    let used: Vec<String> = context.iter().filter_map(|stmt| {
        match &stmt.subject {
            CCExpression::Var(x) => Some(x.clone()),
            _ =>  None
        }
    }).collect();
    for ch in 'a'..'z' {
        if !used.contains(&ch.to_string()) {
            return ch.to_string();
        }
    }
    return String::from("x");
}

pub fn next_unused_type(context: &[Statement]) -> String {
    let used_var: Vec<String> = context.iter().filter_map(|stmt| {
        match &stmt.subject {
            CCExpression::Var(x) => Some(x.clone()),
            _ =>  None
        }}).collect();
    let used_type: Vec<String> = context.iter().filter_map(|stmt| {
        match &stmt.s_type {
            CCExpression::Var(x) => Some(x.clone()),
            _ =>  None
        }}).collect();
    let used: Vec<String> = [used_var, used_type].concat();
    for ch in 'A'..'Z' {
        if !used.contains(&ch.to_string()) {
            return ch.to_string();
        }
    }
    return String::from("x");
}

pub fn do_type_sub(s_type: &CCExpression, def: &Definition,
               arg_map: &Vec<Statement>) -> CCExpression {
    let replacements: Vec<(&String, &CCExpression)> = def.args.iter()
        .zip(arg_map.iter().map(|x| &x.subject)).collect();
    let mut output: CCExpression = s_type.clone();

    for (tok, rep) in replacements {
        output = output.substitute(tok, rep);
    }

    return output;
}

pub trait DerRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement>;
    fn name(&self) -> String;
    fn sig_size(&self) -> u32;

    fn validate(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>,
                    result: &Judgement) -> bool {
        if let Some(j) = self.apply(lhs, rhs) {
            if j.alpha_equiv(result) {
                return true;
            }
        }
        return false;
    }

    fn validate_many(&self, _judges: &[Judgement], _result: &Judgement) -> Option<Vec<u32>> {
        return None;
    }
}

