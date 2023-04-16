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

pub struct False {}

impl Expr<Bool> for False {
    type Eval = FalseValue;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use super::super::internal::*;

    pub struct FalseValue {}
    impl BoolValue for FalseValue {
        type Impl = FalseImpl;
    }

    pub struct FalseImpl {}

    impl BoolTrait for FalseImpl {
        type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>> = IfFalse;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn false_literal() {
        assert_false!(False);
    }

    #[test]
    fn false_does_not_eval_other_branch() {
        assert_true!(If<Bool, False, LongRecursion<Zero>, True>);

        // This causes a build error: "overflow evaluating the requirement" (as desired).
        // assert_true!(If<Bool, False, True, LongRecursion<Zero>>);
    }
}
