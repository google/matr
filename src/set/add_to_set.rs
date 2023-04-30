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
    pub type AddToSet<
        K: EqualityComparableKind, 
        X: Expr<K>, 
        S: Expr<Set<K>>
    >: Expr<Set<K>> =
        If<
            Set<K>,
            IsInSet<K, X, S>,
            S,
            ListToSetUnchecked<K, Cons<K, X, SetToList<K, S>>>>;
}

mod internal {
    pub use super::super::internal::*;
}


#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::marker::PhantomData;
    use crate::*;

    struct CalledVisitEmptySet {}

    struct CalledVisitCons<X, Tail: Expr<Set<Type>>> {
        x: PhantomData<X>,
        tail: PhantomData<Tail>,
    }

    meta!{
        struct MyVisitor : SetVisitor<Type, Type> {
            type VisitEmptySet = WrapType<CalledVisitEmptySet>;
            type VisitCons<X: Expr<Type>, Tail: Expr<Set<Type>>> = WrapType<CalledVisitCons<UnwrapType<X>, UnwrapExpr<Set<Type>, <Set<Type> as KindWithDebugForm>::DebugForm<Tail>>>>;
        }
    }

    #[test]
    fn visit() {
        meta_assert_eq!(Type,
            VisitSet<Type, Type,
                AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u32>, EmptySet<Type>>>,
                MyVisitor>,
            WrapType<CalledVisitCons<
                i32,
                AddToSet<Type, WrapType<u32>, EmptySet<Type>>>>);
    }

    #[test]
    fn add_commutative() {
        type S1 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<i32>, EmptySet<Type>>>;
        meta_assert_eq!(Set<Type>, S1, S2);
    }

    #[test]
    fn add_idempotent() {
        type S1 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<i32>, S1>>;
        meta_assert_eq!(Set<Type>, S1, S2);
    }
}
