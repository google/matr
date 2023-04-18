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
    pub type MapEntrySet<
        K: KindWithDefault + EqualityComparableKind,
        V: KindWithDefault + EqualityComparableKind,
        M: Expr<Map<K, V>>
    >: Expr<Set<Pair<K, V>>> =
        VisitMap<K, V, Set<Pair<K, V>>, M, MapEntrySetVisitor<K, V>>;
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct MapEntrySetVisitor<
            K: KindWithDefault + EqualityComparableKind,
            V: KindWithDefault + EqualityComparableKind
        >: MapVisitor<K, V, Set<Pair<K, V>>> {
            type VisitEmptyMap = EmptySet<Pair<K, V>>;
            type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>> = AddToSet<Pair<K, V>, ConsPair<K, V, Key, Value>, VisitMap<K, V, Set<Pair<K, V>>, Tail, MapEntrySetVisitor<K, V>>>;
        }
    }
}
