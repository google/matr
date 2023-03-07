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

pub struct IsEven<N: Expr<USize>> {
    n: PhantomData<N>,
}

impl<N: Expr<USize>> Expr<Bool> for IsEven<N> {
    type Eval = IsEvenValue<N>;
}

pub struct IsEvenValue<N: Expr<USize>> {
    n: PhantomData<N>,
}

impl<N: Expr<USize>> BoolValue for IsEvenValue<N> {
    type Impl = AsBool<<AsUSize<N> as USizeTrait>::Visit<Bool, IsEvenVisitor>>;
}

pub struct IsOdd<N: Expr<USize>> {
    n: PhantomData<N>,
}

impl<N: Expr<USize>> Expr<Bool> for IsOdd<N> {
    type Eval = IsOddValue<N>;
}

pub struct IsOddValue<N: Expr<USize>> {
    n: PhantomData<N>,
}

impl<N: Expr<USize>> BoolValue for IsOddValue<N> {
    type Impl = AsBool<<AsUSize<N> as USizeTrait>::Visit<Bool, IsOddVisitor>>;
}

mod internal {
    pub use super::super::internal::*;

    pub struct IsEvenFunctor {}

    impl Functor1<USize, Result<Bool>> for IsEvenFunctor {
        type Apply<V1: Expr<USize>> = Ok<Bool, <AsUSize<V1> as USizeTrait>::Visit<Bool, IsEvenVisitor>>;
    }

    pub struct IsOddFunctor {}

    impl Functor1<USize, Result<Bool>> for IsOddFunctor {
        type Apply<V1: Expr<USize>> = Ok<Bool, <AsUSize<V1> as USizeTrait>::Visit<Bool, IsOddVisitor>>;
    }

    pub struct IsEvenVisitor {}

    impl USizeVisitor<Bool> for IsEvenVisitor {
        type VisitZero = True;
        type VisitIncrement<N: Expr<USize>> = <AsUSize<N> as USizeTrait>::Visit<Bool, IsOddVisitor>;
    }

    pub struct IsOddVisitor {}

    impl USizeVisitor<Bool> for IsOddVisitor {
        type VisitZero = False;
        type VisitIncrement<N: Expr<USize>> = <AsUSize<N> as USizeTrait>::Visit<Bool, IsEvenVisitor>;
    }
}

// TODO: add tests.
