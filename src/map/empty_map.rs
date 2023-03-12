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

pub struct EmptyMap<K: EqualityComparableKind, V: Kind> {
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K: EqualityComparableKind, V: Kind> Expr<Map<K, V>> for EmptyMap<K, V> {
    type Eval = EmptyMapValue<K, V>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct EmptyMapValue<K: EqualityComparableKind, V: Kind> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<K: EqualityComparableKind, V: Kind> MapValue<K, V> for EmptyMapValue<K, V> {
        type Impl = EmptyMapImpl<K, V>;
    }

    pub struct EmptyMapImpl<K: EqualityComparableKind, V: Kind> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<K: EqualityComparableKind, V: Kind> MapTrait<K, V> for EmptyMapImpl<K, V> {
        type GetList = EmptyList<K>;
    }
}
