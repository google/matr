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

use std::marker::{Destruct, PhantomData};
use internal::*;

pub struct ToResultConstFn<Out, Args: ~const Destruct, E: Expr<Result<ConstFn<Out, Args>>>> {
    out: PhantomData<Out>,
    args: PhantomData<Args>,
    e: PhantomData<E>,
}

impl<Out, Args: ~const Destruct, E: Expr<Result<ConstFn<Out, Args>>>> Expr<ConstFn<std::result::Result<Out, &'static str>, Args>> for ToResultConstFn<Out, Args, E> {
    type Eval = <OrValue<ConstFn<std::result::Result<Out, &'static str>, Args>, AndThen<ConstFn<Out, Args>, ConstFn<std::result::Result<Out, &'static str>, Args>, E, ToResultConstFnAdapter>, ToResultConstFnError> as Expr<ConstFn<std::result::Result<Out, &'static str>, Args>>>::Eval;
}

mod internal {
    use std::marker::{Destruct, PhantomData};
    pub use super::super::internal::*;

    pub struct ToResultConstFnAdapter {}

    impl<Out, Args> Functor1<ConstFn<Out, Args>, Result<ConstFn<std::result::Result<Out, &'static str>, Args>>> for ToResultConstFnAdapter {
        type Apply<X: Expr<ConstFn<Out, Args>>> = Ok<ConstFn<std::result::Result<Out, &'static str>, Args>, ToResultConstFnAdapterExpr<Out, Args, X>>;
    }

    pub struct ToResultConstFnAdapterExpr<Out, Args, X: Expr<ConstFn<Out, Args>>> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
        x: PhantomData<X>,
    }

    impl<Out, Args, X: Expr<ConstFn<Out, Args>>> Expr<ConstFn<std::result::Result<Out, &'static str>, Args>> for ToResultConstFnAdapterExpr<Out, Args, X> {
        type Eval = ToResultConstFnAdapterValue<Out, Args, X>;
    }

    pub struct ToResultConstFnAdapterValue<Out, Args, X: Expr<ConstFn<Out, Args>>> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
        x: PhantomData<X>,
    }

    impl<Out, Args, X: Expr<ConstFn<Out, Args>>> ConstFnValue<std::result::Result<Out, &'static str>, Args> for ToResultConstFnAdapterValue<Out, Args, X> {
        type Impl = ToResultConstFnAdapterImpl<Out, Args, X>;
    }

    pub struct ToResultConstFnAdapterImpl<Out, Args, X: Expr<ConstFn<Out, Args>>> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
        x: PhantomData<X>,
    }

    impl<Out, Args, X: Expr<ConstFn<Out, Args>>> const ConstFnTrait<std::result::Result<Out, &'static str>, Args> for ToResultConstFnAdapterImpl<Out, Args, X> {
        fn apply(args: Args) -> std::result::Result<Out, &'static str> {
            return Ok(call_const_fn::<Out, Args, X>(args));
        }
    }

    pub struct ToResultConstFnError {}

    impl<Out, Args: ~const Destruct> Expr<ConstFn<std::result::Result<Out, &'static str>, Args>> for ToResultConstFnError {
        type Eval = ToResultConstFnErrorValue<Out, Args>;
    }

    pub struct ToResultConstFnErrorValue<Out, Args> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
    }

    impl<Out, Args: ~const Destruct> ConstFnValue<std::result::Result<Out, &'static str>, Args> for ToResultConstFnErrorValue<Out, Args> {
        type Impl = ToResultConstFnErrorImpl<Out, Args>;
    }

    pub struct ToResultConstFnErrorImpl<Out, Args> {
        out: PhantomData<Out>,
        args: PhantomData<Args>,
    }

    impl<Out, Args: ~const Destruct> const ConstFnTrait<std::result::Result<Out, &'static str>, Args> for ToResultConstFnErrorImpl<Out, Args> {
        fn apply(_: Args) -> std::result::Result<Out, &'static str> {
            return Err("error Result in ToResultConstFn")
        }
    }
}
