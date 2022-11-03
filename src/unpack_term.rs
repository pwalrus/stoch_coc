use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::rules::base::{do_type_sub};

fn unpack_star(_: &[Statement]) -> Result<Vec<Judgement>, String> {
    let stmt = Statement {
        subject: CCExpression::Star,
        s_type: CCExpression::Sq
    };

    return Ok(vec![Judgement {
        defs: vec![],
        statement: stmt,
        context: vec![]
    }]);
}


fn unpack_var(var: &str, context: &[Statement], defs: &[Definition]) -> Result<Vec<Judgement>, String> {
    let v_type: Option<CCExpression> = context.iter().filter_map(
        |st| if st.subject.var_str() == Some(var.to_string()) {
            return Some(st.s_type.clone());
        } else {
            return None;
        }).next();

    if let Some(s_type) = v_type {
        let stmt = Statement {
            subject: CCExpression::Var(var.to_string()),
            s_type: s_type
        };
        let ctx2: Vec<Statement> = context.iter().filter_map(
            |st| if st.subject.var_str() == Some(var.to_string()) {
                return None;
            } else {
                return Some(st.clone());
            }).collect();

        let res_r = unpack_term(&stmt.s_type, &ctx2, defs);
        if let Err(msg) = res_r { return Err(msg); }
        return Ok([res_r.unwrap(), vec![Judgement {
            defs: vec![],
            statement: stmt,
            context: context.to_vec()
        }]].concat());
    }
    return Err(format!("failed to unpack Var term: {}", var));
}

fn unpack_type_abs(var: &str, v_type: &CCExpression, ret: &CCExpression,
                   context: &[Statement], defs: &[Definition]) -> Result<Vec<Judgement>, String> {
    let p1_r = unpack_term(v_type, context, defs);

    if let Err(msg) = p1_r { return Err(msg); }

    let p1 = p1_r.unwrap();

    let stmt = Statement {
        subject: CCExpression::Var(var.to_string()),
        s_type: v_type.clone()
    };
    let new_ctx = [context, &vec![stmt]].concat();

    let p2_r = unpack_term(ret, &new_ctx, defs);
    if let Err(msg) = p2_r { return Err(msg); }
    let p2 = p2_r.unwrap();

    let last = Judgement {
        defs: vec![],
        context: context.to_vec(),
        statement: Statement {
            subject: CCExpression::TypeAbs(String::from(var),
                                           Box::new(v_type.clone()),
                                           Box::new(ret.clone())),
            s_type: p2.last().unwrap().statement.s_type.clone()
        }
    };

    return Ok(remove_dup(p1.iter().chain(p2.iter())
                      .chain(std::iter::once(&last))));
}

fn def_arg_type_eq(def: &Definition, args: &Vec<Vec<Judgement>>) -> bool {
    let tl_o = def.type_list();
    if let None = tl_o {
        return false;
    }
    let d_types = tl_o.unwrap();
    let a_types: Vec<&CCExpression> = args.iter().map(
        |arg_list| &arg_list.last().unwrap().statement.s_type).collect();

    return a_types.iter().zip(d_types).all(
        |(a, d)| **a == d);
}

fn unpack_def(name: &str, args: &[CCExpression],
                   context: &[Statement], defs: &[Definition]) -> Result<Vec<Judgement>, String> {

    let recur_res: Vec<Result<Vec<Judgement>, String>> = args.iter().map(
        |x| unpack_term(x, context, defs)
        ).collect();
    if recur_res.iter().any(|x| x.is_err()) {
        return Err(recur_res.iter().find(|x| x.is_err()).as_ref().unwrap().as_ref().unwrap_err().to_string());
    }
    let recur_args: Vec<Vec<Judgement>> = recur_res.iter().map(|x| x.clone().unwrap()).collect();
    let curr_def_o: Option<&Definition> = defs.iter().find(
        |def| def.name == name && def.args.len() == args.len()
        && def_arg_type_eq(def, &recur_args));
    if let Some(curr_def) = curr_def_o {
        let output: Vec<Judgement> = recur_args.iter().flatten().map(
            |x| x.clone()).collect();
        let known: Vec<Statement> = recur_args.iter().map(
            |arg_list| arg_list.last().unwrap().statement.clone()).collect();
        let last = Judgement {
            defs: vec![],
            context: context.to_vec(),
            statement: Statement {
                subject: CCExpression::Def(String::from(name.to_string()),
                                           args.to_vec()),
                s_type: do_type_sub(&curr_def.body.s_type, &curr_def,
                                    &known)
                }
        };
        return Ok(remove_dup(output.iter().chain(std::iter::once(&last))));
    }

    return Err(format!("failed to unpack Def term: {}", name));
}

