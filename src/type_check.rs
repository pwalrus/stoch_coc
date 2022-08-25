
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

fn rule_applies_two(jdg: &Judgement, 
                    rule: &dyn DerRule,
                    lines: &[Judgement]
                    )  -> Option<LineRef> {
    for (idx1, j1) in lines.iter().enumerate() {
        for (idx2, j2) in lines.iter().enumerate() {
            if let Some(j) = rule.apply(Some(j1), Some(j2)) {
                println!("comparing ({})  {} with {}", rule.name(), j.to_latex(), jdg.to_latex());
                if j.alpha_equiv(&jdg) {
                    return Some(LineRef {
                        rule: rule.name(),
                        line1: Some(idx1 as u32),
                        line2: Some(idx2 as u32)
                    });
                }
            }
        }
    }
    return None;
}
fn rule_applies_one(jdg: &Judgement, 
                    rule: &dyn DerRule,
                    lines: &[Judgement]
                    )  -> Option<LineRef> {
    for (idx1, j1) in lines.iter().enumerate() {
        if let Some(j) = rule.apply(Some(j1), None) {
            println!("comparing ({})  {} with {}", rule.name(), j.to_latex(), jdg.to_latex());
            if j.alpha_equiv(jdg) {
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
    for (idx, jdg) in judges.iter().enumerate() {
        let mut found : Option<LineRef> = None;
        for rule in &rules {
            if rule.sig_size() == 0 {
                found = rule_applies_zero(jdg, &(**rule));
            } else if rule.sig_size() == 1 {
                found = rule_applies_one(jdg, &(**rule), &judges[0..idx]);
            } else if rule.sig_size() == 2 {
                found = rule_applies_two(jdg, &(**rule), &judges[0..idx]);
            }
            if let Some(_) = &found {
                break;
            } 
        }
        if let Some(r) = found {
            output.push(r);
        } else {
            return None;
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

    #[test]
    fn type_check_var() {
        let lines: Vec<Judgement> = vec![
            parse_judgement("\\vdash \\ast : \\square").unwrap(),
            parse_judgement("B : \\ast \\vdash B : \\ast").unwrap()
        ];
        let refs = check_proof(&lines);
        if let Some(r) = refs {
            assert_eq!(r, vec![
               LineRef { rule: String::from("sort"), 
                   line1: None, 
                   line2: None },
               LineRef { rule: String::from("var"), 
                   line1: Some(0), 
                   line2: None }
            ]);
        } else {
            panic!();
        }
    }

    #[test]
    fn type_check_var_reject() {
        let lines: Vec<Judgement> = vec![
            parse_judgement("\\vdash \\ast : \\square").unwrap(),
            parse_judgement("A : \\ast \\vdash B : \\ast").unwrap()
        ];
        let refs = check_proof(&lines);
        assert_eq!(refs, None);
    }

    #[test]
    fn type_check_weak() {
        let lines: Vec<Judgement> = vec![
            parse_judgement("\\vdash \\ast : \\square").unwrap(),
            parse_judgement("A : \\ast \\vdash A : \\ast").unwrap(),
            parse_judgement("A : \\ast, x : A \\vdash A : \\ast").unwrap()
        ];
        let refs = check_proof(&lines);
        if let Some(r) = refs {
            let r_str: Vec<String> = r.iter().filter_map(|x| 
                                                         Some(x.to_latex())
                                                         ).collect();
            assert_eq!(r_str, vec![ "sort", "var 0", "weak 1,1" ]);
        } else {
            panic!();
        }
    }
}

