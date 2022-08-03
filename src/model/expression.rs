

enum CCExpression {
    Var(String),
    Sq,
    Star,
    Application(Box<CCExpression>, Box<CCExpression>),
    Abs(String, Box<CCExpression>, Box<CCExpression>),
    TypeAbs(String, Box<CCExpression>, Box<CCExpression>),
}


impl CCExpression {

    fn to_latex(&self) -> String {
        match self {
            CCExpression::Var(x) => x.to_string(),
            CCExpression::Sq => String::from("\\square"),
            CCExpression::Star => String::from("\\ast"),
            CCExpression::Application(left, right) => {
                left.to_latex() + " " + &right.to_latex()
            }
            CCExpression::Abs(arg, t, ret) => {
                String::from("\\lambda ") + arg + " : " + &t.to_latex()
                + " . " + &ret.to_latex()
            }
            CCExpression::TypeAbs(arg, t, ret) => {
                String::from("\\prod ") + arg + " : " + &t.to_latex()
                + " . " + &ret.to_latex()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_latex_simple_var() {
        let expr1 = CCExpression::Var(String::from("banana"));
        assert_eq!(expr1.to_latex(), "banana");
    }

    #[test]
    fn to_latex_simple_square() {
        let expr1 = CCExpression::Sq;
        assert_eq!(expr1.to_latex(), "\\square");
    }

    #[test]
    fn to_latex_simple_star() {
        let expr1 = CCExpression::Star;
        assert_eq!(expr1.to_latex(), "\\ast");
    }

    #[test]
    fn to_latex_simple_appl() {
        let expr1 = CCExpression::Var(String::from("apple"));
        let expr2 = CCExpression::Var(String::from("orange"));
        let expr3 = CCExpression::Application(Box::new(expr1), 
                                              Box::new(expr2));
        assert_eq!(expr3.to_latex(), "apple orange");
    }

    #[test]
    fn to_latex_simple_abs() {
        let expr1 = String::from("potato");
        let expr2 = CCExpression::Var(String::from("A"));
        let expr3 = CCExpression::Var(String::from("avocado"));
        let expr4 = CCExpression::Abs(expr1, 
                                      Box::new(expr2), 
                                      Box::new(expr3));
        assert_eq!(expr4.to_latex(), "\\lambda potato : A . avocado");
    }

    #[test]
    fn to_latex_simple_type_abs() {
        let expr1 = String::from("potato");
        let expr2 = CCExpression::Var(String::from("A"));
        let expr3 = CCExpression::Var(String::from("avocado"));
        let expr4 = CCExpression::TypeAbs(expr1, 
                                          Box::new(expr2), 
                                          Box::new(expr3));
        assert_eq!(expr4.to_latex(), "\\prod potato : A . avocado");
    }
}
