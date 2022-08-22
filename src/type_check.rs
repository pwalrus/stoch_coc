
use crate::model::judgement::{Judgement};
use crate::parser::{parse_judgement};
use crate::model::rules::ruleset::{all_rules};
use crate::model::rules::base::{DerRule};

#[derive(Debug,PartialEq)]
pub struct LineRef {
    rule: String,
    line1: Option<u32>,
    line2: Option<u32>
}

impl LineRef {

    fn to_latex(&self) -> String {
        if let Some(l) = self.line1 {
            if let Some(r) = self.line2 {
                return format!("{} {},{}", &self.rule, l, r);
            } else {
                return format!("{} {}", &self.rule, l);
            }
        } else {
            return self.rule.clone();
        }
    }
}

fn rule_applies_one(jdg: &Judgement, 
                    rule: &dyn DerRule,
                    lines: &[Judgement]
                    )  -> Option<LineRef> {
    for (idx1, j1) in lines.iter().enumerate() {
        if let Some(j) = rule.apply(Some(j1.clone()), None) {
            if &j == jdg {
                return Some(LineRef {
                    rule: rule.name(),
                    line1: Some(idx1 as u32),
                    line2: None
                });
            }
        }
}
    return None;

}

fn rule_applies_zero(jdg: &Judgement, 
                     rule: &dyn DerRule)  -> Option<LineRef> {
    if let Some(j) = rule.apply(None, None) {
        println!("comparing {} with {}", j.to_latex(), jdg.to_latex());
        if &j == jdg {
            return Some(LineRef {
                rule: rule.name(),
                line1: None,
                line2: None
            });
        }
    }
    return None;
}


pub fn check_proof(judges: &[Judgement]) -> Option<Vec<LineRef>> {
    let rules = all_rules();
    let mut output: Vec<LineRef> = vec![];
    println!("starting with {} lines", judges.len());
    for (idx, jdg) in judges.iter().enumerate() {
        println!("trying {} = {}", idx, jdg.to_latex());
        for rule in &rules {
            println!("trying {} = {} with {}", idx, jdg.to_latex(), rule.name());
            let mut lref = None;
            if rule.sig_size() == 0 {
                lref = rule_applies_zero(jdg, &(**rule));
            } else if rule.sig_size() == 1 {
                lref = rule_applies_zero(jdg, &(**rule));
            }

            if let Some(r) = lref {
                output.push(r);
                break;
            } else {
                return None;
            }
        }
    }

    return Some(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_type_check() {
        let lines: Vec<Judgement> = vec![
            parse_judgement("\\vdash \\ast : \\square").unwrap()
        ];
        let refs = check_proof(&lines);
        if let Some(r) = refs {
            assert_eq!(r, vec![
               LineRef { rule: String::from("sort"), 
                   line1: None, 
                   line2: None }
            ]);
            assert_eq!(r[0].to_latex(), "sort");
        } else {
            panic!();
        }
    }
}

