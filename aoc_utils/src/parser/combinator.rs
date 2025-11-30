use super::Parser;

use std::ops::{Bound, RangeBounds};

////////////////////////////////////////////////////
//                    Repeated                    //
////////////////////////////////////////////////////

/// Parser that parses a range of occurrences.
#[derive(Clone, Copy)]
pub struct Repeat<P, R> {
    pub(super) parser: P,
    pub(super) range: R,
}

impl<P: Parser, R: RangeBounds<usize>> Parser for Repeat<P, R> {
    type Output = Vec<P::Output>;

    fn parse<'a>(&self, mut input: &'a str) -> Option<(Self::Output, &'a str)> {
        let min = match self.range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        let max = match self.range.end_bound() {
            Bound::Included(&n) => Some(n),
            Bound::Excluded(&n) => Some(n - 1),
            Bound::Unbounded => None,
        };

        let mut items = vec![];
        let mut count = 0;

        while max.is_none_or(|m| count < m)
            && let Some((out, rem)) = self.parser.parse(input)
        {
            items.push(out);
            input = rem;
            count += 1;
        }

        (count >= min).then_some((items, input))
    }
}

/////////////////////////////////////////////////
//                    SepBy                    //
/////////////////////////////////////////////////

/// Parser that parses a range of occurrences separated by another parser.
#[derive(Clone, Copy)]
pub struct SepBy<P, Q, R> {
    pub(super) parser: P,
    pub(super) rhs: Q,
    pub(super) range: R,
}

impl<P: Parser, Q: Parser, R: RangeBounds<usize>> Parser for SepBy<P, Q, R> {
    type Output = Vec<P::Output>;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        let min = match self.range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        let max = match self.range.end_bound() {
            Bound::Included(&n) => Some(n),
            Bound::Excluded(&n) => Some(n - 1),
            Bound::Unbounded => None,
        };

        let Some((first, mut s)) = self.parser.parse(input) else {
            return (min == 0).then_some((vec![], input));
        };

        let mut results = vec![first];
        let mut count = 1;

        while max.is_none_or(|m| count < m) {
            if let Some((a, rem)) = self
                .rhs
                .parse(s)
                .and_then(|(_, rem)| self.parser.parse(rem))
            {
                results.push(a);
                s = rem;
                count += 1;
            } else {
                break;
            }
        }

        (count >= min).then_some((results, s))
    }
}

///////////////////////////////////////////////
//                    Map                    //
///////////////////////////////////////////////

/// Parser that transforms the output of another parser using a function.
#[derive(Clone, Copy)]
pub struct Map<P, F> {
    pub(super) parser: P,
    pub(super) f: F,
}

impl<U, P: Parser, F: Fn(P::Output) -> U> Parser for Map<P, F> {
    type Output = U;

    fn parse<'a>(&self, s: &'a str) -> Option<(Self::Output, &'a str)> {
        self.parser
            .parse(s)
            .map(|(output, remaining)| ((self.f)(output), remaining))
    }
}

//////////////////////////////////////////////
//                    Or                    //
//////////////////////////////////////////////

/// Parser that combines two parsers, trying the first one and running the second if the first fails.
#[derive(Clone, Copy)]
pub struct Or<P, Q> {
    pub(super) lhs: P,
    pub(super) rhs: Q,
}

impl<P, Q> Parser for Or<P, Q>
where
    P: Parser,
    Q: Parser<Output = P::Output>,
{
    type Output = P::Output;

    fn parse<'a>(&self, s: &'a str) -> Option<(Self::Output, &'a str)> {
        self.lhs.parse(s).or_else(|| self.rhs.parse(s))
    }
}

///////////////////////////////////////////////////////
//                    IgnoredThen                    //
///////////////////////////////////////////////////////

/// Parser that combines two parsers executing them in sequence and returning the output of the second one.
#[derive(Clone, Copy)]
pub struct IgnoredThen<P, Q> {
    pub(super) parser: P,
    pub(super) rhs: Q,
}

impl<P: Parser, Q: Parser> Parser for IgnoredThen<P, Q> {
    type Output = Q::Output;