fn unpack_abs(var: &str, v_type: &CCExpression, ret: &CCExpression,
                   context: &[Statement], defs: &[Definition]) -> Result<Vec<Judgement>, String> {
    let c_stmt = Statement {
        subject: CCExpression::Var(var.to_string()),
        s_type: v_type.clone()
    };

    let p1_r = unpack_term(ret,
                         &[context, &vec![c_stmt]].concat(),
                         defs);
    if let Err(msg) = p1_r { return Err(msg); }
    let p1 = p1_r.unwrap();

    let new_type = CCExpression::TypeAbs(
        String::from(var),
        Box::new(v_type.clone()),
        Box::new(p1.last().unwrap().statement.s_type.clone())
    );

    let p2_r = unpack_term(&new_type, context, defs);
    if let Err(msg) = p2_r { return Err(msg); }
    let p2 = p2_r.unwrap();

    let last = Judgement {
        defs: vec![],
        context: context.to_vec(),
        statement: Statement {
            subject: CCExpression::Abs(String::from(var),
                                       Box::new(v_type.clone()),
                                       Box::new(ret.clone())),
            s_type: new_type
        }
    };
    return Ok(remove_dup(p1.iter().chain(&p2).chain(std::iter::once(&last))));
}

fn unpack_appl(lhs: &CCExpression, rhs: &CCExpression,
                   context: &[Statement], defs: &[Definition]) -> Result<Vec<Judgement>, String> {
    let p1_r = unpack_term(lhs, context, defs);
    let p2_r = unpack_term(rhs, context, defs);

    if let Err(msg) = p1_r { return Err(msg); }
    if let Err(msg) = p2_r { return Err(msg); }

    let p1 = p1_r.unwrap();
    let p2 = p2_r.unwrap();

    let ret_type = p1.last().unwrap().statement.s_type.clone();
    if let CCExpression::TypeAbs(_x, _v_type, inner_ret) = ret_type {
        let last = Judgement {
            defs: vec![],
            context: context.to_vec(),
            statement: Statement {
                subject: CCExpression::Application(Box::new(lhs.clone()),
                                                   Box::new(rhs.clone())),
                s_type: *inner_ret
            }
        };

        return Ok(remove_dup(p2.iter().chain(&p1).chain(std::iter::once(&last))));
    } else {
        return Err(format!("failed to unpack Appl term: ({}) ({})", lhs.to_latex(), rhs.to_latex()));
    }
}

fn remove_dup<'a, T>(lst: T) -> Vec<Judgement>
where T: Iterator<Item= &'a Judgement>
{
    let mut seen: Vec<Judgement> = vec![];
    return lst.into_iter().filter_map(
        |x| if seen.contains(x) {
            return None;
        } else {
            seen.push(x.clone());
            return Some(x.clone());
        }
        ).collect();
}


