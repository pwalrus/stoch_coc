
use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};
use super::base::{ProofStrat};

pub struct UnpackTypeAbs {}

impl ProofStrat for UnpackTypeAbs {
    fn sub_goals(&self, ex: &CCExpression,
                 context: &[Statement],
                 inner_context: &[Statement],
                 _: &[Judgement],
                 _: &[Definition]) -> Result<Vec<Goal>, String> {
        if let CCExpression::TypeAbs(var, a_type, ret) = ex {
            let new_var = Statement::next_unused_var(&[context, inner_context].concat());
            let new_stmt = Statement {
                subject: CCExpression::Var(new_var.to_string()),
                s_type: *a_type.clone()
            };
            let subs = vec![
                Goal::Initial(
                    ret.substitute(var, &new_stmt.subject),
                    [inner_context, &[new_stmt]].concat())
            ];
            Ok(vec![Goal::Unpacked(CCExpression::Abs(
                                       new_var.to_string(),
                                       Box::new(*a_type.clone()),
                                       Box::new(CCExpression::Var("sub_{0}".to_string()))),
                                   CCExpression::TypeAbs(
                                       var.to_string(),
                                       Box::new(*a_type.clone()),
                                       Box::new(*ret.clone())),
                                       subs,
                                       inner_context.to_vec())])
        } else {
            Err(format!("Not a type abstraction: {}", ex.to_latex()))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};

    #[test]
    fn test_unpack_type_abs_strat() {
        let jdg: Judgement = parse_judgement("A:\\ast \\vdash y:\\prod x:A.A").unwrap();
        let strat = UnpackTypeAbs {};
        let ex = &jdg.statement.s_type;
        let context = &jdg.context;
        let res = strat.sub_goals(ex, context, &[], &[], &[]);

        match res {
            Ok(lst) => {
                assert_eq!(lst.len(), 1);
                if let Goal::Unpacked(inst, ex, subs, inner) = &lst[0] {
                    assert_eq!(inst.to_latex(), "\\lambda a : A . sub_{0}");
                    assert_eq!(ex.to_latex(), "A \\to A");
                    assert_eq!(inner, &[]);
                    if let Goal::Initial(ex, _) = &subs[0] {
                        assert_eq!(ex.to_latex(), "A");
                    } else { panic!(); }
                } else { panic!(); }
            },
            Err(_) => { panic!(); }
        }
    }
}