    fn parse<'a>(&self, s: &'a str) -> Option<(Self::Output, &'a str)> {
        self.parser
            .parse(s)
            .and_then(|(_, remaining)| self.rhs.parse(remaining))
    }
}

//////////////////////////////////////////////////////
//                    ThenIgnore                    //
//////////////////////////////////////////////////////

/// Parser that combines two parsers executing them in sequence and returning the output of the first one.
#[derive(Clone, Copy)]
pub struct ThenIgnore<P, Q> {
    pub(super) parser: P,
    pub(super) rhs: Q,
}

impl<P: Parser, Q: Parser> Parser for ThenIgnore<P, Q> {
    type Output = P::Output;

    fn parse<'a>(&self, s: &'a str) -> Option<(Self::Output, &'a str)> {
        self.parser.parse(s).and_then(|(output, remaining)| {
            self.rhs
                .parse(remaining)
                .map(|(_, remaining)| (output, remaining))
        })
    }
}

///////////////////////////////////////////////////
//                    ThenZip                    //
///////////////////////////////////////////////////

/// Parser that combines two parsers executing them in sequence and returning a tuple of the outputs.
#[derive(Clone, Copy)]
pub struct ThenZip<P, Q> {
    pub(super) parser: P,
    pub(super) rhs: Q,
}

impl<P: Parser, Q: Parser> Parser for ThenZip<P, Q> {
    type Output = (P::Output, Q::Output);

    fn parse<'a>(&self, s: &'a str) -> Option<(Self::Output, &'a str)> {
        self.parser.parse(s).and_then(|(left, remaining)| {
            self.rhs
                .parse(remaining)
                .map(|(right, remaining)| ((left, right), remaining))
        })
    }
}

///////////////////////////////////////////////////////
//                    ThenZipWith                    //
///////////////////////////////////////////////////////

/// Parser that combines two parsers executing them in sequence and returning the application of a function to the outputs.
#[derive(Clone, Copy)]
pub struct ThenZipWith<P, Q, F> {
    pub(super) parser: P,
    pub(super) rhs: Q,
    pub(super) f: F,
}

impl<V, P: Parser, Q: Parser, F: Fn(P::Output, Q::Output) -> V> Parser for ThenZipWith<P, Q, F> {
    type Output = V;

    fn parse<'a>(&self, s: &'a str) -> Option<(Self::Output, &'a str)> {
        self.parser.parse(s).and_then(|(left, remaining)| {
            self.rhs
                .parse(remaining)
                .map(|(right, remaining)| ((self.f)(left, right), remaining))
        })
    }
}

///////////////////////////////////////////////////
//                    AndThen                    //
///////////////////////////////////////////////////

/// Parser that is equivalent to `flat_map`.
#[derive(Clone, Copy)]
pub struct AndThen<P, F> {
    pub(super) parser: P,
    pub(super) f: F,
}

impl<P: Parser, Q: Parser, F: Fn(P::Output) -> Q> Parser for AndThen<P, F> {
    type Output = Q::Output;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        self.parser
            .parse(input)
            .and_then(|(x, remaining)| (self.f)(x).parse(remaining))
    }
}

/////////////////////////////////////////////////////
//                    LookAhead                    //
/////////////////////////////////////////////////////

/// Parser that not consumes any input.
#[derive(Clone, Copy)]
pub struct LookAhead<P> {
    pub(super) parser: P,
}

impl<P: Parser> Parser for LookAhead<P> {
    type Output = P::Output;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        self.parser.parse(input).map(|(output, _)| (output, input))
    }
}

////////////////////////////////////////////////////
//                    Anywhere                    //
////////////////////////////////////////////////////

/// Parser that discards input until the underlying parser succeeds.
#[derive(Clone, Copy)]
pub struct Anywhere<P> {
    pub(super) parser: P,
}

impl<P: Parser> Parser for Anywhere<P> {
    type Output = P::Output;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        match self.parser.parse(input) {
            None => match input {
                "" => None,
                other => self.parse(&other[1..]),
            },
            ok => ok,
        }
    }
}
