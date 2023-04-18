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

pub fn call_runtime_fn<Result, Args, F: Expr<RuntimeFn<Result, Args>>>(args: Args) -> Result {
    return <AsRuntimeFn<Result, Args, F> as RuntimeFnTrait<Result, Args>>::apply(args);
}

mod internal {
    pub use super::super::internal::*;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::internal::*;
    
    meta!{
        struct Add42Impl: RuntimeFnTrait<i32, i32> {
            fn apply(n: i32) -> i32 {
                return n + 42;
            }
        }
        
        struct Add42Value: RuntimeFnValue<i32, i32> {
            type Impl = Add42Impl;
        }
        
        struct Add42: Expr<RuntimeFn<i32, i32>> {
            type Eval = Add42Value;
        }
    }
    
    #[test]
    fn call() {
        assert_eq!(call_runtime_fn::<i32, i32, Add42>(1000), 1042);
    }
}
