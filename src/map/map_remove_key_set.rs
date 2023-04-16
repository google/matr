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

pub struct MapRemoveKeySet<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault, M: Expr<Map<K, V>>, S: Expr<Set<K>>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    m: PhantomData<M>,
    s: PhantomData<S>,
}

impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault, M: Expr<Map<K, V>>, S: Expr<Set<K>>> Expr<Map<K, V>> for MapRemoveKeySet<K, V, M, S> {
    type Eval = <VisitMap<K, V, Map<K, V>, M, MapRemoveKeySetVisitor<K, V, S, EmptyMap<K, V>>> as Expr<Map<K, V>>>::Eval;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct MapRemoveKeySetVisitor<K: EqualityComparableKind, V: Kind, S: Expr<Set<K>>, ResultM: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        s: PhantomData<S>,
        result_m: PhantomData<ResultM>,
    }

    impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault, S: Expr<Set<K>>, ResultM: Expr<Map<K, V>>> MapVisitor<K, V, Map<K, V>> for MapRemoveKeySetVisitor<K, V, S, ResultM> {
        type VisitEmptyMap = EmptyMap<K, V>;
        type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>> =
            VisitMap<K, V, Map<K, V>, Tail, MapRemoveKeySetVisitor<K, V, S,
                If<Map<K, V>,
                    IsInSet<K, Key, S>,
                    ResultM,
                    Put<K, V, Key, Value, ResultM>
                >>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn empty_map_and_empty_set() {
        assert_type_map_eq!(MapRemoveKeySet<Type, Type, EmptyMap<Type, Type>, EmptySet<Type>>, EmptyMap<Type, Type>);
    }

    #[test]
    fn non_empty_map_and_empty_set() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<i64>, EmptyMap<Type, Type>>;
        assert_type_map_eq!(MapRemoveKeySet<Type, Type, M, EmptySet<Type>>, M);
    }

    #[test]
    fn empty_map_and_non_empty_set() {
        type S = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        assert_type_map_eq!(MapRemoveKeySet<Type, Type, EmptyMap<Type, Type>, S>, EmptyMap<Type, Type>);
    }

    #[test]
    fn subset() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<i64>, Put<Type, Type, WrapType<f64>, WrapType<f32>, EmptyMap<Type, Type>>>;
        type S = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>>;
        assert_type_map_eq!(MapRemoveKeySet<Type, Type, M, S>, EmptyMap<Type, Type>);
    }

    #[test]
    fn superset() {
        type M = Put<Type, Type, WrapType<u32>, WrapType<(u32,)>, Put<Type, Type, WrapType<i32>, WrapType<(i32,)>, Put<Type, Type, WrapType<u64>, WrapType<(u64,)>, Put<Type, Type, WrapType<f64>, WrapType<(f64,)>, EmptyMap<Type, Type>>>>>;
        type S = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        assert_type_map_eq!(MapRemoveKeySet<Type, Type, M, S>, M);
    }

    #[test]
    fn general() {
        type M = Put<Type, Type, WrapType<u32>, WrapType<(u32,)>, Put<Type, Type, WrapType<i32>, WrapType<(i32,)>, EmptyMap<Type, Type>>>;
        type S = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        type M2 = Put<Type, Type, WrapType<i32>, WrapType<(i32,)>, Put<Type, Type, WrapType<f64>, WrapType<(f64,)>, Put<Type, Type, WrapType<f64>, WrapType<(f64,)>, EmptyMap<Type, Type>>>>;
        assert_type_map_eq!(MapRemoveKeySet<Type, Type, M, S>, M2);
    }
}
