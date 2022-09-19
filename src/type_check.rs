
use crate::model::judgement::{Judgement};
use crate::model::def::{Definition};
use crate::parser::{parse_judgement};
use crate::model::rules::ruleset::{all_rules};
use crate::model::rules::base::{DerRule, abst_alternatives};
use crate::model::proof::{LineRef};


fn abst_alt_equiv(j1: &Judgement, j2: &Judgement) -> bool {
    let alts = abst_alternatives(j1);
    for alt in alts {
        if alt.alpha_equiv(j2) { return true; }
    }
    return false;
}

fn rule_applies_two(jdg: &Judgement,
                    rule: &dyn DerRule,
                    lines: &[Judgement]
                    )  -> Option<LineRef> {
    for (idx1, j1) in lines.iter().enumerate() {
        for (idx2, j2) in lines.iter().enumerate() {
            if let Some(j) = rule.apply(Some(j1), Some(j2)) {
                if j.alpha_equiv(&jdg) || abst_alt_equiv(&j, &jdg) {
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


pub fn check_proof(defs: &[Definition], 
                   judges: &[Judgement]) -> Result<Vec<LineRef>, String> {
    let rules = all_rules(defs);
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
            return Err(format!("No rule applies on line {}: {}",
                               idx,
                               jdg.to_latex()));
        }
    }

    return Ok(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_type_check() {
        let lines: Vec<Judgement> = vec![
            parse_judgement("\\vdash \\ast : \\square").unwrap()
        ];
        let refs = check_proof(&[], &lines);
        if let Ok(r) = refs {
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
        let refs = check_proof(&[], &lines);
        if let Ok(r) = refs {
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
        let refs = check_proof(&[], &lines);

        assert!(matches!(refs, Result::Err{ .. }));
    }

    #[test]
    fn type_check_weak() {
        let lines: Vec<Judgement> = vec![
            parse_judgement("\\vdash \\ast : \\square").unwrap(),
            parse_judgement("A : \\ast \\vdash A : \\ast").unwrap(),
            parse_judgement("A : \\ast, x : A \\vdash A : \\ast").unwrap(),
            parse_judgement("A : \\ast, x : A \\vdash x : A").unwrap(),
            parse_judgement("A : \\ast \\vdash \\prod x  : A . A : \\ast").unwrap(),
            parse_judgement("A : \\ast \\vdash \\lambda x : A . x : \\prod x  : A . A").unwrap()
        ];
        let refs = check_proof(&[], &lines);

        if let Ok(r) = refs {
            let r_str: Vec<String> = r.iter().filter_map(|x|
                                                         Some(x.to_latex())
                                                         ).collect();
            assert_eq!(r_str, vec![
                       "sort",
                       "var 0",
                       "weak 1,1",
                       "var 1",
                       "form 1,2",
                       "abst 3,4"
            ]);
        } else {
            panic!();
        }
    }

    #[test]
    fn alt_abst() {
        let jdg: Judgement = parse_judgement(
            "A:\\ast, b:A\\vdash \\lambda x : A . x : \\prod x : A . A"
            ).unwrap();
        let alts = abst_alternatives(&jdg);

        assert_eq!(alts.len(), 1);
        assert_eq!(alts[0].to_latex(),
            "A : \\ast, x : A \\vdash \\lambda b : A . x : \\prod x : A . A"
        );

    }
}

