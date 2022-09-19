use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::def::{Definition};
use crate::model::rules::base::{DerRule};


pub struct InstRule {
    pub defs : Vec<Definition>
}

impl DerRule for InstRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        return None;
    }

    fn name(&self) -> String {
        return String::from("inst");
    }

    fn sig_size(&self) -> u32 { return 1; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_inst_check() {
        let c_stmt = Statement {
            subject: CCExpression::Var(String::from("I")),
            s_type: CCExpression::Star
        };
        let d_stmt = Statement {
            subject: CCExpression::Var(String::from("A")),
            s_type: CCExpression::Star
        };
        let b_stmt = Statement {
            subject: CCExpression::Abs(
                         "x".to_string(),
                         Box::new(CCExpression::Var("A".to_string())),
                         Box::new(CCExpression::Var("x".to_string()))
                         ),
            s_type: CCExpression::TypeAbs(
                         "x".to_string(),
                         Box::new(CCExpression::Var("A".to_string())),
                         Box::new(CCExpression::Var("A".to_string()))
                         )
        };
        let def = Definition {
            context: vec![d_stmt],
            name: "id".to_string(),
            args: vec!["A".to_string()],
            body: b_stmt

        };
        // let rule = InstRule { defs: vec![def.clone()] };
        let jdg = Judgement {
            defs: vec![],
            context: vec![c_stmt.clone()],
            statement: c_stmt
        };

        assert_eq!(jdg.to_latex(), "I : \\ast \\vdash I : \\ast");
        assert_eq!(def.to_latex(), "A : \\ast \\vartriangleright id \\langle A \\rangle := \\lambda x : A . x : \\prod x : A . A");

        /*
        let output = rule.apply(Some(&jdg), None);
        assert_eq!(rule.name(), "inst");
        assert_ne!(output, None);
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(x) = output {
            assert_eq!(&x.to_latex(), "a : A \\vdash a : A");
        }
        */
    }
}
