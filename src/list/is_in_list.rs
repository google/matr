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

pub struct IsInList<K: EqualityComparableKind, X: Expr<K>, L: Expr<List<K>>> {
    k: PhantomData<K>,
    x: PhantomData<X>,
    l: PhantomData<L>,
}

impl<K: EqualityComparableKind, X: Expr<K>, L: Expr<List<K>>> Expr<Bool> for IsInList<K, X, L> {
    type Eval = IsInListValue<K, X, L>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct IsInListValue<K: EqualityComparableKind, X: Expr<K>, L: Expr<List<K>>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
        l: PhantomData<L>,
    }

    impl<K: EqualityComparableKind, X: Expr<K>, L: Expr<List<K>>> BoolValue for IsInListValue<K, X, L> {
        type Impl = AsBool<<AsList<K, L> as ListTrait<K>>::Visit<Bool, IsInListVisitor<K, X>>>;
    }

    pub struct IsInListVisitor<K: EqualityComparableKind, X: Expr<K>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
    }

    impl<K: EqualityComparableKind, X: Expr<K>> ListVisitor<K, Bool> for IsInListVisitor<K, X> {
        type VisitEmptyList = False;
        type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = Or<Equals<K, Elem, X>, IsInList<K, X, Tail>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::bool::assertions::{assert_false, assert_true};
    use crate::type_list::type_list;

    #[test]
    fn is_in_empty_list() {
        assert_false!(IsInList<Type, WrapType<i32>, EmptyList<Type>>);
    }

    #[test]
    fn is_in_list_found() {
        type L = type_list![i32, f64, usize];
        assert_true!(IsInList<Type, WrapType<f64>, L>);
    }

    #[test]
    fn is_in_list_not_found() {
        type L = type_list![i32, f64, usize];
        assert_false!(IsInList<Type, WrapType<u32>, L>);
    }
}
