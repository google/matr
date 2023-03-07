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

pub const fn call_const_fn<Out, Args, F: Expr<ConstFn<Out, Args>>>(args: Args) -> Out {
    return <AsConstFn<Out, Args, F> as ConstFnTrait<Out, Args>>::apply(args);
}

mod internal {
    pub use super::super::internal::*;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::internal::*;

    struct Add42Impl {}

    impl const ConstFnTrait<i32, i32> for Add42Impl {
        fn apply(n: i32) -> i32 {
            return n + 42;
        }
    }

    struct Add42Value {}

    impl ConstFnValue<i32, i32> for Add42Value {
        type Impl = Add42Impl;
    }

    struct Add42 {}

    impl Expr<ConstFn<i32, i32>> for Add42 {
        type Eval = Add42Value;
    }

    #[test]
    fn call() {
        const N: i32 = call_const_fn::<i32, i32, Add42>(1000);
        assert_eq!(N, 1042);
    }
}
