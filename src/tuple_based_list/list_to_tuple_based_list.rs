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
use crate::list::*;

meta!{
    pub type ListToTupleBasedList<
        K: EqualityComparableKind, 
        L: Expr<List<K>>
    >: Expr<TupleBasedList<K>> =
        VisitList<K, TupleBasedList<K>, L, ListToTupleBasedListVisitor<K>>;
}

mod internal {
    pub use super::super::internal::*;
    use crate::list::*;

    meta!{
        pub struct ListToTupleBasedListVisitor<K: EqualityComparableKind>: ListVisitor<K, TupleBasedList<K>> {
            type VisitEmptyList = WrapTupleBasedList<K, ()>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = WrapTupleBasedList<K, (Elem, UnwrapTupleBasedList<K, ListToTupleBasedList<K, Tail>>)>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::r#type::*;

    #[test]
    fn list_to_tuple_based_list() {
        meta_assert_eq!(TupleBasedList<Type>,
            ListToTupleBasedList<Type, type_list![]>,
            WrapTupleBasedList<Type, ()>);
        meta_assert_eq!(TupleBasedList<Type>,
            ListToTupleBasedList<Type, type_list![i32, u32, i32]>,
            WrapTupleBasedList<Type, (WrapType<i32>, (WrapType<u32>, (WrapType<i32>, ())))>);
    }
}
