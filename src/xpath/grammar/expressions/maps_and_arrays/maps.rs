//! https://www.w3.org/TR/2017/REC-xpath-31-20170321/#id-maps

use nom::{
    bytes::complete::tag, character::complete::char, combinator::opt, multi::many0, sequence::tuple,
};

use crate::xpath::grammar::{
    expressions::{expr_single, ExprSingle},
    recipes::Res,
};

pub fn map_constructor(input: &str) -> Res<&str, MapConstructor> {
    // https://www.w3.org/TR/2017/REC-xpath-31-20170321/#prod-xpath31-MapConstructor

    tuple((
        tag("map"),
        char('{'),
        opt(tuple((
            map_constructor_entry,
            many0(tuple((char(','), map_constructor_entry))),
        ))),
        char('}'),
    ))(input)
    .map(|(next_input, res)| {
        let mut entries = Vec::new();
        if let Some(res) = res.2 {
            entries.push(res.0);
            let extras = res.1.into_iter().map(|res| res.1);
            entries.extend(extras);
        }
        (next_input, MapConstructor { entries })
    })
}

pub struct MapConstructor {
    pub entries: Vec<MapConstructorEntry>,
}

fn map_constructor_entry(input: &str) -> Res<&str, MapConstructorEntry> {
    // https://www.w3.org/TR/2017/REC-xpath-31-20170321/#prod-xpath31-MapConstructorEntry
    tuple((map_key_expr, map_value_expr))(input).map(|(next_input, res)| {
        (
            next_input,
            MapConstructorEntry {
                key: res.0,
                value: res.1,
            },
        )
    })
}

pub struct MapConstructorEntry {
    pub key: ExprSingle,
    pub value: ExprSingle,
}

fn map_key_expr(input: &str) -> Res<&str, ExprSingle> {
    // https://www.w3.org/TR/2017/REC-xpath-31-20170321/#prod-xpath31-MapKeyExpr
    expr_single(input)
}

fn map_value_expr(input: &str) -> Res<&str, ExprSingle> {
    // https://www.w3.org/TR/2017/REC-xpath-31-20170321/#prod-xpath31-MapValueExpr
    expr_single(input)
}
