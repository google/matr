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
use internal::*;

// An Expr<K> that, when evaluated, causes a compilation error due to exceeding the maximum
// number of allowed recursion steps.
// This isn't *really* infinite (that is impossible due to Rust's type checker) but it recurses
// so many times that any reasonable recursion steps maximum would be exceeded.
pub struct LongRecursion<N: Expr<USize>> {
    n: PhantomData<N>,
}

impl<N: Expr<USize>> Expr<Bool> for LongRecursion<N> {
    type Eval = <IsEven<Sum<N, OneBillion>> as Expr<Bool>>::Eval;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    type Ten = Increment<Increment<Increment<Increment<Increment<Increment<Increment<Increment<Increment<Increment<Zero>>>>>>>>>>;
    type OneThousand = Multiply<Multiply<Ten, Ten>, Ten>;
    pub type OneBillion = Multiply<Multiply<OneThousand, OneThousand>, OneThousand>;

    #[allow(dead_code)]
    pub struct LongRecursionValue<N: Expr<USize>> {
        n: PhantomData<N>,
    }
}

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn long_recursion() {
        // This fails with a recursion overflow (as expected)
        // TODO: use a negative compilation test.
        // assert_false!(LongRecursion<Zero>);
    }
}
