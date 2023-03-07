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

use std::marker::{PhantomData};
use internal::*;

pub struct ToResultRuntimeFn<Out, Args, E: Expr<Result<RuntimeFn<Out, Args>>>> {
    out: PhantomData<Out>,
    args: PhantomData<Args>,
    e: PhantomData<E>,
}

impl<Out, Args, E: Expr<Result<RuntimeFn<Out, Args>>>> Expr<RuntimeFn<std::result::Result<Out, &'static str>, Args>> for ToResultRuntimeFn<Out, Args, E> {
    type Eval = <OrValue<RuntimeFn<std::result::Result<Out, &'static str>, Args>, AndThen<RuntimeFn<Out, Args>, RuntimeFn<std::result::Result<Out, &'static str>, Args>, E, ToResultRuntimeFnAdapter>, ToResultRuntimeFnError> as Expr<RuntimeFn<std::result::Result<Out, &'static str>, Args>>>::Eval;
}

mod internal {
    use std::marker::{PhantomData};
    pub use super::super::internal::*;

    pub struct ToResultRuntimeFnAdapter {}

    impl<Out, Args> Functor1<RuntimeFn<Out, Args>, Result<RuntimeFn<std::result::Result<Out, &'static str>, Args>>> for ToResultRuntimeFnAdapter {
        type Apply<X: Expr<RuntimeFn<Out, Args>>> = Ok<RuntimeFn<std::result::Result<Out, &'static str>, Args>, ToResultRuntimeFnAdapterExpr<Out, Args, X>>;
    }

    pub struct ToResultRuntimeFnAdapterExpr<Out, Args, X: Expr<RuntimeFn<Out, Args>>> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
        x: PhantomData<X>,
    }

    impl<Out, Args, X: Expr<RuntimeFn<Out, Args>>> Expr<RuntimeFn<std::result::Result<Out, &'static str>, Args>> for ToResultRuntimeFnAdapterExpr<Out, Args, X> {
        type Eval = ToResultRuntimeFnAdapterValue<Out, Args, X>;
    }

    pub struct ToResultRuntimeFnAdapterValue<Out, Args, X: Expr<RuntimeFn<Out, Args>>> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
        x: PhantomData<X>,
    }

    impl<Out, Args, X: Expr<RuntimeFn<Out, Args>>> RuntimeFnValue<std::result::Result<Out, &'static str>, Args> for ToResultRuntimeFnAdapterValue<Out, Args, X> {
        type Impl = ToResultRuntimeFnAdapterImpl<Out, Args, X>;
    }

    pub struct ToResultRuntimeFnAdapterImpl<Out, Args, X: Expr<RuntimeFn<Out, Args>>> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
        x: PhantomData<X>,
    }

    impl<Out, Args, X: Expr<RuntimeFn<Out, Args>>> RuntimeFnTrait<std::result::Result<Out, &'static str>, Args> for ToResultRuntimeFnAdapterImpl<Out, Args, X> {
        fn apply(args: Args) -> std::result::Result<Out, &'static str> {
            return Ok(call_runtime_fn::<Out, Args, X>(args));
        }
    }

    pub struct ToResultRuntimeFnError {}

    impl<Out, Args> Expr<RuntimeFn<std::result::Result<Out, &'static str>, Args>> for ToResultRuntimeFnError {
        type Eval = ToResultRuntimeFnErrorValue<Out, Args>;
    }

    pub struct ToResultRuntimeFnErrorValue<Out, Args> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
    }

    impl<Out, Args> RuntimeFnValue<std::result::Result<Out, &'static str>, Args> for ToResultRuntimeFnErrorValue<Out, Args> {
        type Impl = ToResultRuntimeFnErrorImpl<Out, Args>;
    }

    pub struct ToResultRuntimeFnErrorImpl<Out, Args> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
    }

    impl<Out, Args> RuntimeFnTrait<std::result::Result<Out, &'static str>, Args> for ToResultRuntimeFnErrorImpl<Out, Args> {
        fn apply(_: Args) -> std::result::Result<Out, &'static str> {
            return Err("error Result in ToResultRuntimeFn")
        }
    }
}
