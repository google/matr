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

pub struct NoOpConstFn {}

impl Expr<ConstFn<(), ()>> for NoOpConstFn {
    type Eval = NoOpConstFnValue;
}

mod internal {
    pub use crate::*;

    pub struct NoOpConstFnValue {}

    impl ConstFnValue<(), ()> for NoOpConstFnValue {
        type Impl = NoOpConstFnImpl;
    }

    pub struct NoOpConstFnImpl {}

    impl const ConstFnTrait<(), ()> for NoOpConstFnImpl {
        fn apply(_: ()) -> () {}
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

