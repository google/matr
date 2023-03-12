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

pub struct ReverseList<K: Kind, L: Expr<List<K>>> {
    k: PhantomData<K>,
    l: PhantomData<L>,
}

impl<K: Kind, L: Expr<List<K>>> Expr<List<K>> for ReverseList<K, L> {
    type Eval = ReverseListValue<K, L>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ReverseListValue<K: Kind, L: Expr<List<K>>> {
        k: PhantomData<K>,
        l: PhantomData<L>,
    }

    impl<K: Kind, L: Expr<List<K>>> ListValue<K> for ReverseListValue<K, L> {
        type Impl = AsList<K, ReversedListConcat<K, L, EmptyList<K>>>;
    }
}
