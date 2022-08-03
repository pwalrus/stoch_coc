
use super::expression::{CCExpression};

struct Statement {
    subject: CCExpression,
    s_type: CCExpression
}

impl Statement {

    fn to_latex(&self) -> String {
        return self.subject.to_latex() + " : " + &self.s_type.to_latex()
    }
}

struct Judgement {
    context: Vec<Statement>,
    statement: Statement
}

impl Judgement {

    fn to_latex(&self) -> String {
        let output = self.context.iter().map(
                |x| x.to_latex()
            ).reduce(
                |a, b| a + ", " + &b
            );
           
        match output {
            Some(x) => x + " \\vdash " + &self.statement.to_latex(), 
            None => String::from("")
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_latex_simple_stmt() {
        let expr1 = CCExpression::Var(String::from("banana"));
        let expr2 = CCExpression::Var(String::from("A"));
        let stmt = Statement { subject: expr1, s_type: expr2 };
        assert_eq!(stmt.to_latex(), String::from("banana : A"));
    }

    #[test]
    fn to_latex_simple_judgement() {
        let expr1 = CCExpression::Var(String::from("banana"));
        let expr2 = CCExpression::Var(String::from("A"));
        let stmt1 = Statement { subject: expr1, s_type: expr2 };
        let expr3 = CCExpression::Var(String::from("orange"));
        let expr4 = CCExpression::Var(String::from("B"));
        let stmt2 = Statement { subject: expr3, s_type: expr4 };
        let expr5 = CCExpression::Var(String::from("potato"));
        let expr6 = CCExpression::Var(String::from("C"));
        let stmt3 = Statement { subject: expr5, s_type: expr6 };
        let judge = Judgement {
            context: vec![stmt1, stmt2],
            statement: stmt3
        };
        assert_eq!(judge.to_latex(), String::from(
                "banana : A, orange : B \\vdash potato : C"
                ));
    }
}
