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
    pub type And<
        X: Expr<Bool>,
        Y: Expr<Bool>
    >: Expr<Bool> =
        If<Bool, X, Y, False>;
}

mod internal {
    pub use super::super::internal::*;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::internal::*;

    #[test]
    fn and() {
        meta_assert_eq!(Bool, And<True, True>, True);
        meta_assert_eq!(Bool, And<True, False>, False);
        meta_assert_eq!(Bool, And<False, True>, False);
        meta_assert_eq!(Bool, And<False, False>, False);
    }

    #[test]
    fn and_does_not_eval_other_branch() {
        meta_assert_eq!(Bool, And<False, LongRecursion<Zero>>, False);

        // This causes a build error: "overflow evaluating the requirement" (as desired).
        // assert_true!(And<True, LongRecursion<Zero>>);
    }
}
