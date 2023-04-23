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

pub struct ExprWrapper<K: Kind + ?Sized> {
    k: PhantomData<K>,
}

impl<K: Kind + ?Sized> Kind for ExprWrapper<K> {}

pub trait ExprWrapperValue<K: Kind> {
    type UnconstrainedImpl;
}

meta!{
    pub struct WrapExpr<
        K: Kind,
        E: Expr<K>
    >: Expr<ExprWrapper<K>> {
        type Eval = WrapExprValue<K, E>;
    }
}


impl<K: Kind, E: ExprWrapperValue<K>> Value<ExprWrapper<K>> for E {
    type UnconstrainedImpl = <E as ExprWrapperValue<K>>::UnconstrainedImpl;
}

pub trait AsWrappedExprTrait<K: KindWithDefault> {
    type Get: Expr<K>;
}

pub struct AsWrappedExpr<K: KindWithDefault, E: Expr<ExprWrapper<K>>> {
    k: PhantomData<K>,
    e: PhantomData<E>,
}

impl<K: KindWithDefault, E: Expr<ExprWrapper<K>>> AsWrappedExprTrait<K> for AsWrappedExpr<K, E> {
    default type Get = K::Default;
}

impl<K: KindWithDefault, E: Expr<ExprWrapper<K>>> AsWrappedExprTrait<K> for AsWrappedExpr<K, E> where <<E as Expr<ExprWrapper<K>>>::Eval as Value<ExprWrapper<K>>>::UnconstrainedImpl: Expr<K> {
    type Get = <<E as Expr<ExprWrapper<K>>>::Eval as Value<ExprWrapper<K>>>::UnconstrainedImpl;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use crate::*;

    meta!{
        pub struct WrapExprValue<
            K: Kind,
            E: Expr<K>
        >: ExprWrapperValue<K> {
            type UnconstrainedImpl = E;
        }
    }
}
