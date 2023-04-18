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

use super::internal::*;

meta!{
    pub type Or<
        X: Expr<Bool>,
        Y: Expr<Bool>
    >: Expr<Bool> =
        If<Bool, X, True, Y>;

    pub type OrResults<
        X: Expr<Result<Bool>>,
        Y: Expr<Result<Bool>>
    >: Expr<Result<Bool>> =
        IfResult<Bool, X, Ok<Bool, True>, Y>;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn or() {
        assert_true!(Or<True, True>);
        assert_true!(Or<True, False>);
        assert_true!(Or<False, True>);
        assert_false!(Or<False, False>);
    }

    #[test]
    fn or_does_not_eval_other_branch() {
        assert_true!(Or<True, LongRecursion<Zero>>);

        // This causes a build error: "overflow evaluating the requirement" (as desired).
        // assert_true!(Or<False, LongRecursion<Zero>>);
    }
}
