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

pub struct MapToList<K: EqualityComparableKind, V: Kind, S: Expr<Map<K, V>>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    s: PhantomData<S>,
}

impl<K: EqualityComparableKind, V: Kind, S: Expr<Map<K, V>>> Expr<List<Pair<K, V>>> for MapToList<K, V, S> {
    type Eval = MapToListValue<K, V, S>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct MapToListValue<K: EqualityComparableKind, V: Kind, S: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind, V: Kind, S: Expr<Map<K, V>>> ListValue<Pair<K, V>> for MapToListValue<K, V, S> {
        type Impl = AsList<Pair<K, V>, <AsMap<K, V, S> as MapTrait<K, V>>::GetList>;
    }
}
