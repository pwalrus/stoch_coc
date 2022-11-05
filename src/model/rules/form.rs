
use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::rules::base::{DerRule};

pub struct FormRule {}

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
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        if let Some(jdg1) = lhs {
            if let Some(jdg2) = rhs {
                let m_stmt = find_matching_stmt(&jdg2.context, &jdg1.statement);
                if let Some(stmt) = m_stmt {
                    let ctx = make_new_ctx(&jdg2.context, &stmt); 
                    if let CCExpression::Var(x) = stmt.subject {
                        let new_type = CCExpression::TypeAbs(
                                x,
                                Box::new(stmt.s_type),
                                Box::new(jdg2.statement.subject.clone())
                            );
                        let new_stmt = Statement {
                            subject: new_type,
                            s_type: jdg2.statement.s_type.clone()
                        };
                        return Some(Judgement {
                            defs: jdg1.defs.clone(),
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
    
    fn sig_size(&self) -> u32 { return 2; }

    fn validate(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>,
                    result: &Judgement) -> bool {
        if let Some(lex) = lhs {
            if !lex.statement.s_type.is_sort() { return false; }
            if let Some(rex) = rhs {
                if !rex.statement.s_type.is_sort() { return false; }
                match (&result.statement.subject, &result.statement.s_type) {
                    (CCExpression::TypeAbs(arg, a_type, ret), CCExpression::Star) => {
                        if **a_type != lex.statement.subject { return false; }
                        if rex.context.len() != result.context.len() + 1 { return false; }
                        let last_stmt = rex.context.last().unwrap();
                        if last_stmt.s_type != **a_type { return false; }
                        if !ret.substitute(arg, &last_stmt.subject).alpha_equiv(&rex.statement.subject) { return false; }
                        if !Statement::weaker_eq(&rex.context, &result.context) { return false; }
                        return true;
                    },
                    _ => { return false; }
                }
            }
        }
        false
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
            defs: vec![],
            context: vec![],
            statement: stmt1
        };
        let judg2 = Judgement {
            defs: vec![],
            context: vec![stmt3],
            statement: stmt2
        };
        assert_eq!(judg1.to_latex(), "\\vdash A : \\ast");

        let output = rule.apply(Some(&judg1), Some(&judg2));
        assert_ne!(output, None);
        assert_eq!(rule.name(), "form");
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(judge) = output {
            assert_eq!(judge.to_latex(), "\\vdash A \\to B : \\ast");
        } else {
            panic!();
        }
    }
}
