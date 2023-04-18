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
    pub type IsInMap<
        K: EqualityComparableKind + KindWithDefault,
        V: KindWithDefault,
        X: Expr<K>,
        M: Expr<Map<K, V>>
    >: Expr<Bool> =
        VisitMap<K, V, Bool, M, IsInMapVisitor<K, V, X>>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct IsInMapVisitor<
            K: EqualityComparableKind + KindWithDefault,
            V: KindWithDefault,
            X: Expr<K>
        >: MapVisitor<K, V, Bool> {
            type VisitEmptyMap = False;
            type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>> = Or<Equals<K, Key, X>, IsInMap<K, V, X, Tail>>;
        }
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
    fn is_in_empty_map() {
        assert_false!(IsInMap<Type, Type, WrapType<i32>, EmptyMap<Type, Type>>);
    }

    #[test]
    fn is_in_map_found() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<f64>, WrapType<u64>, Put<Type, Type, WrapType<usize>, WrapType<usize>, EmptyMap<Type, Type>>>>;
        assert_true!(IsInMap<Type, Type, WrapType<f64>, M>);
    }

    #[test]
    fn is_in_map_not_found() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<f64>, WrapType<u64>, Put<Type, Type, WrapType<usize>, WrapType<usize>, EmptyMap<Type, Type>>>>;
        assert_false!(IsInMap<Type, Type, WrapType<u32>, M>);
    }
}
