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

pub struct OptionOrValue<K: Kind, V: Expr<Option<K>>, Fallback: Expr<K>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    fallback: PhantomData<Fallback>,
}

impl<K: Kind, V: Expr<Option<K>>, Fallback: Expr<K>> Expr<K> for OptionOrValue<K, V, Fallback> {
    type Eval = <VisitOption<K, K, V, OptionOrValueVisitor<K, V, Fallback>> as Expr<K>>::Eval;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct OptionOrValueVisitor<K: Kind, V: Expr<Option<K>>, Fallback: Expr<K>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        fallback: PhantomData<Fallback>,
    }

    impl<K: Kind, V: Expr<Option<K>>, Fallback: Expr<K>> OptionVisitor<K, K> for OptionOrValueVisitor<K, V, Fallback> {
        type VisitNone = Fallback;
        type VisitSome<V2: Expr<K>> = V2;
    }
}
