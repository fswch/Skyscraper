//! https://www.w3.org/TR/2017/REC-xpath-31-20170321/#id-instance-of

use std::fmt::Display;

use nom::{bytes::complete::tag, combinator::opt, error::context, sequence::tuple};

use crate::xpath::grammar::{
    recipes::Res,
    types::sequence_type::{sequence_type, SequenceType},
};

use super::treat::{treat_expr, TreatExpr};

pub fn instanceof_expr(input: &str) -> Res<&str, InstanceofExpr> {
    // https://www.w3.org/TR/2017/REC-xpath-31-20170321/#prod-xpath31-InstanceofExpr

    context(
        "instanceof_expr",
        tuple((
            treat_expr,
            opt(tuple((tag("instance"), tag("of"), sequence_type))),
        )),
    )(input)
    .map(|(next_input, res)| {
        let instanceof_type = res.1.map(|res| res.2);
        (
            next_input,
            InstanceofExpr {
                expr: res.0,
                instanceof_type,
            },
        )
    })
}

#[derive(PartialEq, Debug)]
pub struct InstanceofExpr {
    pub expr: TreatExpr,
    pub instanceof_type: Option<SequenceType>,
}

impl Display for InstanceofExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)?;
        if let Some(x) = &self.instanceof_type {
            write!(f, " instance of {}", x)?;
        }

        Ok(())
    }
}
