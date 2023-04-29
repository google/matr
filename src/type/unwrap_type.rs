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

use internal::*;

// Intended to be used with:
// E: Expr<Type>
// Then UnwrapType<E> is the type that E evaluates to.
pub type UnwrapType<E> = <UnwrapTypeHelper<E> as UnwrapTypeTrait>::Get;

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct UnwrapTypeHelper<E: Expr<Type>> {
        e: PhantomData<E>,
    }

    pub trait UnwrapTypeTrait {
        type Get;
    }

    impl<E: Expr<Type>> UnwrapTypeTrait for UnwrapTypeHelper<E> {
        type Get = <E::Eval as Value<Type>>::UnconstrainedImpl;
    }
}
