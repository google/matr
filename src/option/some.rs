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

pub struct Some<K: Kind, X: Expr<K>> {
    k: PhantomData<K>,
    x: PhantomData<X>,
}

impl<K: Kind, X: Expr<K>> Expr<Option<K>> for Some<K, X> {
    type Eval = SomeValue<K, X>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct SomeValue<K: Kind, X: Expr<K>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
    }

    impl<K: Kind, X: Expr<K>> OptionValue<K> for SomeValue<K, X> {
        type Impl = SomeImpl<K, X>;
    }

    pub struct SomeImpl<K: Kind, X: Expr<K>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
    }

    impl<K: Kind, X: Expr<K>> OptionTrait<K> for SomeImpl<K, X> {
        type Visit<ResultK: Kind, Visitor: OptionVisitor<K, ResultK>> = Visitor::VisitSome<X>;
    }
}
