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

meta!{
    pub struct False: Expr<Bool> {
        type Eval = WrapBoolValue<FalseValue>;
    }
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct FalseValue: BoolValue {
            type Impl = FalseImpl;
        }

        pub struct FalseImpl: BoolTrait {
            type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>> = IfFalse;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::usize::*;

    #[test]
    fn false_literal() {
        meta_assert_eq!(Bool, False, False);
    }

    #[test]
    fn false_does_not_eval_other_branch() {
        meta_assert_eq!(Bool, If<Bool, False, LongRecursion<Zero>, True>, True);

        // This causes a build error: "overflow evaluating the requirement" (as desired).
        // assert_true!(If<Bool, False, True, LongRecursion<Zero>>);
    }
}
