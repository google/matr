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

pub struct ListToMap<K: EqualityComparableKind, V: Kind, L: Expr<List<Pair<K, V>>>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    l: PhantomData<L>,
}

impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, L: Expr<List<Pair<K, V>>>> Expr<Map<K, V>> for ListToMap<K, V, L> {
    type Eval = <VisitList<Pair<K, V>, Map<K, V>, L, ListToMapVisitor<K, V>> as Expr<Map<K, V>>>::Eval;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ListToMapVisitor<K: EqualityComparableKind, V: Kind> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault> ListVisitor<Pair<K, V>, Map<K, V>> for ListToMapVisitor<K, V> {
        type VisitEmptyList = EmptyMap<K, V>;
        type VisitCons<Elem: Expr<Pair<K, V>>, Tail: Expr<List<Pair<K, V>>>> = Put<K, V, GetFirst<K, V, Elem>, GetSecond<K, V, Elem>, ListToMap<K, V, Tail>>;
    }
}
