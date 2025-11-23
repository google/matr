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
use crate::const_fn::*;

meta!{
    pub type NoOpConstFn: Expr<ConstFn<(), ()>> =
        WrapConstFn<(), (), NoOpConstFnImpl>;
}

mod internal {
    pub use crate::*;
    use crate::const_fn::*;

    meta!{
        pub const struct NoOpConstFnImpl: ConstFnTrait<(), ()> {
            fn apply(_: ()) -> () {}
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::super::*;

    #[test]
    fn no_op_const_fn() {
        call_const_fn::<(), (), NoOpConstFn>(());
    }
}
