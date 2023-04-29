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
    pub type MapKeySet<
        K: KindWithDefault + EqualityComparableKind, 
        V: KindWithDefault, 
        M: Expr<Map<K, V>>
    >: Expr<Set<K>> =
        VisitMap<K, V, Set<K>, M, MapKeySetVisitor<K, V>>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct MapKeySetVisitor<
            K: KindWithDefault + EqualityComparableKind, 
            V: KindWithDefault
        >: MapVisitor<K, V, Set<K>> {
            type VisitEmptyMap = EmptySet<K>;
            type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>> = AddToSet<K, Key, VisitMap<K, V, Set<K>, Tail, MapKeySetVisitor<K, V>>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn map_key_set() {
        meta_assert_eq!(Set<Type>,
            MapKeySet<Type, Type, type_map!{}>,
            type_set!{});
        meta_assert_eq!(Set<Type>,
            MapKeySet<Type, Type, type_map!{i32: u32, i64: u64}>,
            type_set!{i32, i64});
    }
}
