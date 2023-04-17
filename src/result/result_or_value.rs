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

// Intended to be used at toplevel, in conjunction with a call to check_no_error with the same Expr<Type>.
pub struct ResultOrValue<K: Kind, V: Expr<Result<K>>, Fallback: Expr<K>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    fallback: PhantomData<Fallback>,
}

impl<K: Kind, V: Expr<Result<K>>, Fallback: Expr<K>> Expr<K> for ResultOrValue<K, V, Fallback> {
    type Eval = <VisitResult<K, K, V, ResultOrValueVisitor<K, V, Fallback>> as Expr<K>>::Eval;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ResultOrValueVisitor<K: Kind, V: Expr<Result<K>>, Fallback: Expr<K>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        fallback: PhantomData<Fallback>,
    }

    impl<K: Kind, V: Expr<Result<K>>, Fallback: Expr<K>> ResultVisitor<K, K> for ResultOrValueVisitor<K, V, Fallback> {
        type VisitOk<V2: Expr<K>> = V2;
        type VisitErr<E> = Fallback;
    }
}
