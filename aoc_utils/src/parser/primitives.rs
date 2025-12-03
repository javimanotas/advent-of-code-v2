//! Primitive parsers for basic parsing tasks.
//! These parsers can be combined to create more complex parsers by using the `Parser` trait.

use super::{Parser, ParserImpl};

///////////////////////////////////////////////////
//                    AnyChar                    //
///////////////////////////////////////////////////

/// Parser that returns the first char.
#[derive(Clone, Copy)]
pub struct AnyChar {}

/// Creates a parser that returns the first char.
///
/// # Returns
/// A new `Parser` that produces `char`.
///
/// # Examples
/// ```
/// use aoc_utils::parser::*;
///
/// assert_eq!(any_char().run_parser("blablabla"), Some('b'));
/// assert_eq!(any_char().run_parser(""), None);
/// ```
pub fn any_char() -> ParserImpl<AnyChar> {
    ParserImpl(AnyChar {})
}

impl Parser for AnyChar {
    type Output = char;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        input.chars().next().map(|c| (c, &input[1..]))
    }
}

///////////////////////////////////////////////
//                    Eof                    //
///////////////////////////////////////////////

/// Parser that succeeds only if the input is empty (end of file).
#[derive(Clone, Copy)]
pub struct Eof {}

/// Creates a parser that succeeds only if the input is empty (end of file).
///
/// # Returns
/// A new `Parser` that produces `()`.
///
/// # Examples
/// ```
/// use aoc_utils::parser::*;
///
/// assert_eq!(eof().run_parser(""), Some(()));
/// assert_eq!(eof().run_parser("blablabla"), None);
/// ```
pub fn eof() -> ParserImpl<Eof> {
    ParserImpl(Eof {})
}

impl Parser for Eof {
    type Output = ();

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        input.is_empty().then_some(((), ""))
    }
}

/////////////////////////////////////////////////////
//                    Satisfies                    //
/////////////////////////////////////////////////////

/// Parser that matches a single character satisfying a given predicate.
#[derive(Clone, Copy)]
pub struct Satisfies<F: Fn(char) -> bool> {
    f: F,
}

/// Creates a parser that matches a single character satisfying a given predicate.
///
/// # Arguments
/// * `f`: A closure that takes a `char` and returns if the character satisfies a condition.
///
/// # Returns
/// A new `Parser` that produces the matched `char`.
///
/// # Examples
/// ```
/// use aoc_utils::parser::*;
///
/// let uppercase = satisfies(|c| c.is_uppercase());
///
/// assert_eq!(uppercase.run_parser("Hello!"), Some('H'));
/// assert_eq!(uppercase.run_parser("hello!"), None);
/// ```
pub fn satisfies<F: Fn(char) -> bool>(f: F) -> ParserImpl<Satisfies<F>> {
    ParserImpl(Satisfies { f })
}

impl<F: Fn(char) -> bool> Parser for Satisfies<F> {
    type Output = char;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        let mut chars = input.chars();
        chars
            .next()
            .and_then(|c| (self.f)(c).then_some((c, chars.as_str())))
    }
}

//////////////////////////////////////////////////
//                    Prefix                    //
//////////////////////////////////////////////////

/// Parser that consumes a prefix from the input.
#[derive(Clone, Copy)]
pub struct Prefix<'a> {
    p: &'a str,
}

/// Creates a parser that consumes a prefix from the input.
///
/// # Arguments
/// * `p`: The string slice to match as a prefix.
///
/// # Returns
/// A new `Parser` that produces the prefix.
///
/// # Examples
/// ```
/// use aoc_utils::parser::*;
///
/// let greetings = prefix("hello ");
///
/// assert_eq!(greetings.run_parser("hello Javi"), Some(("hello ")));
/// assert_eq!(greetings.run_parser("Hi Javi"), None);
/// ```
pub fn prefix<'a>(p: &'a str) -> ParserImpl<Prefix<'a>> {
    ParserImpl(Prefix { p })
}

impl<'b> Parser for Prefix<'b> {
    type Output = &'b str;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        input.strip_prefix(self.p).map(|s| (self.p, s))
    }
}

//////////////////////////////////////////////////
//                    Number                    //
//////////////////////////////////////////////////

/// Parser for positive integers.
#[derive(Clone, Copy)]
pub struct Number {}

/// Creates a parser that parses a positive integer.
///
/// # Returns
/// A new `Parser` that produces a `usize`.
///
/// # Examples
/// ```
/// use aoc_utils::parser::*;
///
/// assert_eq!(number().run_parser("123"), Some(123));
/// ```
pub fn number() -> ParserImpl<Number> {
    ParserImpl(Number {})
}

impl Parser for Number {
    type Output = usize;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        let fst_non_digit = input
            .char_indices()
            .find(|(_, c)| !c.is_ascii_digit())
            .map(|(i, _)| i)
            .unwrap_or_else(|| input.len());

        input[0..fst_non_digit]
            .parse::<usize>()
            .ok()
            .map(|n| (n, &input[fst_non_digit..]))
    }
}

////////////////////////////////////////////////////
//                    Closures                    //
////////////////////////////////////////////////////

/// Implementing `Parser<T>` for closures allows to define recursive parsers
///
/// # Examples
/// ```
/// use aoc_utils::parser::*;
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Nested {
///     Single(usize),
///     Multiple(Vec<Nested>),
/// }
///
/// use Nested::*;
///
/// fn nested(input: &str) -> Option<(Nested, &str)> {
///     number()
///         .map(Single)
///         .or(prefix("[")
///             .ignored_then(nested.sep_by(prefix(", "), ..).map(Multiple))
///             .then_ignore(prefix("]")))
///         .parse(input)
/// }
///
/// assert_eq!(
///     nested.run_parser("[1, 2, [3, [4], []], 5]"),
///     Some(Multiple(vec![
///         Single(1),
///         Single(2),
///         Multiple(vec![Single(3), Multiple(vec![Single(4)]), Multiple(vec![])]),
///         Single(5)
///     ]))
/// );
/// ```
impl<T, F: Fn(&str) -> Option<(T, &str)>> Parser for F {
    type Output = T;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        self(input)
    }
}
