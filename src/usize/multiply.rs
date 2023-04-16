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

pub struct Multiply<X: Expr<USize>, Y: Expr<USize>> {
    x: PhantomData<X>,
    y: PhantomData<Y>,
}

impl<X: Expr<USize>, Y: Expr<USize>> Expr<USize> for Multiply<X, Y> {
    type Eval = <VisitUSize<USize, X, MultiplyValueVisitor<Y>> as Expr<USize>>::Eval;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct MultiplyValueVisitor<N: Expr<USize>> {
        n: PhantomData<N>,
    }

    impl<X: Expr<USize>> USizeVisitor<USize> for MultiplyValueVisitor<X> {
        type VisitZero = X;
        type VisitIncrement<Y: Expr<USize>> = Sum<X, Multiply<X, Y>>;
    }

}

// TODO: add tests
