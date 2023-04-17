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

mod empty_map;
mod put;
mod is_in_map;
pub mod assertions;
mod map_to_list;
mod list_to_map;
mod to_hash_map;
mod remove_from_map;
mod map_get;
mod map_key_set;
mod map_entry_set;
mod map_remove_key_set;
mod map_common_keys_with_different_value;
pub mod map;
pub mod type_map;

pub use empty_map::*;
pub use put::*;
pub use is_in_map::*;
pub use map_to_list::*;
pub use list_to_map::*;
pub use to_hash_map::*;
pub use remove_from_map::*;
pub use map_get::*;
pub use map_key_set::*;
pub use map_entry_set::*;
pub use map_remove_key_set::*;
pub use map_common_keys_with_different_value::*;
pub use map::*;
pub use type_map::*;

use std::marker::PhantomData;
use internal::*;

pub struct Map<K: EqualityComparableKind, V: Kind> {
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K: EqualityComparableKind, V: Kind> Kind for Map<K, V> {}

impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault + EqualityComparableKind> EqualityComparableKind for Map<K, V> {
    type Eq<X: Expr<Map<K, V>>, Y: Expr<Map<K, V>>> = MapEquals<K, V, X, Y>;
}

impl<K: EqualityComparableKind, V: Kind> KindWithDefault for Map<K, V> {
    type Default = EmptyMap<K, V>;
}

pub trait MapVisitor<K: EqualityComparableKind + KindWithDefault, V: Kind, OutK: Kind> {
    type VisitEmptyMap: Expr<OutK>;
    type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>>: Expr<OutK>;
}

pub struct VisitMap<K: EqualityComparableKind + KindWithDefault, V: Kind, OutK: Kind, M: Expr<Map<K, V>>, Visitor: MapVisitor<K, V, OutK>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    out_k: PhantomData<OutK>,
    m: PhantomData<M>,
    visitor: PhantomData<Visitor>,
}

impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, OutK: Kind, M: Expr<Map<K, V>>, Visitor: MapVisitor<K, V, OutK>> Expr<OutK> for VisitMap<K, V, OutK, M, Visitor> {
    type Eval = <VisitList<Pair<K, V>, OutK, <AsMap<K, V, M> as MapTrait<K, V>>::GetList, MapVisitorToListVisitorAdapter<K, V, OutK, Visitor>> as Expr<OutK>>::Eval;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub struct ListToMapUnchecked<K: EqualityComparableKind, V: Kind, L: Expr<List<Pair<K, V>>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        l: PhantomData<L>,
    }

    impl<K: EqualityComparableKind, V: Kind, L: Expr<List<Pair<K, V>>>> Expr<Map<K, V>> for ListToMapUnchecked<K, V, L> {
        type Eval = ListToMapUncheckedValue<K, V, L>;
    }

    pub struct ListToMapUncheckedValue<K: EqualityComparableKind, V: Kind, L: Expr<List<Pair<K, V>>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        l: PhantomData<L>,
    }

    impl<K: EqualityComparableKind, V: Kind, L: Expr<List<Pair<K, V>>>> MapValue<K, V> for ListToMapUncheckedValue<K, V, L> {
        type Impl = ListToMapUncheckedImpl<K, V, L>;
    }

    pub struct ListToMapUncheckedImpl<K: EqualityComparableKind, V: Kind, L: Expr<List<Pair<K, V>>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        l: PhantomData<L>,
    }

    impl<K: EqualityComparableKind, V: Kind, L: Expr<List<Pair<K, V>>>> MapTrait<K, V> for ListToMapUncheckedImpl<K, V, L> {
        type GetList = L;
    }

    pub struct MapVisitorToListVisitorAdapter<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, OutK: Kind, Visitor: MapVisitor<K, V, OutK>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        out_k: PhantomData<OutK>,
        visitor: PhantomData<Visitor>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, OutK: Kind, Visitor: MapVisitor<K, V, OutK>> ListVisitor<Pair<K, V>, OutK> for MapVisitorToListVisitorAdapter<K, V, OutK, Visitor> {
        type VisitEmptyList = Visitor::VisitEmptyMap;
        type VisitCons<Elem: Expr<Pair<K, V>>, Tail: Expr<List<Pair<K, V>>>> = Visitor::VisitEntry<GetFirst<K, V, Elem>, GetSecond<K, V, Elem>, ListToMapUnchecked<K, V, Tail>>;
    }

    pub trait MapValue<K: EqualityComparableKind, V: Kind> {
        type Impl: MapTrait<K, V>;
    }

    impl<K: EqualityComparableKind, V: Kind, U: MapValue<K, V>> Value<Map<K, V>> for U {
        type UnconstrainedImpl = <U as MapValue<K, V>>::Impl;
    }

    pub trait MapTrait<K: EqualityComparableKind, V: Kind> {
        type GetList: Expr<List<Pair<K, V>>>;
    }

    pub struct AsMap<K: EqualityComparableKind + KindWithDefault, V: Kind, S: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        s: PhantomData<S>,
    }

    impl<K: KindWithDefault + EqualityComparableKind, V: Kind, M: Expr<Map<K, V>>> MapTrait<K, V> for AsMap<K, V, M> {
        default type GetList = EmptyList<Pair<K, V>>;
    }

    impl<K: KindWithDefault + EqualityComparableKind, V: Kind, M: Expr<Map<K, V>>> MapTrait<K, V> for AsMap<K, V, M> where <<M as Expr<Map<K, V>>>::Eval as Value<Map<K, V>>>::UnconstrainedImpl: MapTrait<K, V> {
        type GetList = <<<M as Expr<Map<K, V>>>::Eval as Value<Map<K, V>>>::UnconstrainedImpl as MapTrait<K, V>>::GetList;
    }

    pub struct MapEquals<K: EqualityComparableKind, V: EqualityComparableKind, X: Expr<Map<K, V>>, Y: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        x: PhantomData<X>,
        y: PhantomData<Y>,
    }

    impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault + EqualityComparableKind, X: Expr<Map<K, V>>, Y: Expr<Map<K, V>>> Expr<Bool> for MapEquals<K, V, X, Y> {
        type Eval = <And<IsSubmap<K, V, X, Y>, IsSubmap<K, V, Y, X>> as Expr<Bool>>::Eval;
    }

    pub struct IsSubmap<K: EqualityComparableKind, V: EqualityComparableKind, CandidateSubmap: Expr<Map<K, V>>, CandidateSupermap: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        candidate_submap: PhantomData<CandidateSubmap>,
        candidate_supermap: PhantomData<CandidateSupermap>,
    }

    impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault + EqualityComparableKind, CandidateSubmap: Expr<Map<K, V>>, CandidateSupermap: Expr<Map<K, V>>> Expr<Bool> for IsSubmap<K, V, CandidateSubmap, CandidateSupermap> {
        type Eval = <VisitMap<K, V, Bool, CandidateSubmap, IsSubmapVisitor<K, V, CandidateSupermap>> as Expr<Bool>>::Eval;
    }

    pub struct IsSubmapVisitor<K: EqualityComparableKind, V: EqualityComparableKind, CandidateSupermap: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        candidate_supermap: PhantomData<CandidateSupermap>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: EqualityComparableKind + KindWithDefault, CandidateSupermap: Expr<Map<K, V>>> MapVisitor<K, V, Bool> for IsSubmapVisitor<K, V, CandidateSupermap> {
        type VisitEmptyMap = True;
        type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>> = And<
            Equals<Option<V>, MapGet<K, V, Key, CandidateSupermap>, Some<V, Value>>,
            VisitMap<K, V, Bool, Tail, IsSubmapVisitor<K, V, CandidateSupermap>>>;
    }
}
