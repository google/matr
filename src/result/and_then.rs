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
use crate::*;
use internal::*;

pub struct AndThen<K: Kind, ResultK: Kind, V: Expr<Result<K>>, F: Functor1<K, Result<ResultK>>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    result_k: PhantomData<ResultK>,
    f: PhantomData<F>,
}

impl<K: Kind, Value: Expr<Result<K>>, ResultK: Kind, F: Functor1<K, Result<ResultK>>> Expr<Result<ResultK>> for AndThen<K, ResultK, Value, F> {
    type Eval = <<AsResult<K, Value> as ResultTrait<K>>::Visit<Result<ResultK>, ValueMapperToVisitorAdapterForAndThen<K, ResultK, F>> as Expr<Result<ResultK>>>::Eval;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ValueMapperToVisitorAdapterForAndThen<K: Kind, ResultK: Kind, M: Functor1<K, Result<ResultK>>> {
        k: PhantomData<K>,
        result_k: PhantomData<ResultK>,
        m: PhantomData<M>,
    }

    impl<K: Kind, ResultK: Kind, M: Functor1<K, Result<ResultK>>> ResultVisitor<K, Result<ResultK>> for ValueMapperToVisitorAdapterForAndThen<K, ResultK, M> {
        type VisitOk<V: Expr<K>> = M::Apply<V>;
        type VisitErr<E> = Err<ResultK, E>;
    }
}
