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
    // An Expr<K> that, when evaluated, causes a compilation error due to exceeding the maximum
    // number of allowed recursion steps.
    // This isn't *really* infinite (that is impossible due to Rust's type checker) but it recurses
    // so many times that any reasonable recursion steps maximum would be exceeded.
    pub type LongRecursion<
        N: Expr<USize>
    >: Expr<Bool> =
        IsEven<Sum<N, OneBillion>>;
}

pub type OneBillion = Multiply<Multiply<OneThousand, OneThousand>, OneThousand>;

mod internal {
    pub use crate::*;

    pub type Ten = Increment<Increment<Increment<Increment<Increment<Increment<Increment<Increment<Increment<Increment<Zero>>>>>>>>>>;
    pub type OneThousand = Multiply<Multiply<Ten, Ten>, Ten>;
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
