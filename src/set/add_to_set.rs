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

pub struct AddToSet<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> {
    k: PhantomData<K>,
    x: PhantomData<X>,
    s: PhantomData<S>,
}

impl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> Expr<Set<K>> for AddToSet<K, X, S> {
    type Eval = AddToSetValue<K, X, S>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct AddToSetValue<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> SetValue<K> for AddToSetValue<K, X, S> {
        type Impl = AddToSetImpl<K, X, S>;
    }

    pub struct AddToSetImpl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> SetTrait<K> for AddToSetImpl<K, X, S> {
        type GetList = If<
            List<K>,
            IsInList<K, X, <AsSet<K, S> as SetTrait<K>>::GetList>,
            <AsSet<K, S> as SetTrait<K>>::GetList,
            Cons<K, X, <AsSet<K, S> as SetTrait<K>>::GetList>
        >;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn add_commutative() {
        type S1 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<i32>, EmptySet<Type>>>;
        assert_type_set_eq!(S1, S2);
    }

    #[test]
    fn add_idempotent() {
        type S1 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<i32>, S1>>;
        assert_type_set_eq!(S1, S2);
    }
}
