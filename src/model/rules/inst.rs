use crate::model::expression::CCExpression;
use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::def::{Definition};
use crate::model::rules::base::{DerRule, do_type_sub};


fn build_arg_map(def: &Definition, known_args: &[(usize, Statement)],
                 args: &[CCExpression]) -> Option<Vec<(usize, Statement)>> {
    if !def.context.iter().all(
        |arg| known_args.iter().any(|knwn| arg.s_type == knwn.1.s_type) ) {
        return None;
    }
    let arg_types: Vec<CCExpression> = def.type_list().unwrap();
    let output: Vec<(usize, Statement)> = arg_types.iter().zip(args).filter_map(
            |(argt, uarg)| known_args.iter().find(
                |knwn| knwn.1.s_type == *argt && knwn.1.subject == *uarg)
            ).map(|x| x.clone()).collect();
    if args.len() > 0 && output.len() == args.len() {
        return Some(output);
    } else if args.len() > 0 {
        return None;
    }
    return Some(arg_types.iter().filter_map(
            |argt: &CCExpression| known_args.iter().find(
                |knwn| knwn.1.s_type == *argt)
            ).map(|x| x.clone()).collect());
}

fn find_jdg_for_def(def: &Definition, args: &[CCExpression],
                    judges: &[Judgement], result: &Judgement) -> Option<Vec<u32>> {
    let usable_stmts: Vec<(usize, Statement)> = judges.iter().enumerate().
        filter_map(|(idx, jdg)| if result.weaker_eq(jdg) {
            Some((idx, jdg.statement.clone()))
        } else {
            None
        }).collect();
    if let Some(known) = build_arg_map(def, &usable_stmts, args) {
        let arg_names: Vec<CCExpression> = known.iter().map(
            |arg| arg.1.subject.clone()).collect();
        let new_def = CCExpression::Def(def.name.clone(), arg_names);
        let new_stmt = Statement {
            subject: new_def,
            s_type: do_type_sub(&def.body.s_type, &def,
                                &known.iter().map(
                                    |x| x.1.clone()).collect())
        };
        let new_jdg = Judgement {
            defs: result.defs.clone(),
            context: result.context.clone(),
            statement: new_stmt
        };
        if new_jdg.alpha_equiv(result) {
            return Some(known.iter().map(|(idx, _)| *idx as u32).collect());
        }
    }
    return None;
}

pub struct InstRule {
    pub defs : Vec<Definition>
}

impl DerRule for InstRule {
    fn apply(&self, lhs: Option<&Judgement>, rhs: Option<&Judgement>) -> Option<Judgement> {
        if let None = lhs { return None; }
        if let Some(_) = rhs { return None; }
        for def in &self.defs {
            if let Some(args) = build_arg_map(def, &vec![(0, lhs.unwrap().statement.clone())], &[]) {
                let arg_names: Vec<CCExpression> = args.iter().map(
                    |arg| arg.1.subject.clone()).collect();
                return Some(Judgement {
                    defs: lhs.unwrap().defs.clone(),
                    context: lhs.unwrap().context.clone(),
                    statement: Statement {
                        subject: CCExpression::Def(def.name.clone(),
                        arg_names),
                        s_type: do_type_sub(&def.body.s_type, &def, &args.iter().map(|x| x.1.clone()).collect())
                    }
                });
            }
        }
        return None;
    }

    fn validate_many(&self, judges: &[Judgement], result: &Judgement) -> Option<Vec<u32>> {
        if let CCExpression::Def(name, args) = &result.statement.subject {
            for def in &self.defs {
                if def.name == *name && def.args.len() == args.len() {
                    let lines = find_jdg_for_def(def, args, judges, result);
                    if let Some(_) = lines {
                        return lines;
                    }
                }
            }
        }
        return None
    }

    fn name(&self) -> String {
        return String::from("inst");
    }

    fn sig_size(&self) -> u32 { return 3; }
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
        assert_eq!(def.to_latex(), "A : \\ast \\vartriangleright id \\langle A \\rangle := \\lambda x : A . x : A \\to A");
        assert_eq!(def_jdg.to_latex(), "I : \\ast \\vdash id \\langle I \\rangle : I \\to I");

        let output = rule.apply(Some(&jdg), None);
        assert_eq!(rule.name(), "inst");
        assert_ne!(output, None);
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(x) = output {
            assert_eq!(&x.to_latex(), "I : \\ast \\vdash id \\langle I \\rangle : I \\to I");
        }

        let output2 = rule.validate_many(&[jdg], &def_jdg);
        assert_eq!(output2, Some(vec![0]));
    }
}
