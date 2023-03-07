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

pub struct Err<K: Kind, E> {
    k: PhantomData<K>,
    e: PhantomData<E>,
}

impl<K: Kind, E> Expr<Result<K>> for Err<K, E> {
    type Eval = ErrValue<K, E>;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ErrImpl<K: Kind, E> {
        k: PhantomData<K>,
        e: PhantomData<E>,
    }

    impl<K: Kind, E> ResultTrait<K> for ErrImpl<K, E> {
        type Visit<ResultK: Kind, Visitor: ResultVisitor<K, ResultK>> = Visitor::VisitErr<E>;
    }

    pub struct ErrValue<K: Kind, E> {
        k: PhantomData<K>,
        e: PhantomData<E>,
    }

    impl<K: Kind, E> ResultValue<K> for ErrValue<K, E> {
        type Impl = ErrImpl<K, E>;
    }
}
