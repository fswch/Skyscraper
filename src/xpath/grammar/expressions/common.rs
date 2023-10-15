use nom::{branch::alt, character::complete::char, combinator::opt, multi::many0, sequence::tuple};

use crate::xpath::grammar::{expressions::expr_single, recipes::Res};

use super::ExprSingle;

pub fn argument_list(input: &str) -> Res<&str, ArgumentList> {
    // https://www.w3.org/TR/2017/REC-xpath-31-20170321/#prod-xpath31-ArgumentList

    tuple((
        char('('),
        opt(tuple((argument, many0(tuple((char(','), argument)))))),
        char(')'),
    ))(input)
    .map(|(next_input, res)| {
        let mut arguments = Vec::new();
        if let Some(res) = res.1 {
            arguments.push(res.0);
            let extras = res.1.into_iter().map(|res| res.1);
            arguments.extend(extras);
        }
        (next_input, ArgumentList(arguments))
    })
}

pub struct ArgumentList(pub Vec<Argument>);

pub fn argument(input: &str) -> Res<&str, Argument> {
    // https://www.w3.org/TR/2017/REC-xpath-31-20170321/#prod-xpath31-Argument

    fn argument_placeholder(input: &str) -> Res<&str, Argument> {
        // https://www.w3.org/TR/2017/REC-xpath-31-20170321/#prod-xpath31-ArgumentPlaceholder
        char('?')(input).map(|(next_input, _res)| (next_input, Argument::ArgumentPlaceHolder))
    }

    fn expr_single_map(input: &str) -> Res<&str, Argument> {
        expr_single(input).map(|(next_input, res)| (next_input, Argument::ExprSingle(res)))
    }

    alt((expr_single_map, argument_placeholder))(input)
}

pub enum Argument {
    ExprSingle(ExprSingle),
    ArgumentPlaceHolder,
}
