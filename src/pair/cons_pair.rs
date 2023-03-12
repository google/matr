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

pub struct ConsPair<FirstK: Kind, SecondK: Kind, First: Expr<FirstK>, Second: Expr<SecondK>> {
    first_k: PhantomData<FirstK>,
    second_k: PhantomData<SecondK>,
    first: PhantomData<First>,
    second: PhantomData<Second>,
}

impl<FirstK: Kind, SecondK: Kind, First: Expr<FirstK>, Second: Expr<SecondK>> Expr<Pair<FirstK, SecondK>> for ConsPair<FirstK, SecondK, First, Second> {
    type Eval = ConsPairValue<FirstK, SecondK, First, Second>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ConsPairValue<FirstK: Kind, SecondK: Kind, First: Expr<FirstK>, Second: Expr<SecondK>> {
        first_k: PhantomData<FirstK>,
        second_k: PhantomData<SecondK>,
        first: PhantomData<First>,
        second: PhantomData<Second>,
    }

    impl<FirstK: Kind, SecondK: Kind, First: Expr<FirstK>, Second: Expr<SecondK>> PairValue<FirstK, SecondK> for ConsPairValue<FirstK, SecondK, First, Second> {
        type Impl = ConsPairImpl<FirstK, SecondK, First, Second>;
    }

    pub struct ConsPairImpl<FirstK: Kind, SecondK: Kind, First: Expr<FirstK>, Second: Expr<SecondK>> {
        first_k: PhantomData<FirstK>,
        second_k: PhantomData<SecondK>,
        first: PhantomData<First>,
        second: PhantomData<Second>,
    }

    impl<FirstK: Kind, SecondK: Kind, First: Expr<FirstK>, Second: Expr<SecondK>> PairTrait<FirstK, SecondK> for ConsPairImpl<FirstK, SecondK, First, Second> {
        type Visit<ResultK: Kind, Visitor: PairVisitor<FirstK, SecondK, ResultK>> = Visitor::Visit<First, Second>;
    }
}
