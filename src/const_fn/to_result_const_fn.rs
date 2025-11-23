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
use crate::result::*;

meta!{
    pub type ToResultConstFn<
        Out,
        Args: [const] Destruct,
        E: Expr<Result<ConstFn<Out, Args>>>
    >: const Expr<ConstFn<std::result::Result<Out, &'static str>, Args>> =
        ResultOrValue<
            ConstFn<std::result::Result<Out, &'static str>, Args>,
            AndThen<
                ConstFn<Out, Args>,
                ConstFn<std::result::Result<Out, &'static str>, Args>,
                E,
                ToResultConstFnAdapter<Out, Args>>,
            WrapConstFn<std::result::Result<Out, &'static str>, Args, ToResultConstFnErrorImpl<Out, Args>>>;
}

mod internal {
    use std::mem;
use std::marker::Destruct;
    pub use super::super::internal::*;
    use crate::result::*;

    meta!{
        pub struct ToResultConstFnAdapter<Out, Args>: Functor1<ConstFn<Out, Args>, Result<ConstFn<std::result::Result<Out, &'static str>, Args>>> {
            type Apply<X: Expr<ConstFn<Out, Args>>> = Ok<ConstFn<std::result::Result<Out, &'static str>, Args>, WrapConstFn<std::result::Result<Out, &'static str>, Args, ToResultConstFnAdapterImpl<Out, Args, X>>>;
        }

        pub struct ToResultConstFnAdapterImpl<Out, Args, X: Expr<ConstFn<Out, Args>>>: const ConstFnTrait<std::result::Result<Out, &'static str>, Args> {
            fn apply(args: Args) -> std::result::Result<Out, &'static str> {
                return Ok(call_const_fn::<Out, Args, X>(args));
            }
        }

        pub struct ToResultConstFnErrorImpl<Out, Args: [const] Destruct>: const ConstFnTrait<std::result::Result<Out, &'static str>, Args> {
            fn apply(args: Args) -> std::result::Result<Out, &'static str> {
                mem::forget(args);
                return Err("error Result in ToResultConstFn")
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::result::*;

    meta!{
        pub struct ToUnsigned32ConstFn: ConstFnTrait<u32, i32> {
            fn apply(n: i32) -> u32 {
                return n as u32
            }
        }
    }

    #[test]
    fn ok() {
        assert_eq!(
            call_const_fn::<std::result::Result<u32, &'static str>, i32, ToResultConstFn<u32, i32, Ok<ConstFn<u32, i32>, WrapConstFn<u32, i32, ToUnsigned32ConstFn>>>>(42 as i32),
            std::result::Result::Ok(42 as u32));
    }

    struct MyError {}

    #[test]
    fn error() {
        assert_eq!(
            call_const_fn::<std::result::Result<u32, &'static str>, i32, ToResultConstFn<u32, i32, Err<ConstFn<u32, i32>, MyError>>>(42 as i32),
            std::result::Result::Err("error Result in ToResultConstFn"));
    }
}