pub fn unpack_term(term: &CCExpression, context: &[Statement], defs: &[Definition]) -> Result<Vec<Judgement>, String> {

    match term {
        CCExpression::Star => unpack_star(context),
        CCExpression::Sq => Err("Cannot unwrap Sq".to_string()),
        CCExpression::Prim => Err("Cannot unwrap Prim".to_string()),
        CCExpression::Var(x) => unpack_var(&x, context, defs),
        CCExpression::Def(name, args) => unpack_def(&name, args, context, defs),
        CCExpression::Abs(x, v_type, ret) => unpack_abs(&x, &v_type, &ret, context, defs),
        CCExpression::TypeAbs(x, v_type, ret) => unpack_type_abs(&x, &v_type, &ret, context, defs),
        CCExpression::Application(lhs, rhs) => unpack_appl(&lhs, &rhs, context, defs)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement, parse_definition};
    use crate::type_check::{check_proof};

    #[test]
    fn simple_unpack() {
        let jdg: Judgement = parse_judgement("A:\\ast,a:A \\vdash a:A").unwrap();
        assert_eq!(jdg.to_latex(), "A : \\ast, a : A \\vdash a : A");
        let lines = unpack_term(&jdg.statement.subject, &jdg.context, &[]).unwrap();

        assert_eq!(lines.len(), 3);

        let str_lines: Vec<String> = lines.iter().map(
            |x| x.to_latex()
            ).collect();
        assert_eq!(str_lines, [
                   "\\vdash \\ast : \\square",
                   "A : \\ast \\vdash A : \\ast",
                   "A : \\ast, a : A \\vdash a : A"
        ]);
        let refs = check_proof(&[], &lines).unwrap();
        assert_eq!(lines.len(), refs.len());
    }

    #[test]
    fn type_abs_unpack() {
        let jdg: Judgement = parse_judgement("A:\\ast \\vdash \\prod x : A . A : \\ast").unwrap();
        let lines = unpack_term(&jdg.statement.subject, &jdg.context, &[]).unwrap();

        let str_lines: Vec<String> = lines.iter().map(
            |x| x.to_latex()
            ).collect();
        assert_eq!(str_lines, [
                   "\\vdash \\ast : \\square",
                   "A : \\ast \\vdash A : \\ast",
                   "A : \\ast, x : A \\vdash A : \\ast",
                   "A : \\ast \\vdash A \\to A : \\ast"
        ]);
        let refs = check_proof(&[], &lines).unwrap();
        assert_eq!(lines.len(), refs.len());
    }

    #[test]
    fn abs_unpack() {
        let jdg: Judgement = parse_judgement("A:\\ast \\vdash \\lambda x : A . x : \\ast").unwrap();
        let lines = unpack_term(&jdg.statement.subject, &jdg.context, &[]).unwrap();

        let str_lines: Vec<String> = lines.iter().map(
            |x| x.to_latex()
            ).collect();
        assert_eq!(str_lines, [
                   "\\vdash \\ast : \\square",
                   "A : \\ast \\vdash A : \\ast",
                   "A : \\ast, x : A \\vdash x : A",
                   "A : \\ast, x : A \\vdash A : \\ast",
                   "A : \\ast \\vdash A \\to A : \\ast",
                   "A : \\ast \\vdash \\lambda x : A . x : A \\to A"
        ]);
        let refs = check_proof(&[], &lines).unwrap();
        assert_eq!(lines.len(), refs.len());
    }

    #[test]
    fn appl_unpack() {
        let jdg: Judgement = parse_judgement("A:\\ast, y:A \\vdash (\\lambda x : A . x) y : \\ast").unwrap();
        let lines = unpack_term(&jdg.statement.subject, &jdg.context, &[]).unwrap();

        let str_lines: Vec<String> = lines.iter().map(
            |x| x.to_latex()
            ).collect();
        assert_eq!(str_lines, [
                   "\\vdash \\ast : \\square",
                   "A : \\ast \\vdash A : \\ast",
                   "A : \\ast, y : A \\vdash y : A",
                   "A : \\ast, y : A \\vdash A : \\ast",
                   "A : \\ast, y : A, x : A \\vdash x : A",
                   "A : \\ast, y : A, x : A \\vdash A : \\ast",
                   "A : \\ast, y : A \\vdash A \\to A : \\ast",
                   "A : \\ast, y : A \\vdash \\lambda x : A . x : A \\to A",
                   "A : \\ast, y : A \\vdash (\\lambda x : A . x) y : A"
        ]);
        let refs = check_proof(&[], &lines).unwrap();
        assert_eq!(lines.len(), refs.len());
    }

    #[test]
    fn inst_unpack() {
        let defs = vec![parse_definition(
            "A : \\ast \\vartriangleright id \\langle A \\rangle := \\lambda x : A . x : A \\to A"
            ).unwrap()];
        let jdg: Judgement = parse_judgement(
            "I:\\ast, q:I \\vdash (id \\langle I \\rangle) q : I").unwrap();
        let lines = unpack_term(&jdg.statement.subject, &jdg.context, &defs).unwrap();
        let str_lines: Vec<String> = lines.iter().map(
            |x| x.to_latex()
            ).collect();
        assert_eq!(str_lines,
                   ["\\vdash \\ast : \\square",
                   "I : \\ast \\vdash I : \\ast",
                   "I : \\ast, q : I \\vdash q : I",
                   "I : \\ast, q : I \\vdash I : \\ast",
                   "I : \\ast, q : I \\vdash id \\langle I \\rangle : I \\to I",
                   "I : \\ast, q : I \\vdash id \\langle I \\rangle q : I"]);
        let refs = check_proof(&defs, &lines).unwrap();
        assert_eq!(lines.len(), refs.len());
    }
}

