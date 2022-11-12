
use super::untypeabs::{UnpackTypeAbs};
use super::incontext::{InContext};
use super::known_arrow::{KnownArrow};
use super::prod_elim::{ProdElim};
use super::def_known::{DefKnown};
use super::base::{ProofStrat};


pub fn standard_strategy() -> Vec<Box<dyn ProofStrat>> {
    return vec![
        Box::new(InContext {}),
        Box::new(KnownArrow {}),
        Box::new(ProdElim {}),
        Box::new(DefKnown {}),
        Box::new(UnpackTypeAbs {})
    ];
}

