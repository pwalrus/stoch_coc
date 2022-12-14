
#[derive(Debug,PartialEq,Eq)]
pub enum CCExpression {
    Var(String),
    Sq,
    Star,
    Prim,
    Def(String, Vec<CCExpression>),
    Application(Box<CCExpression>, Box<CCExpression>),
    Abs(String, Box<CCExpression>, Box<CCExpression>),
    TypeAbs(String, Box<CCExpression>, Box<CCExpression>),
}


fn type_abs_to_latex(ex: &CCExpression, arg: &String, t: &CCExpression,
                     ret: &CCExpression) -> String {
    if let Some((a, b)) = ex.is_or() {
        let lhs = if a.is_or().is_some() || a.unbracketed() { a.to_latex()
        } else { format!("({})", a.to_latex()) };
        let rhs = if b.is_or().is_some() || b.unbracketed() { b.to_latex()
        } else { format!("({})", b.to_latex()) };
        format!("{} \\vee {}", lhs, rhs)
    } else if let Some((a, b)) = ex.is_and() {
        let lhs = if a.is_and().is_some() || a.unbracketed() { a.to_latex()
        } else { format!("({})", a.to_latex()) };
        let rhs = if b.is_and().is_some() || b.unbracketed() { b.to_latex()
        } else { format!("({})", b.to_latex()) };
        format!("{} \\wedge {}", lhs, rhs)
    } else if let Some(x) = ex.is_neg() {
        return if x.unbracketed() {
            format!("\\neg {}", x.to_latex())
        } else {
            format!("\\neg ({})", x.to_latex())
        }
    } else if let Some((t1, ret1)) = ex.is_arrow() {
        let lhs = match t1 {
            CCExpression::Var(_) => t1.to_latex(),
            CCExpression::TypeAbs(_,_,_) => {
                if t1.unbracketed() {
                    t1.to_latex()
                } else {
                    format!("({})", t1.to_latex())
                }
            }
            _ => format!("({})", t1.to_latex())
        };
        lhs + " \\to " + &ret1.to_latex()
    } else if ex.is_contradiction() {
        "\\perp".to_string()
    } else {
        String::from("\\prod ") + arg + " : " + &t.to_latex()
        + " . " + &ret.to_latex()
    }
}


impl CCExpression {

    pub fn to_latex(&self) -> String {
        match self {
            CCExpression::Var(x) => x.to_string(),
            CCExpression::Sq => String::from("\\square"),
            CCExpression::Star => String::from("\\ast"),
            CCExpression::Prim => String::from("\\independent"),
            CCExpression::Def(name, args) => {
                let arg_list = args.iter().filter_map(
                    |x| Some(x.to_latex())
                    ).collect::<Vec<String>>().join(", ");
                format!("{} \\langle {} \\rangle", name, arg_list)
            },
            CCExpression::Application(left, right) => {
                let r_out = if right.unbracketed() {
                    right.to_latex()
                } else {
                    String::from("(") + &right.to_latex() + ")"
                };
                let l_out = match **left {
                    CCExpression::Var(_) => left.to_latex(),
                    CCExpression::Application(_, _) => left.to_latex(),
                    CCExpression::Def(_, _) => left.to_latex(),
                    CCExpression::TypeAbs(_,_,_) => {
                        if left.unbracketed() {
                            left.to_latex()
                        } else {
                            String::from("(") + &left.to_latex() + ")"
                        }
                    }
                    _ => String::from("(") + &left.to_latex() + ")"
                };
                l_out + " " + &r_out
            }
            CCExpression::Abs(arg, t, ret) => {
                String::from("\\lambda ") + arg + " : " + &t.to_latex()
                + " . " + &ret.to_latex()
            }
            CCExpression::TypeAbs(arg, t, ret) => type_abs_to_latex(self, &arg, &t, &ret)
        }
    }

    pub fn is_contradiction(&self) -> bool {
        match self {
            CCExpression::TypeAbs(arg, t, ret) => {
                match **t {
                    CCExpression::Star => {
                        if let Some(x) = ret.var_str() {
                            if x == *arg {
                                return true;
                            }
                        }
                        false
                    },
                    _ => false
                }
            },
            _ => false
        }
    }

    pub fn arrow_chain(&self) -> Vec<&CCExpression> {
        match self.is_arrow() {
            None => vec![&self],
            Some((lhs, rhs)) => [vec![lhs], rhs.arrow_chain()].concat()
        }
    }

    pub fn is_arrow(&self) -> Option<(&CCExpression, &CCExpression)> {
        match self {
            CCExpression::TypeAbs(arg, t, ret) => {
                if ret.free_var().contains(arg) { None }
                else { Some((&t, &ret)) }
            }
            _ => None
        }
    }

