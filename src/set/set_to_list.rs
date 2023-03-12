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

pub struct SetToList<K: EqualityComparableKind, S: Expr<Set<K>>> {
    k: PhantomData<K>,
    s: PhantomData<S>,
}

impl<K: EqualityComparableKind, S: Expr<Set<K>>> Expr<List<K>> for SetToList<K, S> {
    type Eval = SetToListValue<K, S>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct SetToListValue<K: EqualityComparableKind, S: Expr<Set<K>>> {
        k: PhantomData<K>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind, S: Expr<Set<K>>> ListValue<K> for SetToListValue<K, S> {
        type Impl = AsList<K, <AsSet<K, S> as SetTrait<K>>::GetList>;
    }
}
