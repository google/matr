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
    pub type TupleBasedListToList<
        K: EqualityComparableKind,
        S: Expr<TupleBasedList<K>>
    >: Expr<List<K>> =
        VisitTupleBasedList<K, List<K>, S, TupleBasedListToListVisitor<K>>;
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct TupleBasedListToListVisitor<K: EqualityComparableKind>: TupleBasedListVisitor<K, List<K>> {
            type VisitEmptyTupleBasedList = EmptyList<K>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<TupleBasedList<K>>> = Cons<K, Elem, TupleBasedListToList<K, Tail>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn tuple_based_list_to_list() {
        meta_assert_eq!(List<Type>,
            TupleBasedListToList<Type, WrapTupleBasedList<Type, ()>>,
            type_list![]);
        meta_assert_eq!(List<Type>,
            TupleBasedListToList<Type, WrapTupleBasedList<Type, (WrapType<i32>, (WrapType<u32>, ()))>>,
            type_list![i32, u32]);
    }
}
