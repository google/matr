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

pub struct Ok<K: Kind, V: Expr<K>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K: Kind, V: Expr<K>> Expr<Result<K>> for Ok<K, V> {
    type Eval = OkValue<K, V>;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct OkValue<K: Kind, V: Expr<K>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<K: Kind, V: Expr<K>> ResultValue<K> for OkValue<K, V> {
        type Impl = OkImpl<K, V>;
    }

    pub struct OkImpl<K: Kind, V: Expr<K>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<K: Kind, V: Expr<K>> ResultTrait<K> for OkImpl<K, V> {
        type Visit<ResultK: Kind, Visitor: ResultVisitor<K, ResultK>> = Visitor::VisitOk<V>;
    }
}
