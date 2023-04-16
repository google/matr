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

pub struct MapEntrySet<K: EqualityComparableKind, V: Kind, M: Expr<Map<K, V>>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    m: PhantomData<M>,
}

impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault + EqualityComparableKind, M: Expr<Map<K, V>>> Expr<Set<Pair<K, V>>> for MapEntrySet<K, V, M> {
    type Eval = MapEntrySetValue<K, V, M>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct MapEntrySetValue<K: EqualityComparableKind, V: Kind, M: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        m: PhantomData<M>,
    }

    impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault + EqualityComparableKind, M: Expr<Map<K, V>>> SetValue<Pair<K, V>> for MapEntrySetValue<K, V, M> {
        type Impl = AsSet<Pair<K, V>,
            <AsList<Pair<K, V>, <AsMap<K, V, M> as MapTrait<K, V>>::GetList> as ListTrait<Pair<K, V>>>::Visit<Set<Pair<K, V>>, MapEntrySetVisitor<K, V>>
        >;
    }

    pub struct MapEntrySetVisitor<K: EqualityComparableKind, V: Kind> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault + EqualityComparableKind> ListVisitor<Pair<K, V>, Set<Pair<K, V>>> for MapEntrySetVisitor<K, V> {
        type VisitEmptyList = EmptySet<Pair<K, V>>;
        type VisitCons<Elem: Expr<Pair<K, V>>, Tail: Expr<List<Pair<K, V>>>> = AddToSet<Pair<K, V>, Elem, <AsList<Pair<K, V>, Tail> as ListTrait<Pair<K, V>>>::Visit<Set<Pair<K, V>>, MapEntrySetVisitor<K, V>>>;
    }
}
