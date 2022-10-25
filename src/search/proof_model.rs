
use crate::model::def::{Definition};
use crate::model::partial::{PartialSol};

use super::base::{SearchModel};
use super::proof::subgoal::{next_sol_from_sol};
use super::proof::finalize::{recursive_finalize};

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

        fn finalize(&self, x: PartialSol) -> Result<PartialSol, String> {
            return recursive_finalize(&x, &self.defs);
        }
}


