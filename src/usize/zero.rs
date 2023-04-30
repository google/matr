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
    pub struct Zero: Expr<USize> {
        type Eval = ZeroValue;
    }
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct ZeroValue: USizeValue {
            type Impl = ZeroImpl;
        }

        pub struct ZeroImpl: USizeTrait {
            type Visit<ResultK: Kind, V: USizeVisitor<ResultK>> = V::VisitZero;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::marker::PhantomData;
    use crate::*;

    #[test]
    fn zero() {
        const N: usize = to_usize::<Zero>();
        assert_eq!(N, 0);
    }

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
            VisitUSize<Type, Zero, MyVisitor>,
            WrapType<CalledVisitZero>);
    }
}

