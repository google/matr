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

pub trait ConstFnValue<Result, Args> {
    type Impl: ~const ConstFnTrait<Result, Args>;
}

impl<Result, Args, U: ConstFnValue<Result, Args>> Value<ConstFn<Result, Args>> for U {
    type UnconstrainedImpl = <U as ConstFnValue<Result, Args>>::Impl;
}

pub struct AsConstFn<Result, Args, Fn: Expr<ConstFn<Result, Args>>> {
    result: PhantomData<Result>,
    args: PhantomData<Args>,
    f: PhantomData<Fn>,
}

impl<Result, Args, Fn: Expr<ConstFn<Result, Args>>> const ConstFnTrait<Result, Args> for AsConstFn<Result, Args, Fn> {
    default fn apply(_: Args) -> Result {
        panic!("AsConstFn was called on a Value<ConstFn<...>> that does not implement ConstFnTrait")
    }
}

impl<Result, Args, Fn: Expr<ConstFn<Result, Args>>> const ConstFnTrait<Result, Args> for AsConstFn<Result, Args, Fn>
    where <<Fn as Expr<ConstFn<Result, Args>>>::Eval as Value<ConstFn<Result, Args>>>::UnconstrainedImpl: ~ const ConstFnTrait<Result, Args> {
    fn apply(args: Args) -> Result {
        return <<<Fn as Expr<ConstFn<Result, Args>>>::Eval as Value<ConstFn<Result, Args>>>::UnconstrainedImpl as ConstFnTrait<Result, Args>>::apply(args);
    }
}

// The contents of this module have to be "pub" because otherwise Rust would complain that
// "can't leak private type". The wrapping non-pub mod ensures that these are never explicitly
// referenced outside.
mod internal {
    pub use crate::*;
}
