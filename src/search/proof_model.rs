
use crate::model::def::{Definition};
use crate::model::partial::{Goal, PartialSol};

use super::base::{SearchModel};
use super::proof::subgoal::{unpack_goal};

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

pub struct ProofSearchModel {
    pub defs: Vec<Definition>
}


impl SearchModel<PartialSol> for ProofSearchModel {
        fn done(&self, x: &PartialSol) -> bool {
            return x.count().i == 0;
        }

        fn next(&self, x: &PartialSol) -> Vec<PartialSol> {
            let res = next_sol_from_sol(x, &self.defs);
            if let Ok(x) = res {
                return x;
            } else {
                return vec![];
            }
        }

        fn weight(&self, x: &PartialSol) -> i32 {
            let c = x.count();
            let w = (c.i*20 + c.u*10 + c.f) as i32;
            return 10 - w
        }
}


