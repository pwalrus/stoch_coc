
pub enum CCExpression {
    Var(String),
    Sq,
    Star,
    Application(Box<CCExpression>, Box<CCExpression>),
    Abs(String, Box<CCExpression>, Box<CCExpression>),
    TypeAbs(String, Box<CCExpression>, Box<CCExpression>),
}


impl CCExpression {

    pub fn to_latex(&self) -> String {
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

    pub fn sub_terms(&self) -> Vec<CCExpression> {
        match self {
            CCExpression::Application(left, right) => {
                [vec![self.clone()], left.sub_terms(),
                right.sub_terms()].concat()
            }
            CCExpression::Abs(_, _, ret) => {
                [vec![self.clone()], ret.sub_terms()].concat()
            }
            CCExpression::TypeAbs(_, _, ret) => {
                [vec![self.clone()], ret.sub_terms()].concat()
            }
            _other => vec![self.clone()],
        }
    }

    pub fn free_var(&self) -> Vec<String> {
        match self {
            CCExpression::Var(x) => vec![x.clone()],
            CCExpression::Application(left, right) => {
                [left.free_var(), right.free_var()].concat()
            }
            CCExpression::Abs(arg, _, ret) => {
                ret.free_var().into_iter().filter(
                    |x| x != arg
                    ).collect()
            }
            _other => vec![],
        }

    }

}

impl Clone for CCExpression {

    fn clone(&self) -> Self {
        match self {
            CCExpression::Var(x) => CCExpression::Var(x.clone()),
            CCExpression::Sq => CCExpression::Sq,
            CCExpression::Star => CCExpression::Star,
            CCExpression::Application(left, right) => {
                CCExpression::Application(left.clone(), 
                                          right.clone())
            }
            CCExpression::Abs(arg, t, ret) => {
                CCExpression::Abs(arg.clone(),
                                  t.clone(), 
                                  ret.clone())
            }
            CCExpression::TypeAbs(arg, t, ret) => {
                CCExpression::TypeAbs(arg.clone(),
                                      t.clone(),
                                      ret.clone())
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

    #[test]
    fn sub_terms_simple() {
        let expr1 = CCExpression::Var(String::from("A"));
        let terms: Vec<String> = expr1.sub_terms().iter().map(|x| x.to_latex()).collect();
        assert_eq!(terms, vec![
                   String::from("A")
        ]);
    }

    #[test]
    fn sub_terms_bigger() {
        let expr1 = CCExpression::Var(String::from("A"));
        let expr2 = CCExpression::Var(String::from("banana"));
        let expr3 = CCExpression::Var(String::from("apple"));
        let expr4 = CCExpression::Abs(String::from("x"), 
                                      Box::new(expr1),
                                      Box::new(expr3));
        let expr5 = CCExpression::Application(Box::new(expr4), 
                                              Box::new(expr2));
        let terms: Vec<String> = expr5.sub_terms().iter().map(|x| x.to_latex()).collect();
        assert_eq!(terms, vec![
                   String::from("\\lambda x : A . apple banana"),
                   String::from("\\lambda x : A . apple"),
                   String::from("apple"),
                   String::from("banana")
        ]);
    }

    #[test]
    fn free_var_simple() {
        let expr1 = CCExpression::Var(String::from("A"));
        let terms: Vec<String> = expr1.free_var();
        assert_eq!(terms, vec![
                   String::from("A")
        ]);
    }

    #[test]
    fn free_var_bigger() {
        let expr1 = CCExpression::Var(String::from("A"));
        let expr2 = CCExpression::Var(String::from("banana"));
        let expr3 = CCExpression::Var(String::from("apple"));
        let expr4 = CCExpression::Abs(String::from("x"), 
                                      Box::new(expr1),
                                      Box::new(expr3));
        let expr5 = CCExpression::Application(Box::new(expr4), 
                                              Box::new(expr2));
        let terms: Vec<String> = expr5.free_var();
        assert_eq!(terms, vec![
                   String::from("apple"),
                   String::from("banana")
        ]);
    }
}
