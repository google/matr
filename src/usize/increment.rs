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

pub struct Increment<N: Expr<USize>> {
    n: PhantomData<N>,
}

impl<N: Expr<USize>> Expr<USize> for Increment<N> {
    type Eval = IncrementValue<N>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct IncrementValue<N: Expr<USize>> {
        n: PhantomData<N>,
    }

    impl<N: Expr<USize>> USizeValue for IncrementValue<N> {
        type Impl = IncrementImpl<N>;
    }

    pub struct IncrementFunctor {}

    impl Functor1<USize, Result<USize>> for IncrementFunctor {
        type Apply<V1: Expr<USize>> = Ok<USize, Increment<V1>>;
    }

    pub struct IncrementImpl<N: Expr<USize>> {
        n: PhantomData<N>,
    }

    impl<N: Expr<USize>> USizeTrait for IncrementImpl<N> {
        type Visit<ResultK: Kind, V: USizeVisitor<ResultK>> = V::VisitIncrement<N>;
    }
}
