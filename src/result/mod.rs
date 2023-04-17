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

mod result_or_value;
mod ok;
mod err;
mod check_no_error;
mod and_then2;
mod and_then;
mod get_type_result;

pub use result_or_value::*;
pub use ok::*;
pub use err::*;
pub use check_no_error::*;
pub use and_then2::*;
pub use and_then::*;
pub use get_type_result::*;

use std::marker::PhantomData;
use internal::*;

pub struct Result<K: Kind> {
    k: PhantomData<K>,
}

impl<K: Kind> Kind for Result<K> {}

impl<K: KindWithDefault> KindWithDefault for Result<K> {
    type Default = Ok<K, K::Default>;
}

pub trait ResultVisitor<K: Kind, ResultK: Kind> {
    type VisitOk<V: Expr<K>>: Expr<ResultK>;
    type VisitErr<E>: Expr<ResultK>;
}

pub struct VisitResult<K: Kind, OutK: Kind, R: Expr<Result<K>>, V: ResultVisitor<K, OutK>> {
    k: PhantomData<K>,
    out_k: PhantomData<OutK>,
    r: PhantomData<R>,
    v: PhantomData<V>,
}

impl<K: Kind, OutK: Kind, R: Expr<Result<K>>, V: ResultVisitor<K, OutK>> Expr<OutK> for VisitResult<K, OutK, R, V> {
    type Eval = <<AsResult<K, R> as ResultTrait<K>>::Visit<OutK, V> as Expr<OutK>>::Eval;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub trait ResultValue<K: Kind> {
        type Impl: ResultTrait<K>;
    }

    impl<K: Kind, R: ResultValue<K>> Value<Result<K>> for R {
        type UnconstrainedImpl = <R as ResultValue<K>>::Impl;
    }

    pub trait ResultTrait<K: Kind> {
        type Visit<ResultK: Kind, V: ResultVisitor<K, ResultK>>: Expr<ResultK>;
    }

    // Error returned by AsResult when it's called on a Value<Result<K>> that doesn't implement ResultTrait<K>.
    pub struct AsResultError {}

    pub struct AsResult<K: Kind, V: Expr<Result<K>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<K: Kind, V: Expr<Result<K>>> ResultTrait<K> for AsResult<K, V> {
        default type Visit<ResultK: Kind, Visitor: ResultVisitor<K, ResultK>> = Visitor::VisitErr<AsResultError>;
    }

    impl<K: Kind, V: Expr<Result<K>>> ResultTrait<K> for AsResult<K, V> where <<V as Expr<Result<K>>>::Eval as Value<Result<K>>>::UnconstrainedImpl: ResultTrait<K> {
        type Visit<ResultK: Kind, Visitor: ResultVisitor<K, ResultK>> = <<<V as Expr<Result<K>>>::Eval as Value<Result<K>>>::UnconstrainedImpl as ResultTrait<K>>::Visit<ResultK, Visitor>;
    }
}
