
use super::judgement::{Statement};

#[derive(PartialEq,Debug)]
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
        return format!("{} \\vartriangleright {}({}) := {}",
        ctx_str,
        self.name,
        args_str,
        self.body.to_latex()
        );
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
                   "x : A \\vartriangleright ex(x) := x : A");
    }
}


