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

pub struct True {}

impl Expr<Bool> for True {
    type Eval = TrueValue;
}

pub struct TrueValue {}
impl BoolValue for TrueValue {
    type Impl = TrueImpl;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use super::super::internal::*;

    pub struct TrueImpl {}

    impl BoolTrait for TrueImpl {
        type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>> = IfTrue;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::internal::*;
    use crate::bool::assertions::*;

    #[test]
    fn true_literal() {
        assert_true!(True);
    }

    #[test]
    fn true_does_not_eval_other_branch() {
        assert_true!(If<Bool, True, True, LongRecursion<Zero>>);

        // This causes a build error: "overflow evaluating the requirement" (as desired).
        // assert_true!(If<Bool, True, LongRecursion<Zero>, True>);
    }
}