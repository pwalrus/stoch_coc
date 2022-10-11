
use super::statement::{Statement};
use super::expression::{CCExpression};

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct Definition {
    pub context: Vec<Statement>,
    pub name: String,
    pub args: Vec<String>,
    pub body: Statement
}

impl Definition {
    pub fn to_latex(&self) -> String {
        let ctx_str: String = self.context.iter().filter_map(
            |x| Some(x.to_latex())
            ).collect::<Vec<String>>().join(", ");
        let args_str: String = self.args.join(", ");
        return format!("{} \\vartriangleright {} \\langle {} \\rangle := {}",
        ctx_str,
        self.name,
        args_str,
        self.body.to_latex()
        );
    }

    pub fn type_list(&self) -> Option<Vec<CCExpression>> {
        let output: Vec<CCExpression> = self.args.iter().filter_map(
            |x| self.context.iter().find(
                |stmt| stmt.subject.var_str() == Some(x.to_string()))
            ).map(|x| x.s_type.clone()).collect();
        if output.len() == self.args.len() {
            return Some(output);
        } else {
            return None;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::expression::CCExpression;

    #[test]
    fn to_latex_simple_def() {
        let stmt1 = Statement {
            subject: CCExpression::Var("x".to_string()),
            s_type: CCExpression::Var("A".to_string()),
        };
        let def1 = Definition {
            context: vec![stmt1.clone()],
            name: "ex".to_string(),
            args: vec!{"x".to_string()},
            body: stmt1
        };

        assert_eq!(def1.to_latex(),
                   "x : A \\vartriangleright ex \\langle x \\rangle := x : A");
    }
}


