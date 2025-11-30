use super::{Parser, ParserImpl, combinator::*};

use std::ops::{Add, BitOr, Shl, Shr};

impl<P: Parser, Q: Parser<Output = P::Output>> BitOr<Q> for ParserImpl<P> {
    type Output = ParserImpl<Or<P, Q>>;

    fn bitor(self, rhs: Q) -> Self::Output {
        ParserImpl(Or { lhs: self.0, rhs })
    }
}

impl<P: Parser, Q: Parser> Add<Q> for ParserImpl<P> {
    type Output = ParserImpl<ThenZip<P, Q>>;

    fn add(self, rhs: Q) -> Self::Output {
        ParserImpl(ThenZip {
            parser: self.0,
            rhs,
        })
    }
}

impl<P: Parser, Q: Parser> Shr<Q> for ParserImpl<P> {
    type Output = ParserImpl<IgnoredThen<P, Q>>;

    fn shr(self, rhs: Q) -> Self::Output {
        ParserImpl(IgnoredThen {
            parser: self.0,
            rhs,
        })
    }
}

impl<P: Parser, Q: Parser> Shl<Q> for ParserImpl<P> {
    type Output = ParserImpl<ThenIgnore<P, Q>>;

    fn shl(self, rhs: Q) -> Self::Output {
        ParserImpl(ThenIgnore {
            parser: self.0,
            rhs,
        })
    }
}
