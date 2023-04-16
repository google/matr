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

pub struct RemoveFromList<K: EqualityComparableKind, X: Expr<K>, L: Expr<List<K>>> {
    k: PhantomData<K>,
    x: PhantomData<X>,
    l: PhantomData<L>,
}

impl<K: EqualityComparableKind, X: Expr<K>, L: Expr<List<K>>> Expr<List<K>> for RemoveFromList<K, X, L> {
    type Eval = <VisitList<K, List<K>, L, RemoveFromListVisitor<K, X>> as Expr<List<K>>>::Eval;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct RemoveFromListVisitor<K: EqualityComparableKind, X: Expr<K>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
    }

    impl<K: EqualityComparableKind, X: Expr<K>> ListVisitor<K, List<K>> for RemoveFromListVisitor<K, X> {
        type VisitEmptyList = EmptyList<K>;
        type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> =
            If<
                List<K>,
                Equals<K, Elem, X>,
                RemoveFromList<K, X, Tail>,
                Cons<K, Elem, RemoveFromList<K, X, Tail>>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn remove_from_empty_list() {
        assert_type_eq!(ToTypeNestedTuple<RemoveFromList<Type, WrapType<f32>, EmptyList<Type>>>, WrapType<()>);
    }

    #[test]
    fn remove_from_list_found() {
        type L = type_list![usize, f32, u64];
        assert_type_eq!(ToTypeNestedTuple<RemoveFromList<Type, WrapType<f32>, L>>, WrapType<(usize, (u64, ()))>);
    }

    #[test]
    fn remove_from_list_not_found() {
        type L = type_list![usize, f32, u64];
        assert_type_eq!(ToTypeNestedTuple<RemoveFromList<Type, WrapType<bool>, L>>, WrapType<(usize, (f32, (u64, ())))>);
    }
}
