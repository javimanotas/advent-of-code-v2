//! This module, inspired in Haskell's Parsec, defines simple parser combinators of string slices.

mod combinator;
mod operators;
mod primitives;

pub use combinator::*;
pub use primitives::*;

use std::ops::RangeBounds;

/// A parser takes string slices as input and produces values of type `Output` along with
/// the remaining unconsumed string slice.
pub trait Parser: Sized {
    type Output;

    /// Parses the input string slice and returns an `Option` containing
    /// a tuple of the parsed value and the remaining unconsumed string slice.
    ///
    /// # Arguments
    /// * `input`: The string slice to parse.
    ///
    /// # Returns
    /// An `Option<(Self::Output, &str)>` containing the parsed value and
    /// the remaining string slice on success, or `None` on failure.
    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)>;

    /// Runs the parser on the given input and returns only the parsed value, if successful.
    ///
    /// # Arguments
    /// * `input`: The string slice to parse.
    ///
    /// # Returns
    /// An `Option<Self::Output>` containing the parsed value on success, or `None` on failure.
    fn run_parser(&self, input: &str) -> Option<Self::Output> {
        self.parse(input).map(|(output, _)| output)
    }

    /// Parses a range of occurrences of `self` and collects the results into a `Vec<Self::Output>`.
    ///
    /// # Returns
    /// A new `Parser` that produces a `Vec<Self::Output>`.
    ///
    /// # Arguments
    /// * `range`: The range for the number of ocurrences.
    ///
    /// # Examples
    /// ```
    /// use aoc_utils::parser::*;
    ///
    /// let digits = satisfies(|c| c.is_digit(10)).repeat(..);
    ///
    /// assert_eq!(digits.run_parser("123abc"), Some(vec!['1', '2', '3']));
    /// assert_eq!(digits.run_parser("abc"), Some(vec![]));
    ///
    /// let digits = satisfies(|c| c.is_digit(10)).repeat(1..);
    ///
    /// assert_eq!(digits.run_parser("123abc"), Some(vec!['1', '2', '3']));
    /// assert_eq!(digits.run_parser("abc"), None);
    ///
    /// let digits = satisfies(|c| c.is_digit(10)).repeat(..=2);
    ///
    /// assert_eq!(digits.run_parser("123abc"), Some(vec!['1', '2']));
    /// assert_eq!(digits.run_parser("abc"), Some(vec![]));
    /// ```
    fn repeat<R: RangeBounds<usize>>(self, range: R) -> ParserImpl<Repeat<Self, R>> {
        ParserImpl(Repeat {
            parser: self,
            range,
        })
    }

    /// Parses a range of occurrences of `self` separated by `rhs` and collects the results into a `Vec<Self::Output>`.
    ///
    /// # Returns
    /// A new `Parser` that produces a `Vec<Self::Output>`.
    ///
    /// # Arguments
    /// * `rhs`: The separator.
    /// * `range`: The range for the number of ocurrences.
    ///
    /// # Examples
    /// ```
    /// use aoc_utils::parser::*;
    ///
    /// let digits = satisfies(|c| c.is_digit(10)).sep_by(prefix(","), ..);
    ///
    /// assert_eq!(digits.run_parser("1,2,3abc"), Some(vec!['1', '2', '3']));
    /// assert_eq!(digits.run_parser("abc"), Some(vec![]));
    ///
    /// let digits = satisfies(|c| c.is_digit(10)).sep_by(prefix(","), 1..);
    ///
    /// assert_eq!(digits.run_parser("1,2,3abc"), Some(vec!['1', '2', '3']));
    /// assert_eq!(digits.run_parser("abc"), None);
    ///
    /// let digits = satisfies(|c| c.is_digit(10)).sep_by(prefix(","), ..=2);
    ///
    /// assert_eq!(digits.run_parser("1,2,3abc"), Some(vec!['1', '2']));
    /// assert_eq!(digits.run_parser("abc"), Some(vec![]));
    /// ```
    fn sep_by<Q: Parser, R: RangeBounds<usize>>(
        self,
        rhs: Q,
        range: R,
    ) -> ParserImpl<SepBy<Self, Q, R>> {
        ParserImpl(SepBy {
            parser: self,
            rhs,
            range,
        })
    }

    /// Transforms the output of the parser using the provided function `f`.
    ///
    /// # Arguments
    /// * `f`: A closure that takes the parsed value `Self::Output` and returns a new value `U`.
    ///
    /// # Returns
    /// A new `Parser` that produces values of type `U`.
    ///
    /// # Examples
    /// ```
    /// use aoc_utils::parser::*;
    ///
    /// let is_uppercase = satisfies(|c| c.is_alphabetic()).map(|c| c.is_uppercase());
    ///
    /// assert_eq!(is_uppercase.run_parser("Hello!"), Some(true));
    /// assert_eq!(is_uppercase.run_parser("hello!"), Some(false));
    /// ```
    fn map<U, F: Fn(Self::Output) -> U>(self, f: F) -> ParserImpl<Map<Self, F>> {
        ParserImpl(Map { parser: self, f })
    }

    /// Combines two parsers, trying the first one and running the second if the first fails.
    ///
    /// This can also be used via the `|` operator.
    ///
    /// # Arguments
    /// * `rhs`: The alternative parser to try if `self` fails.
    ///
    /// # Returns
    /// A new `Parser` that succeeds if either `self` or `rhs` succeeds.
    ///
    /// # Examples
    /// ```
    /// use aoc_utils::parser::*;
    ///
    /// let digit_or_letter = satisfies(|c| c.is_digit(10)).or(satisfies(char::is_alphabetic));
    /// assert_eq!(digit_or_letter.run_parser("1!"), Some('1'));
    /// assert_eq!(digit_or_letter.run_parser("A!"), Some('A'));
    /// ```
    fn or<Q: Parser<Output = Self::Output>>(self, rhs: Q) -> ParserImpl<Or<Self, Q>> {
        ParserImpl(Or { lhs: self, rhs })
    }

    /// Parses `self`, then parses `rhs`, and returns the result of `rhs`.
    ///
    /// This can also be used via the `>>` operator.
    ///
    /// # Arguments
    /// * `rhs`: The parser whose output will be returned.
    ///
    /// # Returns
    /// A new `Parser` that produces the output of `rhs`.
    ///
    /// # Examples
    /// ```
    /// use aoc_utils::parser::*;
    ///
    /// let digit = satisfies(|c| c.is_digit(10));
    /// let ranking = prefix("#").ignored_then(digit);
    /// assert_eq!(ranking.run_parser("#1"), Some('1'));
    /// ```
    fn ignored_then<Q: Parser>(self, rhs: Q) -> ParserImpl<IgnoredThen<Self, Q>> {
        ParserImpl(IgnoredThen { parser: self, rhs })
    }

    /// Parses `self`, then parses `rhs`, and returns the result of `self`.
    ///
    /// This can also be used via the `<<` operator.
    ///
    /// # Arguments
    /// * `rhs`: The parser whose output will be ignored.
    ///
    /// # Returns
    /// A new `Parser` that produces the output of `self`.
    ///
    /// # Examples
    /// ```
    /// use aoc_utils::parser::*;
    ///
    /// let single_digit = satisfies(|c| c.is_digit(10)).then_ignore(eof());
    /// assert_eq!(single_digit.run_parser("5"), Some('5'));
    /// ```
    fn then_ignore<Q: Parser>(self, rhs: Q) -> ParserImpl<ThenIgnore<Self, Q>> {
        ParserImpl(ThenIgnore { parser: self, rhs })
    }

    /// Parses `self`, then parses `rhs`, and combines their results into a tuple.
    ///
    /// This can also be used via the `+` operator.
    ///
    /// # Arguments
    /// * `rhs`: The parser to run after `self`.
    ///
    /// # Returns
    /// A new `Parser` that produces a tuple of `(Self::Output, Q::Output)`.
    ///
    /// # Examples
    ///```
    /// use aoc_utils::parser::*;
    ///
    /// let digit = satisfies(|c| c.is_digit(10));
    /// let tuple = digit.then_zip(digit);
    /// assert_eq!(tuple.run_parser("12"), Some(('1', '2')));
    /// ```
    fn then_zip<Q: Parser>(self, rhs: Q) -> ParserImpl<ThenZip<Self, Q>> {
        ParserImpl(ThenZip { parser: self, rhs })
    }

    /// Parses `self`, then parses `rhs`, and combines their results using the provided function `f`.
    ///
    /// # Arguments
    /// * `rhs`: The parser to run after `self`.
    /// * `f`: A closure that takes the results of both parsers and combines them into a single value of type `V`.
    ///
    /// # Returns
    /// A new `Parser` that produces values of type `V`.
    ///
    /// # Examples
    ///```
    /// use aoc_utils::parser::*;
    ///
    /// let digit = satisfies(|c| c.is_digit(10));
    /// let tuple = digit.then_zip_with(digit, |a, b| (a, b));
    ///
    /// assert_eq!(tuple.run_parser("12"), Some(('1', '2')));
    /// ```
    fn then_zip_with<Q: Parser, V, F: Fn(Self::Output, Q::Output) -> V>(
        self,
        rhs: Q,
        f: F,
    ) -> ParserImpl<ThenZipWith<Self, Q, F>> {
        ParserImpl(ThenZipWith {
            parser: self,
            rhs,
            f,
        })
    }

    /// Parses `self` and feeds its output to `f`. Then parses the resulting parser.
    ///
    /// # Arguments
    /// * `f`: A closure produces a parser.
    ///
    /// # Returns
    /// A new `Parser` that produces values of type `Q::Output`.
    ///
    ///```
    /// use aoc_utils::parser::*;
    ///
    /// let digit = satisfies(|c| c.is_digit(10));
    /// let tuple = digit.and_then(|c| digit.map(move |c1| (c, c1)));
    ///
    /// assert_eq!(tuple.run_parser("12"), Some(('1', '2')));
    /// ```
    fn and_then<Q: Parser, F: Fn(Self::Output) -> Q>(self, f: F) -> ParserImpl<AndThen<Self, F>> {
        ParserImpl(AndThen { parser: self, f })
    }

    /// Parses `self` without consuming any input.
    ///
    /// # Returns
    /// A new `Parser` that produces values of type `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc_utils::parser::*;
    ///
    /// let any = satisfies(|_| true).look_ahead();
    ///
    /// assert_eq!(any.then_zip(any).run_parser("hello"), Some(('h', 'h')));
    /// ```
    fn look_ahead(self) -> ParserImpl<LookAhead<Self>> {
        ParserImpl(LookAhead { parser: self })
    }

    /// Discards input until `self` succeeds, then parses `self`.
    ///
    /// Fails if reaches eof.
    ///
    /// # Returns
    /// A new `Parser` that produces values of type `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc_utils::parser::*;
    ///
    /// let int = number().anywhere();
    ///
    /// assert_eq!(int.run_parser("abcedf.;.123abc"), Some(123));
    /// ```
    fn anywhere(self) -> ParserImpl<Anywhere<Self>> {
        ParserImpl(Anywhere { parser: self })
    }
}

/// A wrapper for parsers that enables operator overloading.
#[derive(Clone, Copy)]
pub struct ParserImpl<P>(P);

impl<P: Parser> Parser for ParserImpl<P> {
    type Output = P::Output;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        self.0.parse(input)
    }
}

/// Allows to create strings from parsers
pub trait StrParser: Parser<Output = Vec<char>> + Sized {
    fn collect_string(self) -> impl Parser<Output = String>;
}

impl<P: Parser<Output = Vec<char>>> StrParser for P {
    fn collect_string(self) -> impl Parser<Output = String> {
        self.map(|chars| chars.into_iter().collect())
    }
}
