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

use internal::*;

meta!{
    pub type Put<
        KeyK: EqualityComparableKind + KindWithDefault, 
        ValueK: KindWithDefault, 
        Key: Expr<KeyK>,
        Value: Expr<ValueK>, 
        M: Expr<Map<KeyK, ValueK>>
    >: Expr<Map<KeyK, ValueK>> =
        If<
            Map<KeyK, ValueK>,
            IsInMap<KeyK, ValueK, Key, M>,
            M,
            ListToMapUnchecked<KeyK, ValueK, Cons<Pair<KeyK, ValueK>, ConsPair<KeyK, ValueK, Key, Value>, MapToList<KeyK, ValueK, M>>>
        >;
}

mod internal {
    pub use super::super::internal::*;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::marker::PhantomData;
    use crate::*;

    struct CalledVisitEmptyMap {}

    struct CalledVisitEntry<Key, Value, Tail: Expr<Map<Type, Type>>> {
        key: PhantomData<Key>,
        value: PhantomData<Value>,
        tail: PhantomData<Tail>,
    }

    meta!{
        struct MyVisitor : MapVisitor<Type, Type, Type> {
            type VisitEmptyMap = WrapType<CalledVisitEmptyMap>;
            type VisitEntry<Key: Expr<Type>, Value: Expr<Type>, Tail: Expr<Map<Type, Type>>> = WrapType<CalledVisitEntry<UnwrapType<Key>, UnwrapType<Value>, UnwrapExpr<Map<Type, Type>, <Map<Type, Type> as KindWithDebugForm>::DebugForm<Tail>>>>;
        }
    }

    #[test]
    fn visit() {
        meta_assert_eq!(Type,
            VisitMap<Type, Type, Type,
                Put<Type, Type, WrapType<i32>, WrapType<i64>, Put<Type, Type, WrapType<u32>, WrapType<u64>, EmptyMap<Type, Type>>>,
                MyVisitor>,
            WrapType<CalledVisitEntry<
                i32,
                i64,
                Put<Type, Type, WrapType<u32>, WrapType<u64>, EmptyMap<Type, Type>>>>);
    }

    #[test]
    fn put_commutative() {
        type S1 = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<i64>, WrapType<u64>, EmptyMap<Type, Type>>>;
        type S2 = Put<Type, Type, WrapType<i64>, WrapType<u64>, Put<Type, Type, WrapType<i32>, WrapType<u32>, EmptyMap<Type, Type>>>;
        meta_assert_eq!(Map<Type, Type>, S1, S2);
    }

    #[test]
    fn put_idempotent() {
        type S1 = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<i32>, WrapType<u32>, EmptyMap<Type, Type>>>;
        type S2 = Put<Type, Type, WrapType<i32>, WrapType<u32>, EmptyMap<Type, Type>>;
        meta_assert_eq!(Map<Type, Type>, S1, S2);
    }
}
