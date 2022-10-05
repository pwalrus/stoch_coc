use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::def::{Definition};
use crate::model::rules::base::{DerRule};


fn build_arg_map(def: &Definition, known_args: &Vec<Statement>) -> Option<Vec<Statement>> {
    if !def.context.iter().all(
        |arg| known_args.iter().any(|knwn| arg.s_type == knwn.s_type) ) {
        return None;
    }
    let arg_types: Vec<&CCExpression> = def.args.iter().filter_map(
        |arg| Some(&def.context.iter().find(
            |stmt| stmt.subject.var_str() == Some(arg.clone())).unwrap().s_type)
        ).collect();
    return Some(arg_types.iter().filter_map(
            |argt: &&CCExpression| known_args.iter().find(
                |knwn| knwn.s_type == **argt)
            ).map(|x| x.clone()).collect());
}

fn do_type_sub(s_type: &CCExpression, def: &Definition,
               arg_map: &Vec<Statement>) -> CCExpression {
    let replacements: Vec<(&String, &CCExpression)> = def.args.iter()
        .zip(arg_map.iter().map(|x| &x.subject)).collect();
    let mut output: CCExpression = s_type.clone();

    for (tok, rep) in replacements {
        output = output.substitute(tok, rep);
    }

    return output;
}

pub struct InstRule {
    pub defs : Vec<Definition>
}

impl DerRule for InstRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        if let None = lhs { return None; }
        if let Some(_) = rhs { return None; }
        for def in &self.defs {
            if let Some(args) = build_arg_map(def, &vec![lhs.unwrap().statement.clone()]) {
                let arg_names: Vec<CCExpression> = args.iter().map(
                    |arg| arg.subject.clone()).collect();
                return Some(Judgement {
                    defs: lhs.unwrap().defs.clone(),
                    context: lhs.unwrap().context.clone(),
                    statement: Statement {
                        subject: CCExpression::Def(def.name.clone(),
                        arg_names),
                        s_type: do_type_sub(&def.body.s_type, &def, &args)
                    }
                });
            }
        }
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
        let rule = InstRule { defs: vec![def.clone()] };
        let jdg = Judgement {
            defs: vec![],
            context: vec![c_stmt.clone()],
            statement: c_stmt.clone()
        };
        let def_stmt = Statement {
            subject: CCExpression::Def(
                         "id".to_string(),
                         vec![CCExpression::Var("I".to_string())]
                         ),
            s_type: CCExpression::TypeAbs(
                         "x".to_string(),
                         Box::new(CCExpression::Var("I".to_string())),
                         Box::new(CCExpression::Var("I".to_string()))
                         )
        };
        let def_jdg = Judgement {
            defs: vec![],
            context: vec![c_stmt.clone()],
            statement: def_stmt
        };
        assert_eq!(jdg.to_latex(), "I : \\ast \\vdash I : \\ast");
        assert_eq!(def.to_latex(), "A : \\ast \\vartriangleright id \\langle A \\rangle := \\lambda x : A . x : \\prod x : A . A");
        assert_eq!(def_jdg.to_latex(), "I : \\ast \\vdash id \\langle I \\rangle : \\prod x : I . I");

        let output = rule.apply(Some(&jdg), None);
        assert_eq!(rule.name(), "inst");
        assert_ne!(output, None);
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(x) = output {
            assert_eq!(&x.to_latex(), "I : \\ast \\vdash id \\langle I \\rangle : \\prod x : I . I");
        }
    }
}
