//! https://www.w3.org/TR/2017/REC-xpath-31-20170321/#abbrev

use nom::{character::complete::char, combinator::opt, sequence::tuple};

use crate::xpath::grammar::recipes::Res;

use super::steps::node_tests::{node_test, NodeTest};

pub fn abbrev_forward_step(input: &str) -> Res<&str, AbbrevForwardStep> {
    // https://www.w3.org/TR/2017/REC-xpath-31-20170321/#doc-xpath31-AbbrevForwardStep
    tuple((opt(char('@')), node_test))(input).map(|(next_input, res)| {
        (
            next_input,
            AbbrevForwardStep {
                has_at: res.0.is_some(),
                node_test: res.1,
            },
        )
    })
}

pub struct AbbrevForwardStep {
    pub has_at: bool,
    pub node_test: NodeTest,
}