    pub fn is_neg(&self) -> Option<&CCExpression> {
        if let Some((lhs, rhs)) = self.is_arrow() {
            if rhs.is_contradiction() {
                return Some(lhs);
            }
        }
        return None;
    }

    pub fn is_or(&self) -> Option<(&CCExpression, &CCExpression)> {
        match self {
            CCExpression::TypeAbs(arg, t, ret) => {
                let top_arrow = ret.is_arrow();
                match (*t.clone(), top_arrow) {
                    (CCExpression::Star, Some((lhs, rhs))) => {
                        let lhs_a = lhs.is_arrow();
                        let rhs_a = rhs.is_arrow();
                        match (lhs_a, rhs_a) {
                            (Some((l_arg, ret1)), Some((mid_arrow, ret2))) => {
                                let mid_a = mid_arrow.is_arrow();
                                match mid_a {
                                    Some((r_arg, ret3)) => {
                                        let ret_cmp = CCExpression::Var(arg.to_string());
                                        if *ret1 == ret_cmp && *ret2 == ret_cmp && *ret3 == ret_cmp {
                                            return Some((&l_arg, &r_arg));
                                        }
                                        return None;
                                    },
                                    _ => { return None; }
                                }
                            },
                            _ => { return None; }
                        }
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
    }

    pub fn is_and(&self) -> Option<(&CCExpression, &CCExpression)> {
        match self {
            CCExpression::TypeAbs(arg, t, ret) => {
                let top_arrow = ret.is_arrow();
                match (*t.clone(), top_arrow) {
                    (CCExpression::Star, Some((seq, ret1))) => {
                        let first_a = seq.is_arrow();
                        let ret_cmp = CCExpression::Var(arg.to_string());
                        if *ret1 != ret_cmp { return None; }
                        match first_a {
                            Some((lhs, mid_arrow)) => {
                                let mid_a = mid_arrow.is_arrow();
                                match mid_a {
                                    Some((rhs, ret2)) => {
                                        if *ret2 == ret_cmp {
                                            return Some((lhs, rhs));
                                        } else { return None; }
                                    },
                                    _ => { return None; }
                                }
                            },
                            _ => { return None; }
                        }
                    },
                    _ => None
                }
            },
            _ => { return None; }
        }
    }
    pub fn var_str(&self) -> Option<String> {
        match self {
            CCExpression::Var(x) => Some(x.to_string()),
            _ => None
        }
    }

    pub fn primative(&self) -> bool {
        match self {
            CCExpression::Prim => true,
            _ => false
        }
    }

    pub fn unbracketed(&self) -> bool {
        match self {
            CCExpression::Var(_) => true,
            CCExpression::Star => true,
            CCExpression::Sq => true,
            CCExpression::Def(_, _) => true,
            _ => {
                if self.is_contradiction() {
                    true
                } else if let Some(_) = self.is_neg() {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn sub_terms(&self) -> Vec<CCExpression> {
        match self {
            CCExpression::Def(_, args) => {
                [vec![self.clone()], args.clone()].concat()
            },
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
            CCExpression::Abs(arg, t, ret) => {
                ret.free_var().iter().chain(&t.free_var()).filter(
                    |x| *x != arg
                    ).map(|x| x.to_string()).collect()
            },
            CCExpression::TypeAbs(arg, t, ret) => {
                ret.free_var().iter().chain(&t.free_var()).filter(
                    |x| *x != arg
                    ).map(|x| x.to_string()).collect()
            }
            _other => vec![],
        }

    }

    pub fn is_var(&self) -> bool {
        match self {
            CCExpression::Var(_) => true,
            _ => false,
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
            CCExpression::Prim => CCExpression::Prim,
            CCExpression::Def(name, args) => {
                CCExpression::Def(
                    name.clone(),
                    args.iter().filter_map(
                        |x| Some(x.substitute(token, expr))
                        ).collect()
                    )
            },
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
            (CCExpression::Prim, CCExpression::Prim) => true,
            (CCExpression::Def(lname, largs), CCExpression::Def(rname, rargs)) => {
                let names_match: bool = lname == rname;
                let args_match: bool = largs.iter().zip(rargs).all(
                    |(l, r)| l.alpha_equiv(r));
                names_match && args_match
            },
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

    pub fn beta_reduce(&self) -> CCExpression {
        match self {
            CCExpression::Star => CCExpression::Star,
            CCExpression::Sq => CCExpression::Sq,
            CCExpression::Prim => CCExpression::Prim,
            CCExpression::Var(x) => CCExpression::Var(x.clone()),
            CCExpression::Application(lhs, rhs) => {
                let l = lhs.beta_reduce();
                let r = rhs.beta_reduce();
                if let CCExpression::Abs(arg, _, ret) = l {
                    return ret.substitute(&arg, &r);
                }
                return CCExpression::Application(
                    Box::new(l),
                    Box::new(r)
                    );
            },
            other => other.clone()
        }
    }

    pub fn beta_equiv(&self, rhs: &CCExpression) -> bool {
        return self.beta_reduce().alpha_equiv(&rhs.beta_reduce());
    }
}

impl Clone for CCExpression {

    fn clone(&self) -> Self {
        match self {
            CCExpression::Var(x) => CCExpression::Var(x.clone()),
            CCExpression::Sq => CCExpression::Sq,
            CCExpression::Star => CCExpression::Star,
            CCExpression::Prim => CCExpression::Prim,
            CCExpression::Def(name, args) => {
                CCExpression::Def(name.clone(), args.clone())
            }
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
        assert_eq!(expr1.primative(), false);
        assert_eq!(expr1.var_str(), Some("banana".to_string()))
    }

    #[test]
    fn to_latex_simple_square() {
        let expr1 = CCExpression::Sq;
        assert_eq!(expr1.to_latex(), "\\square");
        assert_eq!(expr1.var_str(), None);
    }

    #[test]
    fn to_latex_simple_star() {
        let expr1 = CCExpression::Star;
        assert_eq!(expr1.to_latex(), "\\ast");
    }

    #[test]
    fn to_latex_simple_prim() {
        let expr1 = CCExpression::Prim;
        assert_eq!(expr1.to_latex(), "\\independent");
        assert_eq!(expr1.primative(), true)
    }

    #[test]
    fn to_latex_simple_def() {
        let arg1 = CCExpression::Var(String::from("a"));
        let arg2 = CCExpression::Var(String::from("b"));
        let expr1 = CCExpression::Def(
            "ex".to_string(),
            vec![arg1, arg2]
            );

        assert_eq!(expr1.to_latex(), "ex \\langle a, b \\rangle");
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
                                          Box::new(expr2.clone()),
                                          Box::new(expr3.clone()));
        assert_eq!(expr4.to_latex(), "A \\to avocado");
        assert_eq!(expr4.arrow_chain(), vec![&expr2, &expr3]);
    }

    #[test]
    fn to_latex_simple_depend_abs() {
        let expr1 = String::from("potato");
        let expr2 = CCExpression::Var(String::from("A"));
        let expr3 = CCExpression::Application(
            Box::new(CCExpression::Var(String::from("avocado"))),
            Box::new(CCExpression::Var(String::from("potato")))
            );
        let expr4 = CCExpression::TypeAbs(expr1,
                                          Box::new(expr2),
                                          Box::new(expr3));
        assert_eq!(expr4.to_latex(), "\\prod potato : A . avocado potato");
        assert_eq!(expr4.substitute("potato", &CCExpression::Var("q".to_string())).to_latex(),
        "\\prod potato : A . avocado potato");
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
                   String::from("(\\lambda x : A . apple) banana"),
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
                   String::from("A"),
                   String::from("banana")
        ]);
    }

    #[test]
    fn is_sort_test() {
        assert!(!CCExpression::Var(String::from("X")).is_sort());
        assert!(CCExpression::Star.is_sort());
    }

    #[test]
    fn to_latex_contradiction() {
        let ex1 = CCExpression::Var("x".to_string());
        let ex2 = CCExpression::TypeAbs(
            "x".to_string(),
            Box::new(CCExpression::Star),
            Box::new(ex1)
        );
        assert_eq!(ex2.to_latex(), "\\perp");
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
        let expr5 = CCExpression::Var(String::from("A"));
        let expr6 = CCExpression::Abs(
            String::from("x"),
            Box::new(expr5.clone()),
            Box::new(expr1.clone())
            );

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
        assert_eq!(
            &expr6.substitute("x", &expr4).to_latex(),
            "\\lambda x : A . x"
            );
    }

    #[test]
    fn beta_reduce() {
        let expr1 = CCExpression::Var(String::from("x"));
        let expr2 = CCExpression::Var(String::from("y"));
        let expr4 = CCExpression::Var(String::from("z"));
        let expr5 = CCExpression::Var(String::from("A"));
        let expr6 = CCExpression::Abs(
            String::from("x"),
            Box::new(expr5.clone()),
            Box::new(expr1.clone())
            );
        let expr3 = CCExpression::Application(
            Box::new(expr6.clone()),
            Box::new(expr4.clone())
            );

        assert_eq!(expr3.beta_reduce().to_latex(), "z");
        assert_eq!(expr2.beta_reduce().to_latex(), "y");
        assert_eq!(expr6.beta_reduce().to_latex(), "\\lambda x : A . x");

    }
}
