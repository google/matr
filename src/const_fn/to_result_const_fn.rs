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

use std::marker::Destruct;
use internal::*;

meta!{
    pub type ToResultConstFn<
        Out,
        Args: ~const Destruct,
        E: Expr<Result<ConstFn<Out, Args>>>
    >: Expr<ConstFn<std::result::Result<Out, &'static str>, Args>> =
        ResultOrValue<
            ConstFn<std::result::Result<Out, &'static str>, Args>,
            AndThen<
                ConstFn<Out, Args>,
                ConstFn<std::result::Result<Out, &'static str>, Args>,
                E,
                ToResultConstFnAdapter<Out, Args>>,
            ToResultConstFnError<Out, Args>>;
}

mod internal {
    use std::marker::Destruct;
    pub use super::super::internal::*;

    meta!{
        pub struct ToResultConstFnAdapter<Out, Args>: Functor1<ConstFn<Out, Args>, Result<ConstFn<std::result::Result<Out, &'static str>, Args>>> {
            type Apply<X: Expr<ConstFn<Out, Args>>> = Ok<ConstFn<std::result::Result<Out, &'static str>, Args>, ToResultConstFnAdapterExpr<Out, Args, X>>;
        }

        pub struct ToResultConstFnAdapterExpr<
            Out,
            Args,
            X: Expr<ConstFn<Out, Args>>
        >: Expr<ConstFn<std::result::Result<Out, &'static str>, Args>> {
            type Eval = ToResultConstFnAdapterValue<Out, Args, X>;
        }

        pub struct ToResultConstFnAdapterValue<
            Out,
            Args,
            X: Expr<ConstFn<Out, Args>>
        >: ConstFnValue<std::result::Result<Out, &'static str>, Args> {
            type Impl = ToResultConstFnAdapterImplWrapper<Out, Args, X>;
        }

        pub struct ToResultConstFnAdapterImplWrapper<Out, Args, X: Expr<ConstFn<Out, Args>>>: ConstFnTraitWrapper<std::result::Result<Out, &'static str>, Args> {
            type Fn = ToResultConstFnAdapterImpl<Out, Args, X>;
        }

        pub struct ToResultConstFnAdapterImpl<Out, Args, X: Expr<ConstFn<Out, Args>>>: const ConstFnTrait<std::result::Result<Out, &'static str>, Args> {
            fn apply(args: Args) -> std::result::Result<Out, &'static str> {
                return Ok(call_const_fn::<Out, Args, X>(args));
            }
        }

        pub struct ToResultConstFnError<Out, Args: ~const Destruct>: Expr<ConstFn<std::result::Result<Out, &'static str>, Args>> {
            type Eval = ToResultConstFnErrorValue<Out, Args>;
        }

        pub struct ToResultConstFnErrorValue<Out, Args: ~const Destruct>: ConstFnValue<std::result::Result<Out, &'static str>, Args> {
            type Impl = ToResultConstFnErrorImplWrapper<Out, Args>;
        }

        pub struct ToResultConstFnErrorImplWrapper<Out, Args: ~const Destruct>: ConstFnTraitWrapper<std::result::Result<Out, &'static str>, Args> {
            type Fn = ToResultConstFnErrorImpl<Out, Args>;
        }

        pub struct ToResultConstFnErrorImpl<Out, Args: ~const Destruct>: const ConstFnTrait<std::result::Result<Out, &'static str>, Args> {
            fn apply(_: Args) -> std::result::Result<Out, &'static str> {
                return Err("error Result in ToResultConstFn")
            }
        }
    }
}
