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

use crate::*;
use internal::*;

meta!{
    pub struct NoOpConstFn: Expr<ConstFn<(), ()>> {
        type Eval = NoOpConstFnValue;
    }
}

mod internal {
    pub use crate::*;

    meta!{
        pub struct NoOpConstFnValue: ConstFnValue<(), ()> {
            type Impl = NoOpConstFnImplWrapper;
        }
        
        pub struct NoOpConstFnImplWrapper: ConstFnTraitWrapper<(), ()> {
            type Fn = NoOpConstFnImpl;
        }
        
        pub struct NoOpConstFnImpl: const ConstFnTrait<(), ()> {
            fn apply(_: ()) -> () {}
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::internal::*;

    #[test]
    fn no_op_const_fn() {
        call_const_fn::<(), (), NoOpConstFn>(());
    }
}

