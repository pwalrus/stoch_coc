
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::rules::base::{DerRule};


pub struct VarRule {}

impl DerRule for VarRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        if let Some(_) = rhs { return None; }
        if let Some(in_judge) = lhs {
            let stmt = &in_judge.statement;
            if let CCExpression::Star = &stmt.s_type {
                if let CCExpression::Var(_) = &stmt.subject {
                    let next = Statement::next_unused_var(&in_judge.context);
                    let new_stmt = Statement {
                        s_type: stmt.subject.clone(),
                        subject: CCExpression::Var(next) 
                    };
                    return Some(Judgement {
                        defs: in_judge.defs.clone(),
                        context: [
                            in_judge.context.clone(),
                            vec![new_stmt.clone()]].concat(),
                        statement: new_stmt
                    });
                }
            }
            if let CCExpression::Star = &stmt.subject {
                let next = Statement::next_unused_type(&in_judge.context);
                let new_stmt = Statement {
                    s_type: CCExpression::Star,
                    subject: CCExpression::Var(next) 
                };
                return Some(Judgement {
                    defs: in_judge.defs.clone(),
                    context: [
                        in_judge.context.clone(),
                        vec![new_stmt.clone()]].concat(),
                    statement: new_stmt
                });
            }
        }
        return None;
    }

    fn name(&self) -> String {
        return String::from("var");
    }

    fn sig_size(&self) -> u32 { return 1; }

    fn validate(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>,
                    result: &Judgement) -> bool {
        if let Some(_) = rhs { return false; }
        if let Some(lex) = lhs {
            if result.context.len() != lex.context.len() + 1 {return false; }
            let new_ctx = [lex.context.to_vec(), vec![result.statement.clone()]].concat();
            let has_type = lex.context.iter().any(|stmt| stmt.subject.alpha_equiv(&result.statement.s_type)) || lex.statement.subject.alpha_equiv(&result.statement.s_type);

            if (has_type || result.statement.s_type == CCExpression::Star)
                && result.context.iter().zip(&new_ctx).all(|(x, y)| x.alpha_equiv(y)) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_var_check() {
        let rule = VarRule {};
        let stmt = Statement {
            subject: CCExpression::Var(String::from("A")),
            s_type: CCExpression::Star
        };
        let jdg = Judgement {
            defs: vec![],
            context: vec![],
            statement: stmt
        };
        let output = rule.apply(Some(&jdg), None);
        assert_eq!(rule.name(), "var");
        assert_ne!(output, None);
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(x) = output {
            assert_eq!(&x.to_latex(), "a : A \\vdash a : A");
        }
    }

    #[test]
    fn double_var_check() {
        let rule = VarRule {};
        let stmt = Statement {
            subject: CCExpression::Var(String::from("A")),
            s_type: CCExpression::Star
        };
        let stmt1 = Statement {
            subject: CCExpression::Var(String::from("B")),
            s_type: CCExpression::Star
        };
        let jdg1 = Judgement {
            defs: vec![],
            context: vec![stmt.clone()],
            statement: stmt.clone()
        };
        let jdg2 = Judgement {
            defs: vec![],
            context: vec![stmt.clone(), stmt1.clone()],
            statement: stmt1.clone()
        };
        assert!(rule.validate(Some(&jdg1), None, &jdg2));
    }
}
