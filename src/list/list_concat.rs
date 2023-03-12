// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::marker::PhantomData;
use internal::*;

// Concatenates L and Tail.
pub struct ListConcat<K: Kind, L: Expr<List<K>>, Tail: Expr<List<K>>> {
    k: PhantomData<K>,
    l: PhantomData<L>,
    tail: PhantomData<Tail>,
}

impl<K: Kind, L: Expr<List<K>>, Tail: Expr<List<K>>> Expr<List<K>> for ListConcat<K, L, Tail> {
    type Eval = ListConcatValue<K, L, Tail>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ListConcatValue<K: Kind, L: Expr<List<K>>, Tail: Expr<List<K>>> {
        k: PhantomData<K>,
        l: PhantomData<L>,
        tail: PhantomData<Tail>,
    }

    impl<K: Kind, L: Expr<List<K>>, Tail: Expr<List<K>>> ListValue<K> for ListConcatValue<K, L, Tail> {
        type Impl = AsList<K, ReversedListConcat<K, ReverseList<K, L>, Tail>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    type N0 = Zero;
    type N1 = Increment<N0>;
    type N2 = Increment<N1>;
    type N3 = Increment<N2>;
    type N4 = Increment<N3>;
    type N5 = Increment<N4>;
    type N6 = Increment<N5>;

    #[test]
    fn empty_lists() {
        let v = to_usize_vec::<ListConcat<USize, EmptyList<USize>, EmptyList<USize>>>();
        assert_eq!(v, vec![]);
    }

    #[test]
    fn empty_lhs() {
        type L = Cons<USize, N3, Cons<USize, N4, Cons<USize, N5, EmptyList<USize>>>>;
        let v = to_usize_vec::<ListConcat<USize, EmptyList<USize>, L>>();
        assert_eq!(v, vec![3, 4, 5]);
    }

    #[test]
    fn empty_rhs() {
        type L = Cons<USize, N3, Cons<USize, N4, Cons<USize, N5, EmptyList<USize>>>>;
        let v = to_usize_vec::<ListConcat<USize, L, EmptyList<USize>>>();
        assert_eq!(v, vec![3, 4, 5]);
    }

    #[test]
    fn list_to_usize_vec() {
        type L1 = Cons<USize, N0, Cons<USize, N1, Cons<USize, N2, EmptyList<USize>>>>;
        type L2 = Cons<USize, N3, Cons<USize, N4, Cons<USize, N5, Cons<USize, N6, EmptyList<USize>>>>>;
        let v = to_usize_vec::<ListConcat<USize, L1, L2>>();
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6]);
    }
}
