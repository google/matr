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

pub struct AndThen2<K1: Kind, K2: Kind, ResultK: Kind, V1: Expr<Result<K1>>, V2: Expr<Result<K2>>, F: Functor2<K1, K2, Result<ResultK>>> {
    k1: PhantomData<K1>,
    k2: PhantomData<K2>,
    v1: PhantomData<V1>,
    v2: PhantomData<V2>,
    result_k: PhantomData<ResultK>,
    f: PhantomData<F>,
}

impl<K1: Kind, K2: Kind, ResultK: Kind, V1: Expr<Result<K1>>, V2: Expr<Result<K2>>, F: Functor2<K1, K2, Result<ResultK>>> Expr<Result<ResultK>> for AndThen2<K1, K2, ResultK, V1, V2, F> {
    type Eval = <AndThen<K1, ResultK, V1, AndThen2ImplStage1<K1, K2, ResultK, V2, F>> as Expr<Result<ResultK>>>::Eval;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct AndThen2ImplStage1<K1: Kind, K2: Kind, ResultK: Kind, Expr2: Expr<Result<K2>>, F: Functor2<K1, K2, Result<ResultK>>> {
        k1: PhantomData<K1>,
        k2: PhantomData<K2>,
        resultk: PhantomData<ResultK>,
        expr2: PhantomData<Expr2>,
        f: PhantomData<F>,
    }

    impl<K1: Kind, K2: Kind, ResultK: Kind, Expr2: Expr<Result<K2>>, F: Functor2<K1, K2, Result<ResultK>>> Functor1<K1, Result<ResultK>> for AndThen2ImplStage1<K1, K2, ResultK, Expr2, F> {
        type Apply<V1: Expr<K1>> = AndThen<K2, ResultK, Expr2, AndThen2ImplStage2<K1, K2, ResultK, V1, F>>;
    }

    pub struct AndThen2ImplStage2<K1: Kind, K2: Kind, ResultK: Kind, V1: Expr<K1>, F: Functor2<K1, K2, Result<ResultK>>> {
        k1: PhantomData<K1>,
        k2: PhantomData<K2>,
        resultk: PhantomData<ResultK>,
        v1: PhantomData<V1>,
        f: PhantomData<F>,
    }

    impl<K1: Kind, K2: Kind, ResultK: Kind, V1: Expr<K1>, F: Functor2<K1, K2, Result<ResultK>>> Functor1<K2, Result<ResultK>> for AndThen2ImplStage2<K1, K2, ResultK, V1, F> {
        type Apply<V2: Expr<K2>> = F::Apply<V1, V2>;
    }
}
