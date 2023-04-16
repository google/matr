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

pub struct IsInSet<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> {
    k: PhantomData<K>,
    x: PhantomData<X>,
    s: PhantomData<S>,
}

impl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> Expr<Bool> for IsInSet<K, X, S> {
    type Eval = IsInSetValue<K, X, S>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct IsInSetValue<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> BoolValue for IsInSetValue<K, X, S> {
        type Impl = AsBool<IsInList<K, X, <AsSet<K, S> as SetTrait<K>>::GetList>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    type N0 = Zero;
    type N1 = Increment<N0>;
    type N2 = Increment<N1>;
    type N3 = Increment<N2>;
    type N4 = Increment<N3>;
    type N5 = Increment<N4>;
    type N6 = Increment<N5>;

    #[test]
    fn is_in_empty_set() {
        assert_false!(IsInSet<Type, WrapType<i32>, EmptySet<Type>>);
    }

    #[test]
    fn is_in_set_found() {
        type S = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, AddToSet<Type, WrapType<usize>, EmptySet<Type>>>>;
        assert_true!(IsInSet<Type, WrapType<f64>, S>);
    }

    #[test]
    fn is_in_set_not_found() {
        type S = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, AddToSet<Type, WrapType<usize>, EmptySet<Type>>>>;
        assert_false!(IsInSet<Type, WrapType<u32>, S>);
    }
}
