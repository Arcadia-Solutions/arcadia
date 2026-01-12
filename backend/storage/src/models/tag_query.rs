use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, multispace0},
    combinator::map,
    multi::many0,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum TagExpr {
    Tag(String),
    And(Box<TagExpr>, Box<TagExpr>),
    Or(Box<TagExpr>, Box<TagExpr>),
    Not(Box<TagExpr>),
}

#[derive(Debug, Error)]
pub enum TagQueryError {
    #[error("failed to parse tag query: {0}")]
    ParseError(String),
    #[error("empty tag query")]
    EmptyQuery,
}

fn is_tag_char(c: char) -> bool {
    c.is_alphanumeric() || c == '.' || c == '_' || c == '-'
}

fn parse_tag_name(input: &str) -> IResult<&str, TagExpr> {
    map(take_while1(is_tag_char), |s: &str| {
        TagExpr::Tag(s.to_string())
    })
    .parse(input)
}

fn parse_factor(input: &str) -> IResult<&str, TagExpr> {
    alt((
        // NOT expression: !factor
        map(
            preceded(delimited(multispace0, char('!'), multispace0), parse_factor),
            |expr| TagExpr::Not(Box::new(expr)),
        ),
        // Parenthesized expression: (expr)
        delimited(
            delimited(multispace0, char('('), multispace0),
            parse_expr,
            delimited(multispace0, char(')'), multispace0),
        ),
        // Tag name
        delimited(multispace0, parse_tag_name, multispace0),
    ))
    .parse(input)
}

fn parse_term(input: &str) -> IResult<&str, TagExpr> {
    let (input, first) = parse_factor(input)?;
    let (input, rest) = many0(preceded(
        delimited(multispace0, char('&'), multispace0),
        parse_factor,
    ))
    .parse(input)?;

    let result = rest.into_iter().fold(first, |acc, expr| {
        TagExpr::And(Box::new(acc), Box::new(expr))
    });

    Ok((input, result))
}

fn parse_expr(input: &str) -> IResult<&str, TagExpr> {
    let (input, first) = parse_term(input)?;
    let (input, rest) = many0(preceded(
        delimited(multispace0, char('|'), multispace0),
        parse_term,
    ))
    .parse(input)?;

    let result = rest.into_iter().fold(first, |acc, expr| {
        TagExpr::Or(Box::new(acc), Box::new(expr))
    });

    Ok((input, result))
}

