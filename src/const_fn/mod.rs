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

mod no_op;
mod call;
mod to_result_const_fn;

pub use no_op::*;
pub use call::*;
pub use to_result_const_fn::*;

use std::marker::PhantomData;
use internal::*;

pub struct ConstFn<Result, Args> {
    result: PhantomData<Result>,
    args: PhantomData<Args>,
}

impl<Result, Args> Kind for ConstFn<Result, Args> {}

#[const_trait]
pub trait ConstFnTrait<Result, Args> {
    fn apply(args: Args) -> Result;
}

meta!{
    pub struct WrapConstFn<
        Result,
        Args,
        F: ~const ConstFnTrait<Result, Args>
    >: Expr<ConstFn<Result, Args>> {
        type Eval = WrapConstFnValue<Result, Args, WrapConstFnImpl<Result, Args, F>>;
    }
}

// The contents of this module have to be "pub" because otherwise Rust would complain that
// "can't leak private type". The wrapping non-pub mod ensures that these are never explicitly
// referenced outside.
mod internal {
    use std::marker::PhantomData;
    pub use crate::*;
    pub use super::*;

    pub struct AsConstFn<Result, Args, Fn: Expr<ConstFn<Result, Args>>> {
        result: PhantomData<Result>,
        args: PhantomData<Args>,
        f: PhantomData<Fn>,
    }

    impl<Result, Args, Fn: Expr<ConstFn<Result, Args>>> ConstFnTraitWrapper<Result, Args> for AsConstFn<Result, Args, Fn> {
        default type Fn = PanicWithAsConstFnError<Result, Args, Fn>;
    }

    impl<Result, Args, Fn: Expr<ConstFn<Result, Args>>> ConstFnTraitWrapper<Result, Args> for AsConstFn<Result, Args, Fn>
        where <<Fn as Expr<ConstFn<Result, Args>>>::Eval as Value<ConstFn<Result, Args>>>::UnconstrainedImpl: ConstFnTraitWrapper<Result, Args> {
        type Fn = <<<Fn as Expr<ConstFn<Result, Args>>>::Eval as Value<ConstFn<Result, Args>>>::UnconstrainedImpl as ConstFnTraitWrapper<Result, Args>>::Fn;
    }

    // This is extracted as a separate function so that the build error message shows the expr, value
    // and impl that caused the error, to simplify debugging.
    const fn panic_with_as_const_fn_error<Result, Args, FnExpr, FnValue, FnImpl>() -> ! {
        panic!("AsConstFn was called on a Value<ConstFn<...>> that does not implement const ConstFnTraitWrapper")
    }

    pub trait ConstFnValue<Result, Args> {
        type Impl: ConstFnTraitWrapper<Result, Args>;
    }

    meta!{
        pub struct WrapConstFnValue<
            Result,
            Args,
            U: ConstFnValue<Result, Args>
        >: Value<ConstFn<Result, Args>> {
            type UnconstrainedImpl = <U as ConstFnValue<Result, Args>>::Impl;
        }
    }

    // This wrapper is needed (rather than using ConstFnTrait as trait directly) so that we can
    // specialize based on whether ConstFnTraitWrapper is implemented or not.
    // Specializing on const traits doesn't seem to work ATM (as of Rust nightly 2023-04-20), if that
    // becomes supported then we can get rid of this.
    pub trait ConstFnTraitWrapper<Result, Args> {
        type Fn: ~const ConstFnTrait<Result, Args>;
    }

    meta!{
        pub struct WrapConstFnImpl<
            Result,
            Args,
            F: ~const ConstFnTrait<Result, Args>
        >: ConstFnValue<Result, Args> {
            type Impl = WrapConstFnTrait<Result, Args, F>;
        }

        pub struct WrapConstFnTrait<
            Result,
            Args,
            F: ~const ConstFnTrait<Result, Args>
        >: ConstFnTraitWrapper<Result, Args> {
            type Fn = F;
        }

        pub struct PanicWithAsConstFnError<Result, Args, Fn: Expr<ConstFn<Result, Args>>>: const ConstFnTrait<Result, Args> {
            fn apply(_: Args) -> Result {
                panic_with_as_const_fn_error::<
                    Result,
                    Args,
                    Fn,
                    <Fn as Expr<ConstFn<Result, Args>>>::Eval,
                    <<Fn as Expr<ConstFn<Result, Args>>>::Eval as Value<ConstFn<Result, Args>>>::UnconstrainedImpl
                >();
            }
        }
    }
}
