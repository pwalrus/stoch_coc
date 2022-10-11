
use super::expression::{CCExpression};

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct Statement {
    pub subject: CCExpression,
    pub s_type: CCExpression
}

impl Statement {

    pub fn to_latex(&self) -> String {
        return self.subject.to_latex() + " : " + &self.s_type.to_latex()
    }

    pub fn alpha_equiv(&self, rhs: &Statement) -> bool {
        return self.subject.alpha_equiv(&rhs.subject);
    }

    pub fn primative(&self) -> bool {
        return self.subject.primative();
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
        assert_eq!(stmt.primative(), false);
    }
}
