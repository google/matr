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

pub struct IsNone<K: Kind, P: Expr<Option<K>>> {
    k: PhantomData<K>,
    p: PhantomData<P>,
}

impl<K: Kind, X: Expr<Option<K>>> Expr<Bool> for IsNone<K, X> {
    type Eval = <VisitOption<K, Bool, X, IsNoneVisitor<K>> as Expr<Bool>>::Eval;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct IsNoneVisitor<K: Kind> {
        k: PhantomData<K>,
    }

    impl<K: Kind> OptionVisitor<K, Bool> for IsNoneVisitor<K> {
        type VisitNone = True;
        type VisitSome<X: Expr<K>> = False;
    }
}
