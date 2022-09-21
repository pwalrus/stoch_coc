use crate::model::judgement::{Judgement};
use crate::model::statement::{Statement};
use crate::model::expression::{CCExpression};

fn unpack_star(_: &[Statement]) -> Vec<Judgement> {
    let stmt = Statement {
        subject: CCExpression::Star,
        s_type: CCExpression::Sq
    };

    return vec![Judgement {
        defs: vec![],
        statement: stmt,
        context: vec![]
    }];
}


fn unpack_var(var: &str, context: &[Statement]) -> Vec<Judgement> {
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

        return [unpack_term(&stmt.s_type, &ctx2), vec![Judgement {
            defs: vec![],
            statement: stmt,
            context: context.to_vec()
        }]].concat();
    }
    return vec![];
}

fn unpack_type_abs(var: &str, v_type: &CCExpression, ret: &CCExpression,
                   context: &[Statement]) -> Vec<Judgement> {
    let p1: Vec<Judgement> = unpack_term(v_type, context);
    let stmt = Statement {
        subject: CCExpression::Var(var.to_string()),
        s_type: v_type.clone()
    };
    let new_ctx = [context, &vec![stmt]].concat();
    let p2: Vec<Judgement> = unpack_term(ret, &new_ctx);
    if p1.len() == 0 || p2.len() == 0 {
        return vec![];
    }
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

    return remove_dup(p1.iter().chain(p2.iter())
                      .chain(std::iter::once(&last)));
}

fn unpack_abs(var: &str, v_type: &CCExpression, ret: &CCExpression,
                   context: &[Statement]) -> Vec<Judgement> {
    let c_stmt = Statement {
        subject: CCExpression::Var(var.to_string()),
        s_type: v_type.clone()
    };

    let p1: Vec<Judgement> = unpack_term(ret,
                                         &[context, &vec![c_stmt]].concat());
    if p1.len() == 0 { return vec![]; }

    let new_type = CCExpression::TypeAbs(
        String::from(var),
        Box::new(v_type.clone()),
        Box::new(p1.last().unwrap().statement.s_type.clone())
    );

    let p2: Vec<Judgement> = unpack_term(&new_type, context);
    if p2.len() == 0 { return vec![]; }

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
    return remove_dup(p1.iter().chain(&p2).chain(std::iter::once(&last)));
}

fn unpack_appl(lhs: &CCExpression, rhs: &CCExpression,
                   context: &[Statement]) -> Vec<Judgement> {
    let p1: Vec<Judgement> = unpack_term(lhs, context);
    let p2: Vec<Judgement> = unpack_term(rhs, context);

    if p1.len() == 0 || p2.len() == 0 { return vec![]; }
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

        return remove_dup(p2.iter().chain(&p1).chain(std::iter::once(&last)));
    } else {
        return vec![];
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


pub fn unpack_term(term: &CCExpression, context: &[Statement]) -> Vec<Judgement> {

    match term {
        CCExpression::Star => unpack_star(context),
        CCExpression::Sq => vec![],
        CCExpression::Prim => vec![],
        CCExpression::Var(x) => unpack_var(&x, context),
        CCExpression::Def(name, args) => todo!(),
        CCExpression::Abs(x, v_type, ret) => unpack_abs(&x, &v_type, &ret, context),
        CCExpression::TypeAbs(x, v_type, ret) => unpack_type_abs(&x, &v_type, &ret, context),
        CCExpression::Application(lhs, rhs) => unpack_appl(&lhs, &rhs, context)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};
    use crate::type_check::{check_proof};

    #[test]
    fn simple_unpack() {
        let jdg: Judgement = parse_judgement("A:\\ast,a:A \\vdash a:A").unwrap();
        assert_eq!(jdg.to_latex(), "A : \\ast, a : A \\vdash a : A");
        let lines = unpack_term(&jdg.statement.subject, &jdg.context);

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
        let lines = unpack_term(&jdg.statement.subject, &jdg.context);

        let str_lines: Vec<String> = lines.iter().map(
            |x| x.to_latex()
            ).collect();
        assert_eq!(str_lines, [
                   "\\vdash \\ast : \\square",
                   "A : \\ast \\vdash A : \\ast",
                   "A : \\ast, x : A \\vdash A : \\ast",
                   "A : \\ast \\vdash \\prod x : A . A : \\ast"
        ]);
        let refs = check_proof(&[], &lines).unwrap();
        assert_eq!(lines.len(), refs.len());
    }

    #[test]
    fn abs_unpack() {
        let jdg: Judgement = parse_judgement("A:\\ast \\vdash \\lambda x : A . x : \\ast").unwrap();
        let lines = unpack_term(&jdg.statement.subject, &jdg.context);

        let str_lines: Vec<String> = lines.iter().map(
            |x| x.to_latex()
            ).collect();
        assert_eq!(str_lines, [
                   "\\vdash \\ast : \\square",
                   "A : \\ast \\vdash A : \\ast",
                   "A : \\ast, x : A \\vdash x : A",
                   "A : \\ast, x : A \\vdash A : \\ast",
                   "A : \\ast \\vdash \\prod x : A . A : \\ast",
                   "A : \\ast \\vdash \\lambda x : A . x : \\prod x : A . A"
        ]);
        let refs = check_proof(&[], &lines).unwrap();
        assert_eq!(lines.len(), refs.len());
    }

    #[test]
    fn appl_unpack() {
        let jdg: Judgement = parse_judgement("A:\\ast, y:A \\vdash (\\lambda x : A . x) y : \\ast").unwrap();
        let lines = unpack_term(&jdg.statement.subject, &jdg.context);

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
                   "A : \\ast, y : A \\vdash \\prod x : A . A : \\ast",
                   "A : \\ast, y : A \\vdash \\lambda x : A . x : \\prod x : A . A",
                   "A : \\ast, y : A \\vdash (\\lambda x : A . x) y : A"
        ]);
        let refs = check_proof(&[], &lines).unwrap();
        assert_eq!(lines.len(), refs.len());
    }
}

