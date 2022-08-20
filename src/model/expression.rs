
#[derive(Debug,PartialEq)]
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
                if let CCExpression::Var(_) = **right {
                    left.to_latex() + " " + &right.to_latex()
                } else {
                    left.to_latex() + " (" + &right.to_latex() + ")"
                }
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

    pub fn is_sort(&self) -> bool {
        match self {
            CCExpression::Star => true,
            CCExpression::Sq => true,
            _ => false,
        }
    }

    pub fn substitute(&self, token: &str, expr: &CCExpression) -> CCExpression {
        match self {
            CCExpression::Star => CCExpression::Star,
            CCExpression::Sq => CCExpression::Sq,
            CCExpression::Var(x) => {
                if x == token { expr.clone() }
                else { CCExpression::Var(x.clone()) }
            },
            CCExpression::Abs(x, a_type, ret) => {
                if x == token { CCExpression::Abs(
                        String::from(x), 
                        a_type.clone(), 
                        ret.clone()) }
                else { CCExpression::Abs(
                        String::from(x), 
                        Box::new(a_type.substitute(token, expr)),
                        Box::new(ret.substitute(token, expr))
                        ) }
            },
            CCExpression::TypeAbs(x, a_type, ret) => {
                if x == token { CCExpression::TypeAbs(
                        String::from(x), 
                        a_type.clone(),
                        ret.clone()
                        ) }
                else { CCExpression::TypeAbs(
                        String::from(x), 
                        Box::new(a_type.substitute(token, expr)),
                        Box::new(ret.substitute(token, expr))
                        ) } 
            }
            CCExpression::Application(lhs, rhs) => {
                CCExpression::Application(
                    Box::new(lhs.substitute(token, expr)),
                    Box::new(rhs.substitute(token, expr))
                    )
            }
        }
    }

    pub fn alpha_equiv(&self, rhs: &CCExpression) -> bool {
        match (self, rhs) {
            (CCExpression::Star, CCExpression::Star) => true,
            (CCExpression::Sq, CCExpression::Sq) => true,
            (CCExpression::Var(l), CCExpression::Var(r)) => l == r,
            (CCExpression::Application(q, r), 
            CCExpression::Application(x, y)) => {
                return q.alpha_equiv(x) && r.alpha_equiv(y)
            },
            (CCExpression::Abs(x, a_type1, ret1), 
            CCExpression::Abs(y, a_type2, ret2)) => {
                if x == y {
                    return a_type1.alpha_equiv(a_type2) && ret1.alpha_equiv(ret2);
                } else {
                    let new_type = a_type2.substitute(y, &CCExpression::Var(x.clone()));
                    let new_ret = ret2.substitute(y, &CCExpression::Var(x.clone()));
                    return a_type1.alpha_equiv(&new_type) && ret1.alpha_equiv(&new_ret);
                }
            },
            (CCExpression::TypeAbs(x, a_type1, ret1), 
            CCExpression::TypeAbs(y, a_type2, ret2)) => {
                if x == y {
                    return a_type1.alpha_equiv(a_type2) && ret1.alpha_equiv(ret2);
                } else {
                    let new_type = a_type2.substitute(y, &CCExpression::Var(x.clone()));
                    let new_ret = ret2.substitute(y, &CCExpression::Var(x.clone()));
                    return a_type1.alpha_equiv(&new_type) && ret1.alpha_equiv(&new_ret);
                }
            },
            (_, _) => false
        }
    }

    pub fn beta_equiv(&self, rhs: &CCExpression) -> bool {
        return self.alpha_equiv(rhs) || self == rhs
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

    #[test]
    fn is_sort_test() {
        assert!(!CCExpression::Var(String::from("X")).is_sort());
        assert!(CCExpression::Star.is_sort());
    }

    #[test]
    fn substitute1() {
        let expr1 = CCExpression::Var(String::from("x"));
        let expr2 = CCExpression::Var(String::from("y"));
        let expr3 = CCExpression::Application(
            Box::new(expr1.clone()),
            Box::new(expr2.clone())
            );
        let expr4 = CCExpression::Var(String::from("z"));
        assert_eq!(
            &expr1.substitute("x", &expr4).to_latex(),
            "z"
            );
        assert_eq!(
            &expr2.substitute("x", &expr4).to_latex(),
            "y"
            );
        assert_eq!(
            &expr3.substitute("x", &expr4).to_latex(),
            "z y"
            );
    }
}
