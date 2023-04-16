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

mod zero;
mod increment;
mod sum;
mod multiply;
mod is_even;
mod to_usize;
mod long_recursion;

use std::marker::PhantomData;
pub use zero::*;
pub use increment::*;
pub use sum::*;
pub use multiply::*;
pub use is_even::*;
pub use to_usize::*;
pub use long_recursion::*;

use internal::*;

pub struct USize {}

impl Kind for USize {}

impl EqualityComparableKind for USize {
    type Eq<X: Expr<USize>, Y: Expr<USize>> = USizeEquals<X, Y>;
}

impl KindWithDefault for USize {
    type Default = Zero;
}

pub trait USizeVisitor<ResultK: Kind> {
    type VisitZero: Expr<ResultK>;
    type VisitIncrement<N: Expr<USize>>: Expr<ResultK>;
}

pub struct VisitUSize<K: Kind, N: Expr<USize>, V: USizeVisitor<K>> {
    k: PhantomData<K>,
    n: PhantomData<N>,
    v: PhantomData<V>,
}

impl<K: Kind, N: Expr<USize>, V: USizeVisitor<K>> Expr<K> for VisitUSize<K, N, V> {
    type Eval = <<AsUSize<N> as USizeTrait>::Visit<K, V> as Expr<K>>::Eval;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub trait USizeValue {
        type Impl: USizeTrait;
    }

    impl<U: USizeValue> Value<USize> for U {
        type UnconstrainedImpl = <U as USizeValue>::Impl;
    }

    pub struct AsUSize<N: Expr<USize>> {
        n: PhantomData<N>,
    }

    impl<N: Expr<USize>> USizeTrait for AsUSize<N> {
        default type Visit<ResultK: Kind, V: USizeVisitor<ResultK>> = V::VisitZero;
    }

    impl<N: Expr<USize>> USizeTrait for AsUSize<N> where <<N as Expr<USize>>::Eval as Value<USize>>::UnconstrainedImpl: USizeTrait {
        type Visit<ResultK: Kind, V: USizeVisitor<ResultK>> = <<<N as Expr<USize>>::Eval as Value<USize>>::UnconstrainedImpl as USizeTrait>::Visit<ResultK, V>;
    }

    pub trait USizeTrait {
        type Visit<ResultK: Kind, V: USizeVisitor<ResultK>>: Expr<ResultK>;
    }

    pub struct USizeEquals<X: Expr<USize>, Y: Expr<USize>> {
        x: PhantomData<X>,
        y: PhantomData<Y>,
    }

    impl<X: Expr<USize>, Y: Expr<USize>> Expr<Bool> for USizeEquals<X, Y> {
        type Eval = <VisitUSize<Bool, X, USizeEqualsVisitor<Y>> as Expr<Bool>>::Eval;
    }

    pub struct USizeEqualsVisitor<Other: Expr<USize>> {
        other: PhantomData<Other>,
    }

    impl<Other: Expr<USize>> USizeVisitor<Bool> for USizeEqualsVisitor<Other> {
        type VisitZero = <AsUSize<Other> as USizeTrait>::Visit<Bool, IsZero>;
        type VisitIncrement<N: Expr<USize>> = <AsUSize<Other> as USizeTrait>::Visit<Bool, USizeEqualsOnePlus<N>>;
    }

    pub struct IsZero {}

    impl USizeVisitor<Bool> for IsZero {
        type VisitZero = True;
        type VisitIncrement<N: Expr<USize>> = False;
    }

    pub struct USizeEqualsOnePlus<Other: Expr<USize>> {
        other: PhantomData<Other>,
    }

    impl<Other: Expr<USize>> USizeVisitor<Bool> for USizeEqualsOnePlus<Other> {
        type VisitZero = False;
        type VisitIncrement<N: Expr<USize>> = <AsUSize<N> as USizeTrait>::Visit<Bool, USizeEqualsVisitor<Other>>;
    }
}
