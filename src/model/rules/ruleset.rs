use super::base::DerRule;
use super::sort::SortRule;
use super::var::VarRule;
use super::weak::WeakRule;
use super::form::FormRule;
use super::appl::ApplRule;
use super::abst::AbstRule;
use super::conv::ConvRule;
use super::inst::InstRule;

use crate::model::def::Definition;


pub fn all_rules(defs: &[Definition]) -> Vec<Box<dyn DerRule>> {
    return vec![
        Box::new(SortRule {}),
        Box::new(VarRule {}),
        Box::new(WeakRule {}),
        Box::new(FormRule {}),
        Box::new(ApplRule {}),
        Box::new(AbstRule {}),
        Box::new(InstRule { defs: defs.to_vec() }),
        Box::new(ConvRule {})
    ];
}

