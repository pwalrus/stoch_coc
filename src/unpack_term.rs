use crate::model::judgement::{Judgement, Statement};
use crate::model::expression::{CCExpression};

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
        return vec![Judgement {
            statement: stmt,
            context: context.to_vec()
        }];
    }
    return vec![];
}

pub fn unpack_term(term: &CCExpression, context: &[Statement]) -> Vec<Judgement> {

    match term {
        CCExpression::Star => vec![],
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
        let jdg: Judgement = parse_judgement("a:A \\vdash a:A").unwrap();
        assert_eq!(jdg.to_latex(), "a : A \\vdash a : A");
        let lines = unpack_term(&jdg.statement.subject, &jdg.context);

        assert_eq!(lines.len(), 1);
        
        let str_lines: Vec<String> = lines.iter().map(
            |x| x.to_latex()
            ).collect();
        assert_eq!(str_lines, [
                   "a : A \\vdash a : A"
        ]);
    }
}

