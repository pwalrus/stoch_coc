use super::base::DerRule;
use super::sort::SortRule;
use super::var::VarRule;
use super::weak::WeakRule;
use super::form::FormRule;
use super::appl::ApplRule;
use super::abst::AbstRule;
use super::conv::ConvRule;


pub fn all_rules() -> Vec<Box<dyn DerRule>> {
    return vec![
        Box::new(SortRule {}),
        Box::new(VarRule {}),
        Box::new(WeakRule {}),
        Box::new(FormRule {}),
        Box::new(ApplRule {}),
        Box::new(AbstRule {}),
        Box::new(ConvRule {})
    ];
}

