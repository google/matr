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
use crate::pair::*;

meta!{
    pub type RemoveFromMap<
        K: KindWithDefault + EqualityComparableKind, 
        V: KindWithDefault,
        X: Expr<K>, 
        M: Expr<Map<K, V>>
    >: Expr<Map<K, V>> =
        ListToMapUnchecked<K, V, VisitList<Pair<K, V>, List<Pair<K, V>>, MapToList<K, V, M>, RemoveFromMapVisitor<K, V, X>>>;
}

mod internal {
    pub use super::super::internal::*;
    use crate::bool::*;

    meta!{
        pub struct RemoveFromMapVisitor<
            K: EqualityComparableKind + KindWithDefault, 
            V: KindWithDefault, 
            X: Expr<K>
        >: ListVisitor<Pair<K, V>, List<Pair<K, V>>> {
            type VisitEmptyList = EmptyList<Pair<K, V>>;
            type VisitCons<Elem: Expr<Pair<K, V>>, Tail: Expr<List<Pair<K, V>>>> = If<
                List<Pair<K, V>>,
                Equals<K, GetFirst<K, V, Elem>, X>,
                VisitList<Pair<K, V>, List<Pair<K, V>>, Tail, RemoveFromMapVisitor<K, V, X>>,
                Cons<Pair<K, V>, Elem, VisitList<Pair<K, V>, List<Pair<K, V>>, Tail, RemoveFromMapVisitor<K, V, X>>>
            >;
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
    fn remove_from_empty_map() {
        meta_assert_eq!(
            Map<Type, Type>,
            RemoveFromMap<Type, Type, WrapType<f32>, EmptyMap<Type, Type>>,
            EmptyMap<Type, Type>);
    }

    #[test]
    fn remove_from_map_found() {
        type M = type_map!{
            usize: (usize,),
            f32: (f32,),
            u64: (u64,)
        };
        type M2 = type_map!{
            usize: (usize,),
            u64: (u64,)};
        meta_assert_eq!(
            Map<Type, Type>,
            RemoveFromMap<Type, Type, WrapType<f32>, M>,
            M2);
    }

    #[test]
    fn remove_from_map_not_found() {
        type M = type_map!{
            usize: (usize,),
            f32: (f32,),
            u64: (u64,)};
        meta_assert_eq!(
            Map<Type, Type>,
            RemoveFromMap<Type, Type, WrapType<bool>, M>,
            M);
    }
}
