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

// Returns the value associated with the key in the map, or the default value for the value kind otherwise.
meta!{
    pub type MapGet<
        K: EqualityComparableKind + KindWithDefault,
        V: KindWithDefault, 
        X: Expr<K>,
        M: Expr<Map<K, V>>
    >: Expr<Option<V>> =
        VisitMap<K, V, Option<V>, M, MapGetVisitor<K, V, X>>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct MapGetVisitor<
            K: EqualityComparableKind + KindWithDefault,
            V: KindWithDefault, 
            X: Expr<K>
        >: MapVisitor<K, V, Option<V>> {
            type VisitEmptyMap = None<V>;
            type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>> = If<Option<V>, Equals<K, Key, X>,
                Some<V, Value>,
                VisitMap<K, V, Option<V>, Tail, MapGetVisitor<K, V, X>>
            >;
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
        meta_assert_eq!(
            Option<Type>,
            MapGet<Type, Type, WrapType<i32>, EmptyMap<Type, Type>>,
            None<Type>);
    }

    #[test]
    fn is_in_map_found() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<f64>, WrapType<u64>, Put<Type, Type, WrapType<usize>, WrapType<usize>, EmptyMap<Type, Type>>>>;
        meta_assert_eq!(
            Option<Type>,
            MapGet<Type, Type, WrapType<f64>, M>,
            Some<Type, WrapType<u64>>);
    }

    #[test]
    fn is_in_map_not_found() {
        type M = Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<f64>, WrapType<u64>, Put<Type, Type, WrapType<usize>, WrapType<usize>, EmptyMap<Type, Type>>>>;
        meta_assert_eq!(
            Option<Type>,
            MapGet<Type, Type, WrapType<u32>, M>,
            None<Type>);
    }
}
