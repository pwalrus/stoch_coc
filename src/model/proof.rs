
use super::judgement::{Judgement};

#[derive(Debug,PartialEq)]
pub struct LineRef {
    pub rule: String,
    pub line1: Option<u32>,
    pub line2: Option<u32>
}

impl LineRef {

    pub fn to_latex(&self) -> String {
        if let Some(l) = self.line1 {
            if let Some(r) = self.line2 {
                return format!("{} {},{}", &self.rule, l, r);
            } else {
                return format!("{} {}", &self.rule, l);
            }
        } else {
            return self.rule.clone();
        }
    }
}

fn j_table_latex(jdg: &Judgement) -> String {
    let ctx: String = jdg.context.iter()
        .map(|x| x.to_latex()).collect::<Vec<String>>()
        .join(", ").to_string();
    let stmt: String = jdg.statement.to_latex();
    return format!("${}$ & $\\vdash$ & ${}$", ctx, stmt);
}


pub struct Proof {
    lines: Vec<Judgement>,
    refs: Vec<LineRef>
}

impl Proof {

    pub fn to_latex(&self) -> String {
        let output: Vec<String> = self.lines.iter().zip(&self.refs)
            .enumerate().map(
                |(idx, (a, b))| vec![idx.to_string(), j_table_latex(a), b.to_latex()].join(" & ") 
            ).collect();
        let start_str = "\\begin{tabular}{c c c c c}".to_string();
        let end_str = "\\end{tabular}".to_string();
        let comb: String = vec![
            start_str,
            output.join("\\\\\n"),
            end_str
        ].join("\n").trim().to_string();
        println!("comb: {}", comb);
        return comb;
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::expression::{CCExpression};
    use crate::model::judgement::{Statement};

    #[test]
    fn simple_proof_to_latex() {
        let stmt1 = Statement {
            subject: CCExpression::Star, 
            s_type: CCExpression::Sq
        };
        let stmt2 = Statement {
            subject: CCExpression::Var("A".to_string()), 
            s_type: CCExpression::Star
        };
        let judge1 = Judgement {
            context: vec![],
            statement: stmt1
        };
        let judge2 = Judgement {
            context: vec![stmt2.clone()],
            statement: stmt2
        };
        let refs = vec![
            LineRef {
                rule: String::from("sort"),
                line1: None,
                line2: None
            },
            LineRef {
                rule: String::from("var"),
                line1: Some(0),
                line2: None
            }
        ];
        let p1 = Proof {
            lines: vec![judge1, judge2],
            refs: refs
        };
        assert_eq!(p1.to_latex(), "\\begin{tabular}{c c c c c}\n".to_string()
                   + "0 & $$ & $\\vdash$ & $\\ast : \\square$ & sort\\\\\n"
                   + "1 & $A : \\ast$ & $\\vdash$ & $A : \\ast$ & var 0\n"
                   + "\\end{tabular}");
    }
}

