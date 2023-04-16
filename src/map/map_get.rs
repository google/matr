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

// Returns the value associated with the key in the map, or the default value for the value kind otherwise.
// It's the caller's responsibility to check if the key in the map if getting the default value is not acceptable (e.g. if that could be a valid value in the map).
pub struct MapGet<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>, S: Expr<Map<K, V>>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    x: PhantomData<X>,
    s: PhantomData<S>,
}

impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>, S: Expr<Map<K, V>>> Expr<V> for MapGet<K, V, X, S> {
    type Eval = <<AsList<Pair<K, V>, <AsMap<K, V, S> as MapTrait<K, V>>::GetList> as ListTrait<Pair<K, V>>>::Visit<V, MapGetValueVisitor<K, V, X>> as Expr<V>>::Eval;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct MapGetValueVisitor<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        x: PhantomData<X>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>> ListVisitor<Pair<K, V>, V> for MapGetValueVisitor<K, V, X> {
        type VisitEmptyList = V::Default;
        type VisitCons<Elem: Expr<Pair<K, V>>, Tail: Expr<List<Pair<K, V>>>> = If<V, Equals<K, GetFirst<K, V, Elem>, X>,
            GetSecond<K, V, Elem>,
            <AsList<Pair<K, V>, Tail> as ListTrait<Pair<K, V>>>::Visit<V, MapGetValueVisitor<K, V, X>>
        >;
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
        assert_type_eq!(MapGet<Type, Type, WrapType<i32>, EmptyMap<Type, Type>>, WrapType<()>);
    }

    #[test]
    fn is_in_map_found() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<f64>, WrapType<u64>, Put<Type, Type, WrapType<usize>, WrapType<usize>, EmptyMap<Type, Type>>>>;
        assert_type_eq!(MapGet<Type, Type, WrapType<f64>, M>, WrapType<u64>);
    }

    #[test]
    fn is_in_map_not_found() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<f64>, WrapType<u64>, Put<Type, Type, WrapType<usize>, WrapType<usize>, EmptyMap<Type, Type>>>>;
        assert_type_eq!(MapGet<Type, Type, WrapType<u32>, M>, WrapType<()>);
    }
}