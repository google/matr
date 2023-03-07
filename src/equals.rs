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
use crate::*;

pub trait EqualityComparableKind: Kind {
    type Eq<X: Expr<Self>, Y: Expr<Self>>: Expr<Bool>;
}

pub struct Equals<K: EqualityComparableKind, X: Expr<K>, Y: Expr<K>> {
    k: PhantomData<K>,
    x: PhantomData<X>,
    y: PhantomData<Y>,
}

impl<K: EqualityComparableKind, X: Expr<K>, Y: Expr<K>> Expr<Bool> for Equals<K, X, Y> {
    type Eval = <<K as EqualityComparableKind>::Eq<X, Y> as Expr<Bool>>::Eval;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::bool::assertions::*;

    #[test]
    fn equals_types() {
        assert_true!(Equals<Type, WrapType<i32>, WrapType<i32>>);
        assert_false!(Equals<Type, WrapType<i32>, WrapType<i64>>);
    }

    #[test]
    fn equals_bools() {
        assert_true!(Equals<Bool, True, True>);
        assert_true!(Equals<Bool, False, False>);
        assert_false!(Equals<Bool, True, False>);
        assert_false!(Equals<Bool, False, True>);
    }
}
