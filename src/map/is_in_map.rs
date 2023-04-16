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

pub struct IsInMap<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>, S: Expr<Map<K, V>>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    x: PhantomData<X>,
    s: PhantomData<S>,
}

impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>, S: Expr<Map<K, V>>> Expr<Bool> for IsInMap<K, V, X, S> {
    type Eval = IsInMapValue<K, V, X, S>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct IsInMapValue<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>, S: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        x: PhantomData<X>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>, M: Expr<Map<K, V>>> BoolValue for IsInMapValue<K, V, X, M> {
        type Impl = AsBool<<AsList<Pair<K, V>, <AsMap<K, V, M> as MapTrait<K, V>>::GetList> as ListTrait<Pair<K, V>>>::Visit<Bool, IsInMapVisitor<K, V, X>>>;
    }

    pub struct IsInMapVisitor<K: EqualityComparableKind + KindWithDefault, V: Kind, X: Expr<K>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        x: PhantomData<X>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>> ListVisitor<Pair<K, V>, Bool> for IsInMapVisitor<K, V, X> {
        type VisitEmptyList = False;
        type VisitCons<Elem: Expr<Pair<K, V>>, Tail: Expr<List<Pair<K, V>>>> = Or<Equals<K, GetFirst<K, V, Elem>, X>, <AsList<Pair<K, V>, Tail> as ListTrait<Pair<K, V>>>::Visit<Bool, IsInMapVisitor<K, V, X>>>;
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
    fn is_in_empty_map() {
        assert_false!(IsInMap<Type, Type, WrapType<i32>, EmptyMap<Type, Type>>);
    }

    #[test]
    fn is_in_map_found() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<f64>, WrapType<u64>, Put<Type, Type, WrapType<usize>, WrapType<usize>, EmptyMap<Type, Type>>>>;
        assert_true!(IsInMap<Type, Type, WrapType<f64>, M>);
    }

    #[test]
    fn is_in_map_not_found() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<f64>, WrapType<u64>, Put<Type, Type, WrapType<usize>, WrapType<usize>, EmptyMap<Type, Type>>>>;
        assert_false!(IsInMap<Type, Type, WrapType<u32>, M>);
    }
}
