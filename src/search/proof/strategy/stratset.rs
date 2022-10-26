
use super::untypeabs::{UnpackTypeAbs};
use super::incontext::{InContext};
use super::base::{ProofStrat};


pub fn standard_strategy() -> Vec<Box<dyn ProofStrat>> {
    return vec![
        Box::new(InContext {}),
        Box::new(UnpackTypeAbs {})
    ];
}

