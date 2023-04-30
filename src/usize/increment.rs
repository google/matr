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
    pub struct Increment<
        N: Expr<USize>
    >: Expr<USize> {
        type Eval = IncrementValue<N>;
    }
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct IncrementValue<
            N: Expr<USize>
        >: USizeValue {
            type Impl = IncrementImpl<N>;
        }

        pub struct IncrementImpl<
            N: Expr<USize>
        >: USizeTrait {
            type Visit<ResultK: Kind, V: USizeVisitor<ResultK>> = V::VisitIncrement<N>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::marker::PhantomData;
    use crate::*;

    struct CalledVisitZero {}

    struct CalledVisitIncrement<N: Expr<USize>> {
        n: PhantomData<N>,
    }

    meta!{
        struct MyVisitor : USizeVisitor<Type> {
            type VisitZero = WrapType<CalledVisitZero>;
            type VisitIncrement<N: Expr<USize>> = WrapType<CalledVisitIncrement<N>>;
        }
    }

    #[test]
    fn visit() {
        meta_assert_eq!(Type,
            VisitUSize<Type, Increment<Increment<Zero>>, MyVisitor>,
            WrapType<CalledVisitIncrement<Increment<Zero>>>);
    }
}
