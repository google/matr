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
    pub type MapCommonKeysWithDifferentValue<
        K: EqualityComparableKind + KindWithDefault,
        V: EqualityComparableKind + KindWithDefault, 
        M1: Expr<Map<K, V>>, 
        M2: Expr<Map<K, V>>
    >: Expr<Map<K, Pair<V, V>>> =
        VisitMap<K, V, Map<K, Pair<V, V>>, M1, MapCommonKeysWithDifferentValueVisitor<K, V, M2, EmptyMap<K, Pair<V, V>>>>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct MapCommonKeysWithDifferentValueVisitor<
            K: KindWithDefault + EqualityComparableKind,
            V: KindWithDefault + EqualityComparableKind, 
            M: Expr<Map<K, V>>, 
            ResultM: Expr<Map<K, Pair<V, V>>>
        >: MapVisitor<K, V, Map<K, Pair<V, V>>> {
            type VisitEmptyMap = EmptyMap<K, Pair<V, V>>;
            type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>> =
                VisitMap<K, V, Map<K, Pair<V, V>>, Tail, MapCommonKeysWithDifferentValueVisitor<K, V, M,
                    VisitOption<
                        V,
                        Map<K, Pair<V, V>>,
                        MapGet<K, V, Key, M>,
                        MapCommonKeysWithDifferentValueMapGetVisitor<K, V, M, ResultM, Key, Value>
                    >
            >>;
        }

        pub struct MapCommonKeysWithDifferentValueMapGetVisitor<
            K: KindWithDefault + EqualityComparableKind, 
            V: KindWithDefault + EqualityComparableKind, 
            M: Expr<Map<K, V>>, 
            ResultM: Expr<Map<K, Pair<V, V>>>, 
            Key: Expr<K>, 
            Value: Expr<V>
        >: OptionVisitor<V, Map<K, Pair<V, V>>> {
            type VisitNone = ResultM;
            type VisitSome<ValueInMap: Expr<V>> =
                If<Map<K, Pair<V, V>>,
                    Not<Equals<V,
                        ValueInMap,
                        Value>>,
                    Put<K, Pair<V, V>, Key, ConsPair<V, V, ValueInMap, Value>, ResultM>,
                    ResultM>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn empty_map_and_empty_map() {
        type ResultM = MapCommonKeysWithDifferentValue<Type, Type, EmptyMap<Type, Type>, EmptyMap<Type, Type>>;
        assert_type_eq!(
            ToTypeTripleNestedTuple<MapToList<Type, Pair<Type, Type>, ResultM>>,
            WrapType<()>);
    }

    #[test]
    fn non_empty_map_and_empty_map() {
        type M = type_map!{
            i32: u32
        };
        type ResultM = MapCommonKeysWithDifferentValue<Type, Type, M, EmptyMap<Type, Type>>;
        assert_type_eq!(
            ToTypeTripleNestedTuple<MapToList<Type, Pair<Type, Type>, ResultM>>,
            WrapType<()>);
    }

    #[test]
    fn is_empty_map_and_non_empty_map() {
        type M = type_map!{
            i32: u32
        };
        type ResultM = MapCommonKeysWithDifferentValue<Type, Type, EmptyMap<Type, Type>, M>;
        assert_type_eq!(
            ToTypeTripleNestedTuple<MapToList<Type, Pair<Type, Type>, ResultM>>,
            WrapType<()>);
    }

    // #[test]
    // fn keys_subset() {
    //     type M1 = type_map! {
    //         i32: (i32,),
    //         f64: (u64,),
    //     };
    //     type M2 = type_map! {
    //         u32: (u32,),
    //         i32: (i32,),
    //         u64: (u64,),
    //         f64: (f64,),
    //     };
    //     type ResultM = MapCommonKeysWithDifferentValue<Type, Type, M1, M2>;
    //     assert_type_eq!(GetFirst<Type, Type, MapGet<Type, Pair<Type, Type>, WrapType<f64>, ResultM>>, WrapType<(u64,)>);
    //     assert_type_eq!(GetSecond<Type, Type, MapGet<Type, Pair<Type, Type>, WrapType<f64>, ResultM>>, WrapType<(f64,)>);
    //     assert_type_eq!(
    //         ToTypeTripleNestedTuple<MapToList<Type, Pair<Type, Type>, ResultM>>,
    //         WrapType<((f64, ((u64,), (f64,))), ())>);
    // }

    // #[test]
    // fn superset() {
    //     type M1 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptyMap<Type, Type>>>>>;
    //     type M2 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptyMap<Type, Type>>>;
    //     assert_type_map_eq!(MapCommonKeysWithDifferentValue<Type, Type, M1, M2>, M1);
    // }
    //
    // #[test]
    // fn general() {
    //     type M1 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, EmptyMap<Type, Type>>>;
    //     type M2 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptyMap<Type, Type>>>;
    //     type S3 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, AddToSet<Type, WrapType<f64>, EmptyMap<Type, Type>>>>;
    //     assert_type_map_eq!(MapCommonKeysWithDifferentValue<Type, Type, M1, M2>, S3);
    // }
}
