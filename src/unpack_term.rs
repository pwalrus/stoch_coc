use crate::model::judgement::{Judgement, Statement};
use crate::model::expression::{CCExpression};

fn unpack_star(context: &[Statement]) -> Vec<Judgement> {
    let stmt = Statement {
        subject: CCExpression::Star,
        s_type: CCExpression::Sq
    };

    return vec![Judgement {
        statement: stmt,
        context: context.to_vec()
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
            statement: stmt,
            context: context.to_vec()
        }]].concat();
    }
    return vec![];
}

pub fn unpack_term(term: &CCExpression, context: &[Statement]) -> Vec<Judgement> {

    match term {
        CCExpression::Star => unpack_star(context),
        CCExpression::Sq => vec![],
        CCExpression::Var(x) => unpack_var(&x, context),
        CCExpression::Abs(_, _, _) => vec![],
        CCExpression::TypeAbs(_, _, _) => vec![],
        CCExpression::Application(_, _) => vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};

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
    }
}

