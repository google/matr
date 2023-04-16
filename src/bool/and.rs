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

use std::marker::PhantomData;
use super::internal::*;

pub struct And<X: Expr<Bool>, Y: Expr<Bool>> {
    x: PhantomData<X>,
    y: PhantomData<Y>,
}

impl<X: Expr<Bool>, Y: Expr<Bool>> Expr<Bool> for And<X, Y> {
    type Eval = <If<Bool, X, Y, False> as Expr<Bool>>::Eval;
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
        assert_true!(And<True, True>);
        assert_false!(And<True, False>);
        assert_false!(And<False, True>);
        assert_false!(And<False, False>);
    }

    #[test]
    fn and_does_not_eval_other_branch() {
        assert_false!(And<False, LongRecursion<Zero>>);

        // This causes a build error: "overflow evaluating the requirement" (as desired).
        // assert_true!(And<True, LongRecursion<Zero>>);
    }
}
