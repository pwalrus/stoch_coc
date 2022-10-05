
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::rules::base::{DerRule, abst_alt_equiv};

fn find_matching_stmt(context: &[Statement], stmt: &CCExpression) -> Option<Statement> {
    for x in context {
        if &x.s_type == stmt {
            return Some(x.clone());
        }
    }
    return None
}

fn make_new_ctx(context: &[Statement], stmt: &Statement) -> Vec<Statement> {
    let ctx: Vec<Statement> = context.iter().filter_map(
        |s| if s != stmt {Some(s.clone())} else {None}
        ).collect();
    return ctx
}

pub struct AbstRule {}

impl DerRule for AbstRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        if let Some(a_jdg) = lhs {
            if let Some(t_jdg) = rhs {
                if let CCExpression::TypeAbs(_ph, a_type, r_type) = &t_jdg.statement.subject {
                    if a_jdg.statement.s_type != **r_type {
                        return None;
                    }
                    let o_m_stmt = find_matching_stmt(&a_jdg.context, &*a_type);
                    if let Some(m_stmt) = o_m_stmt {
                        let new_ctx = make_new_ctx(&a_jdg.context, &m_stmt);
                        if let CCExpression::Var(ph2) = m_stmt.subject {
                            let stmt = Statement {
                                subject: CCExpression::Abs(
                                             ph2,
                                             Box::new(m_stmt.s_type.clone()),
                                             Box::new(a_jdg.statement.subject.clone())),
                                s_type: CCExpression::TypeAbs(
                                    _ph.clone(), 
                                    a_type.clone(),
                                    r_type.clone())
                            };
                            return Some(Judgement {
                                defs: vec![],
                                context: new_ctx,
                                statement: stmt
                            });
                        }
                    }
                }
            }
        }
        return None;
    }

    fn name(&self) -> String {
        return String::from("abst");
    }

    fn sig_size(&self) -> u32 { return 2; }

    fn validate(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>,
                    result: &Judgement) -> bool {
        if let Some(j) = self.apply(lhs, rhs) {
            if j.alpha_equiv(&result) || abst_alt_equiv(&j, &result) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_abst() {
        let rule = AbstRule {};
        let stmt1 = Statement {
            subject: CCExpression::Var(String::from("M")),
            s_type: CCExpression::Var(String::from("B"))
        };
        let stmt1c = Statement {
            subject: CCExpression::Var(String::from("x")),
            s_type: CCExpression::Var(String::from("A"))
        };
        let jdg1 = Judgement {
            defs: vec![],
            statement: stmt1,
            context: vec![stmt1c]
        };
        let stmt2 = Statement {
            s_type: CCExpression::Sq,
            subject: CCExpression::TypeAbs(
                String::from("x"),
                Box::new(CCExpression::Var(String::from("A"))),
                Box::new(CCExpression::Var(String::from("B")))
                )
        };
        let jdg2 = Judgement {
            defs:vec![],
            statement: stmt2,
            context: vec![]
        };
        let stmt3 = Statement {
            subject: CCExpression::Abs(
                String::from("x"),
                Box::new(CCExpression::Var(String::from("A"))),
                Box::new(CCExpression::Var(String::from("M")))
                ),
            s_type: CCExpression::TypeAbs(
                String::from("x"),
                Box::new(CCExpression::Var(String::from("A"))),
                Box::new(CCExpression::Var(String::from("B")))
                )
        };
        let jdg3 = Judgement {
            defs: vec![],
            statement: stmt3,
            context: vec![]
        };
        assert_eq!(jdg1.to_latex(), "x : A \\vdash M : B");
        assert_eq!(jdg2.to_latex(), "\\vdash \\prod x : A . B : \\square");
        assert_eq!(jdg3.to_latex(), "\\vdash \\lambda x : A . M : \\prod x : A . B");

        let output = rule.apply(Some(&jdg1), Some(&jdg2));
        assert_eq!(rule.name(), "abst");
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(judge) = output {
            assert_eq!(judge.to_latex(), jdg3.to_latex());
        } else {
            panic!();
        }
    }
}