impl TagExpr {
    pub fn parse(input: &str) -> Result<Self, TagQueryError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(TagQueryError::EmptyQuery);
        }

        match parse_expr(input) {
            Ok((remaining, expr)) => {
                if remaining.trim().is_empty() {
                    Ok(expr)
                } else {
                    Err(TagQueryError::ParseError(format!(
                        "unexpected trailing input: '{}'",
                        remaining
                    )))
                }
            }
            Err(e) => Err(TagQueryError::ParseError(e.to_string())),
        }
    }

    pub fn collect_tags(&self) -> Vec<String> {
        let mut tags = HashSet::new();
        self.collect_tags_inner(&mut tags);
        tags.into_iter().collect()
    }

    fn collect_tags_inner(&self, tags: &mut HashSet<String>) {
        match self {
            TagExpr::Tag(name) => {
                tags.insert(name.clone());
            }
            TagExpr::And(left, right) | TagExpr::Or(left, right) => {
                left.collect_tags_inner(tags);
                right.collect_tags_inner(tags);
            }
            TagExpr::Not(inner) => {
                inner.collect_tags_inner(tags);
            }
        }
    }

    pub fn evaluate(&self, title_group_tags: &HashSet<String>) -> bool {
        match self {
            TagExpr::Tag(name) => title_group_tags.contains(name),
            TagExpr::And(left, right) => {
                left.evaluate(title_group_tags) && right.evaluate(title_group_tags)
            }
            TagExpr::Or(left, right) => {
                left.evaluate(title_group_tags) || right.evaluate(title_group_tags)
            }
            TagExpr::Not(inner) => !inner.evaluate(title_group_tags),
        }
    }

    pub fn has_positive_tags(&self) -> bool {
        match self {
            TagExpr::Tag(_) => true,
            TagExpr::And(left, right) | TagExpr::Or(left, right) => {
                left.has_positive_tags() || right.has_positive_tags()
            }
            TagExpr::Not(_) => false,
        }
    }

    /// Generate SQL WHERE clause fragment with positional parameters.
    /// Returns (sql_fragment, tag_values) where sql_fragment uses $offset, $offset+1, etc.
    /// The `column` parameter is the SQL column name containing the tags array.
    pub fn to_sql(&self, column: &str, param_offset: usize) -> (String, Vec<String>) {
        let mut params = Vec::new();
        let sql = self.to_sql_inner(column, param_offset, &mut params);
        (sql, params)
    }

    fn to_sql_inner(&self, column: &str, param_offset: usize, params: &mut Vec<String>) -> String {
        match self {
            TagExpr::Tag(name) => {
                let param_num = param_offset + params.len() + 1;
                params.push(name.clone());
                format!("${} = ANY({})", param_num, column)
            }
            TagExpr::And(left, right) => {
                let left_sql = left.to_sql_inner(column, param_offset, params);
                let right_sql = right.to_sql_inner(column, param_offset, params);
                format!("({} AND {})", left_sql, right_sql)
            }
            TagExpr::Or(left, right) => {
                let left_sql = left.to_sql_inner(column, param_offset, params);
                let right_sql = right.to_sql_inner(column, param_offset, params);
                format!("({} OR {})", left_sql, right_sql)
            }
            TagExpr::Not(inner) => {
                let inner_sql = inner.to_sql_inner(column, param_offset, params);
                format!("NOT ({})", inner_sql)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_tag() {
        let expr = TagExpr::parse("action").unwrap();
        assert_eq!(expr, TagExpr::Tag("action".to_string()));
    }

    #[test]
    fn test_parse_and() {
        let expr = TagExpr::parse("action & comedy").unwrap();
        assert_eq!(
            expr,
            TagExpr::And(
                Box::new(TagExpr::Tag("action".to_string())),
                Box::new(TagExpr::Tag("comedy".to_string()))
            )
        );
    }

    #[test]
    fn test_parse_or() {
        let expr = TagExpr::parse("action | comedy").unwrap();
        assert_eq!(
            expr,
            TagExpr::Or(
                Box::new(TagExpr::Tag("action".to_string())),
                Box::new(TagExpr::Tag("comedy".to_string()))
            )
        );
    }

    #[test]
    fn test_parse_not() {
        let expr = TagExpr::parse("!horror").unwrap();
        assert_eq!(
            expr,
            TagExpr::Not(Box::new(TagExpr::Tag("horror".to_string())))
        );
    }

    #[test]
    fn test_parse_complex() {
        let expr = TagExpr::parse("(action & comedy) | drama").unwrap();
        assert_eq!(
            expr,
            TagExpr::Or(
                Box::new(TagExpr::And(
                    Box::new(TagExpr::Tag("action".to_string())),
                    Box::new(TagExpr::Tag("comedy".to_string()))
                )),
                Box::new(TagExpr::Tag("drama".to_string()))
            )
        );
    }

    #[test]
    fn test_parse_complex_with_not() {
        let expr = TagExpr::parse("(action & comedy) | drama & !horror").unwrap();
        assert_eq!(
            expr,
            TagExpr::Or(
                Box::new(TagExpr::And(
                    Box::new(TagExpr::Tag("action".to_string())),
                    Box::new(TagExpr::Tag("comedy".to_string()))
                )),
                Box::new(TagExpr::And(
                    Box::new(TagExpr::Tag("drama".to_string())),
                    Box::new(TagExpr::Not(Box::new(TagExpr::Tag("horror".to_string()))))
                ))
            )
        );
    }

    #[test]
    fn test_evaluate_single_tag() {
        let expr = TagExpr::parse("action").unwrap();
        let tags: HashSet<String> = ["action".to_string()].into_iter().collect();
        assert!(expr.evaluate(&tags));

        let tags: HashSet<String> = ["comedy".to_string()].into_iter().collect();
        assert!(!expr.evaluate(&tags));
    }

    #[test]
    fn test_evaluate_and() {
        let expr = TagExpr::parse("action & comedy").unwrap();
        let tags: HashSet<String> = ["action".to_string(), "comedy".to_string()]
            .into_iter()
            .collect();
        assert!(expr.evaluate(&tags));

        let tags: HashSet<String> = ["action".to_string()].into_iter().collect();
        assert!(!expr.evaluate(&tags));
    }

    #[test]
    fn test_evaluate_or() {
        let expr = TagExpr::parse("action | comedy").unwrap();
        let tags: HashSet<String> = ["action".to_string()].into_iter().collect();
        assert!(expr.evaluate(&tags));

        let tags: HashSet<String> = ["drama".to_string()].into_iter().collect();
        assert!(!expr.evaluate(&tags));
    }

    #[test]
    fn test_evaluate_not() {
        let expr = TagExpr::parse("!horror").unwrap();
        let tags: HashSet<String> = ["action".to_string()].into_iter().collect();
        assert!(expr.evaluate(&tags));

        let tags: HashSet<String> = ["horror".to_string()].into_iter().collect();
        assert!(!expr.evaluate(&tags));
    }

    #[test]
    fn test_collect_tags() {
        let expr = TagExpr::parse("(action & comedy) | drama & !horror").unwrap();
        let mut tags = expr.collect_tags();
        tags.sort();
        assert_eq!(tags, vec!["action", "comedy", "drama", "horror"]);
    }

    #[test]
    fn test_parse_dotted_tag() {
        let expr = TagExpr::parse("sci.fi").unwrap();
        assert_eq!(expr, TagExpr::Tag("sci.fi".to_string()));
    }

    #[test]
    fn test_has_positive_tags() {
        let expr = TagExpr::parse("action & comedy").unwrap();
        assert!(expr.has_positive_tags());

        let expr = TagExpr::parse("!horror").unwrap();
        assert!(!expr.has_positive_tags());

        let expr = TagExpr::parse("action & !horror").unwrap();
        assert!(expr.has_positive_tags());
    }

    #[test]
    fn test_to_sql_single_tag() {
        let expr = TagExpr::parse("action").unwrap();
        let (sql, params) = expr.to_sql("tags", 0);
        assert_eq!(sql, "$1 = ANY(tags)");
        assert_eq!(params, vec!["action"]);
    }

    #[test]
    fn test_to_sql_and() {
        let expr = TagExpr::parse("action & comedy").unwrap();
        let (sql, params) = expr.to_sql("tags", 0);
        assert_eq!(sql, "($1 = ANY(tags) AND $2 = ANY(tags))");
        assert_eq!(params, vec!["action", "comedy"]);
    }

    #[test]
    fn test_to_sql_or() {
        let expr = TagExpr::parse("action | comedy").unwrap();
        let (sql, params) = expr.to_sql("tags", 0);
        assert_eq!(sql, "($1 = ANY(tags) OR $2 = ANY(tags))");
        assert_eq!(params, vec!["action", "comedy"]);
    }

    #[test]
    fn test_to_sql_not() {
        let expr = TagExpr::parse("!horror").unwrap();
        let (sql, params) = expr.to_sql("tags", 0);
        assert_eq!(sql, "NOT ($1 = ANY(tags))");
        assert_eq!(params, vec!["horror"]);
    }

    #[test]
    fn test_to_sql_complex() {
        let expr = TagExpr::parse("(action & comedy) | drama & !horror").unwrap();
        let (sql, params) = expr.to_sql("tags", 0);
        assert_eq!(
            sql,
            "(($1 = ANY(tags) AND $2 = ANY(tags)) OR ($3 = ANY(tags) AND NOT ($4 = ANY(tags))))"
        );
        assert_eq!(params, vec!["action", "comedy", "drama", "horror"]);
    }

    #[test]
    fn test_to_sql_with_offset() {
        let expr = TagExpr::parse("action & comedy").unwrap();
        let (sql, params) = expr.to_sql("tags", 5);
        assert_eq!(sql, "($6 = ANY(tags) AND $7 = ANY(tags))");
        assert_eq!(params, vec!["action", "comedy"]);
    }
}
