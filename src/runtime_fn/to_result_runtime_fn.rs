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
use crate::result::*;

meta!{
    pub type ToResultRuntimeFn<
        Out,
        Args,
        E: Expr<Result<RuntimeFn<Out, Args>>>
    >: Expr<RuntimeFn<std::result::Result<Out, &'static str>, Args>> =
        ResultOrValue<RuntimeFn<std::result::Result<Out, &'static str>, Args>,
            AndThen<RuntimeFn<Out, Args>, RuntimeFn<std::result::Result<Out, &'static str>, Args>,
                E,
                ToResultRuntimeFnAdapter<Out, Args>>,
            WrapRuntimeFn<std::result::Result<Out, &'static str>, Args, ToResultRuntimeFnErrorImpl<Out, Args>>>;
}

mod internal {
    pub use super::super::internal::*;
    use crate::result::*;

    meta!{
        pub struct ToResultRuntimeFnAdapter<
            Out,
            Args
        >: Functor1<RuntimeFn<Out, Args>, Result<RuntimeFn<std::result::Result<Out, &'static str>, Args>>> {
            type Apply<X: Expr<RuntimeFn<Out, Args>>> = Ok<RuntimeFn<std::result::Result<Out, &'static str>, Args>,
                WrapRuntimeFn<std::result::Result<Out, &'static str>, Args, ToResultRuntimeFnAdapterImpl<Out, Args, X>>>;
        }

        pub struct ToResultRuntimeFnAdapterImpl<
            Out,
            Args,
            X: Expr<RuntimeFn<Out, Args>>
        >: RuntimeFnTrait<std::result::Result<Out, &'static str>, Args> {
            fn apply(args: Args) -> std::result::Result<Out, &'static str> {
                return Ok(call_runtime_fn::<Out, Args, X>(args));
            }
        }

        pub struct ToResultRuntimeFnErrorImpl<Out, Args>: RuntimeFnTrait<std::result::Result<Out, &'static str>, Args> {
            fn apply(_: Args) -> std::result::Result<Out, &'static str> {
                return Err("error Result in ToResultRuntimeFn")
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
        pub struct ToUnsigned32RuntimeFn: RuntimeFnTrait<u32, i32> {
            fn apply(n: i32) -> u32 {
                return n as u32
            }
        }
    }

    #[test]
    fn ok() {
        assert_eq!(
            call_runtime_fn::<std::result::Result<u32, &'static str>, i32, ToResultRuntimeFn<u32, i32, Ok<RuntimeFn<u32, i32>, WrapRuntimeFn<u32, i32, ToUnsigned32RuntimeFn>>>>(42 as i32),
            std::result::Result::Ok(42 as u32));
    }

    struct MyError {}

    #[test]
    fn error() {
        assert_eq!(
            call_runtime_fn::<std::result::Result<u32, &'static str>, i32, ToResultRuntimeFn<u32, i32, Err<RuntimeFn<u32, i32>, MyError>>>(42 as i32),
            std::result::Result::Err("error Result in ToResultRuntimeFn"));
    }
}
