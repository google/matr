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
mod to_result_runtime_fn;

pub use no_op::*;
pub use call::*;
pub use to_result_runtime_fn::*;

use std::marker::PhantomData;
use internal::*;

pub struct RuntimeFn<Result, Args> {
    result: PhantomData<Result>,
    args: PhantomData<Args>,
}

impl<Result, Args> Kind for RuntimeFn<Result, Args> {}

pub trait RuntimeFnTrait<Result, Args> {
    fn apply(args: Args) -> Result;
}

pub trait RuntimeFnValue<Result, Args> {
    type Impl: RuntimeFnTrait<Result, Args>;
}

impl<Result, Args, U: RuntimeFnValue<Result, Args>> Value<RuntimeFn<Result, Args>> for U {
    type UnconstrainedImpl = <U as RuntimeFnValue<Result, Args>>::Impl;
}

// The contents of this module have to be "pub" because otherwise Rust would complain that
// "can't leak private type". The wrapping non-pub mod ensures that these are never explicitly
// referenced outside.
mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub struct AsRuntimeFn<Result, Args, Fn: Expr<RuntimeFn<Result, Args>>> {
        result: PhantomData<Result>,
        args: PhantomData<Args>,
        f: PhantomData<Fn>,
    }

    impl<Result, Args, Fn: Expr<RuntimeFn<Result, Args>>> RuntimeFnTrait<Result, Args> for AsRuntimeFn<Result, Args, Fn> {
        default fn apply(_: Args) -> Result {
            panic!("AsRuntimeFn was called on a Value<RuntimeFn<...>> that does not implement RuntimeFnTrait")
        }
    }

    impl<Result, Args, Fn: Expr<RuntimeFn<Result, Args>>> RuntimeFnTrait<Result, Args> for AsRuntimeFn<Result, Args, Fn>
        where <<Fn as Expr<RuntimeFn<Result, Args>>>::Eval as Value<RuntimeFn<Result, Args>>>::UnconstrainedImpl: RuntimeFnTrait<Result, Args> {
        fn apply(args: Args) -> Result {
            return <<<Fn as Expr<RuntimeFn<Result, Args>>>::Eval as Value<RuntimeFn<Result, Args>>>::UnconstrainedImpl as RuntimeFnTrait<Result, Args>>::apply(args);
        }
    }
}
