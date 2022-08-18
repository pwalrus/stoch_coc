
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement, Statement};
use crate::model::rules::base::{DerRule};

struct FormRule {}

fn find_matching_stmt(context: &[Statement], stmt: &Statement) -> Option<Statement> {
    for x in context {
        if x.s_type == stmt.subject {
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

impl DerRule for FormRule {
    fn apply(&self, lhs: Option<Judgement>, rhs: Option<Judgement>) -> Option<Judgement> {
        if let Some(jdg1) = lhs {
            if let Some(jdg2) = rhs {
                let m_stmt = find_matching_stmt(&jdg2.context, &jdg1.statement);
                if let Some(stmt) = m_stmt {
                    let ctx = make_new_ctx(&jdg2.context, &stmt); 
                    if let CCExpression::Var(x) = stmt.subject {
                        let new_type = CCExpression::TypeAbs(
                                x,
                                Box::new(stmt.s_type),
                                Box::new(jdg2.statement.subject)
                            );
                        let new_stmt = Statement {
                            subject: new_type,
                            s_type: jdg2.statement.s_type
                        };
                        return Some(Judgement {
                            statement: new_stmt,
                            context: ctx
                        });
                    }
                }
            }
        }
        return None;
    }

    fn name(&self) -> String {
        return String::from("form");
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_form_apply() {
        let rule = FormRule {};
        let stmt1 = Statement {
            subject: CCExpression::Var(String::from("A")),
            s_type: CCExpression::Star
        };
        let stmt2 = Statement {
            subject: CCExpression::Var(String::from("B")),
            s_type: CCExpression::Star
        };
        let stmt3 = Statement {
            subject: CCExpression::Var(String::from("x")),
            s_type: CCExpression::Var(String::from("A"))
        };
        let judg1 = Judgement {
            context: vec![],
            statement: stmt1
        };
        let judg2 = Judgement {
            context: vec![stmt3],
            statement: stmt2
        };
        assert_eq!(judg1.to_latex(), "\\vdash A : \\ast");

        let output = rule.apply(Some(judg1), Some(judg2));
        assert_ne!(output, None);
        assert_eq!(rule.name(), "form");
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(judge) = output {
            assert_eq!(judge.to_latex(), "\\vdash \\prod x : A . B : \\ast");
        } else {
            panic!();
        }
    }
}
