

use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};
use super::base::{ProofStrat};

pub struct KnownArrow {}


fn arr_ends_with<T: PartialEq>(lhs: &[T], rhs: &[T]) -> bool {
    if lhs.len() <= rhs.len() || lhs.len() == 0 {
        return false;
    }
    let offs: usize = (lhs.len() as isize - rhs.len() as isize).try_into().unwrap();
    for (idx, elem) in rhs.iter().enumerate() {
        if lhs[idx + offs] != *elem { return false; }
    }
    return true;
}

fn is_arrow_match(lhs: &CCExpression, rhs: &CCExpression) -> bool {
    return arr_ends_with(&lhs.arrow_chain(), &rhs.arrow_chain());
}

fn make_init_goals(lhs: &CCExpression, rhs: &CCExpression,
                   inner_context: &[Statement]) -> Vec<Goal> {
    let l_arr = lhs.arrow_chain();
    let r_arr = rhs.arrow_chain();
    l_arr[0..l_arr.len() - r_arr.len()].iter().map(
        |ex| Goal::Initial((*ex).clone(), inner_context.to_vec())
        ).collect()
}

fn make_inst(arrow: &CCExpression, subs: &Vec<Goal>) -> CCExpression {
    let mut output: CCExpression = arrow.clone();
    for (idx, _) in subs.iter().enumerate() {
        output = CCExpression::Application(
            Box::new(output),
            Box::new(CCExpression::Var(format!("sub_{{{}}}", idx)))
            )
    }
    output
}

fn make_sub_goal(arrow: &CCExpression,
                 ex: &CCExpression,
                 subs: Vec<Goal>,
                 inner_context: &[Statement]) -> Goal {
    let inst = make_inst(arrow, &subs);
    Goal::Unpacked(inst, ex.clone(), subs, inner_context.to_vec())
}

impl ProofStrat for KnownArrow {
    fn sub_goals(&self, ex: &CCExpression,
                 context: &[Statement],
                 inner_context: &[Statement],
                 concs: &[Judgement],
                 _: &[Definition]) -> Result<Vec<Goal>, String> {
        let full_context: Vec<Statement> = context.iter().chain(inner_context).map(|x| x.clone()).collect();
        let usable_conc: Vec<Statement> = concs.iter().filter(
            |j| Statement::weaker_eq(&full_context, &j.context)
            ).map(|j| j.statement.clone()).collect();

        let output: Vec<Goal> = full_context.iter().chain(&usable_conc).filter_map(
                |stmt1| {
                if is_arrow_match(&stmt1.s_type, ex) {
                    let inits = make_init_goals(&stmt1.s_type, ex, inner_context);
                    Some(make_sub_goal(&stmt1.subject, ex, inits, inner_context))
                } else {
                    None
                }
            }).collect();
        if output.len() > 0 {
            Ok(output)
        } else {
            Err(format!("failed to find arrow matching: {}", ex.to_latex()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};

    #[test]
    fn test_known_arrow_strat() {
        let jdg: Judgement = parse_judgement("A:\\ast, B:\\ast, C:\\ast, q:A\\to B\\to C \\vdash z : C").unwrap();
        let strat = KnownArrow {};
        let ex = &jdg.statement.s_type;
        let context = &jdg.context;
        let res = strat.sub_goals(ex, context, &[], &[], &[]);

        match res {
            Ok(lst) => {
                assert_eq!(lst.len(), 1);
                if let Goal::Unpacked(inst,ex,subs,inner) = &lst[0] {
                    assert_eq!(inst.to_latex(), "q sub_{0} sub_{1}");
                    assert_eq!(ex.to_latex(), "C");
                    assert_eq!(subs.len(), 2);
                    if let Goal::Initial(ex0,_) = &subs[0] {
                        assert_eq!(ex0.to_latex(), "A");
                    } else {
                        panic!();
                    }
                    if let Goal::Initial(ex1,_) = &subs[1] {
                        assert_eq!(ex1.to_latex(), "B");
                    } else {
                        panic!();
                    }
                    assert_eq!(inner.len(), 0);
                } else { panic!(); }
            },
            Err(_) => { panic!(); }
        }
    }
}
