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

pub struct Put<KeyK: EqualityComparableKind + KindWithDefault, ValueK: KindWithDefault, Key: Expr<KeyK>, Value: Expr<ValueK>, M: Expr<Map<KeyK, ValueK>>> {
    key_k: PhantomData<KeyK>,
    value_k: PhantomData<ValueK>,
    key: PhantomData<Key>,
    value: PhantomData<Value>,
    m: PhantomData<M>,
}

impl<KeyK: EqualityComparableKind + KindWithDefault, ValueK: KindWithDefault, Key: Expr<KeyK>, Value: Expr<ValueK>, M: Expr<Map<KeyK, ValueK>>> Expr<Map<KeyK, ValueK>> for Put<KeyK, ValueK, Key, Value, M> {
    type Eval = PutValue<KeyK, ValueK, ConsPair<KeyK, ValueK, Key, Value>, M>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct PutValue<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<Pair<K, V>>, M: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        x: PhantomData<X>,
        m: PhantomData<M>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<Pair<K, V>>, M: Expr<Map<K, V>>> MapValue<K, V> for PutValue<K, V, X, M> {
        type Impl = PutImpl<K, V, X, M>;
    }

    pub struct PutImpl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<Pair<K, V>>, M: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        x: PhantomData<X>,
        m: PhantomData<M>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<Pair<K, V>>, M: Expr<Map<K, V>>> MapTrait<K, V> for PutImpl<K, V, X, M> {
        type GetList = If<
            List<Pair<K, V>>,
            IsInMap<K, V, GetFirst<K, V, X>, M>,
            <AsMap<K, V, M> as MapTrait<K, V>>::GetList,
            Cons<Pair<K, V>, X, <AsMap<K, V, M> as MapTrait<K, V>>::GetList>
        >;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn put_commutative() {
        type S1 = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<i64>, WrapType<u64>, EmptyMap<Type, Type>>>;
        type S2 = Put<Type, Type, WrapType<i64>, WrapType<u64>, Put<Type, Type, WrapType<i32>, WrapType<u32>, EmptyMap<Type, Type>>>;
        assert_type_map_eq!(S1, S2);
    }

    #[test]
    fn put_idempotent() {
        type S1 = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<i32>, WrapType<u32>, EmptyMap<Type, Type>>>;
        type S2 = Put<Type, Type, WrapType<i32>, WrapType<u32>, EmptyMap<Type, Type>>;
        assert_type_map_eq!(S1, S2);
    }
}
