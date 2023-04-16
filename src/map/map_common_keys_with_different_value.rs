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

pub struct MapCommonKeysWithDifferentValue<K: EqualityComparableKind, V: EqualityComparableKind, M1: Expr<Map<K, V>>, M2: Expr<Map<K, V>>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    m1: PhantomData<M1>,
    m2: PhantomData<M2>,
}

impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault + EqualityComparableKind, M1: Expr<Map<K, V>>, M2: Expr<Map<K, V>>> Expr<Map<K, Pair<V, V>>> for MapCommonKeysWithDifferentValue<K, V, M1, M2> {
    type Eval = MapCommonKeysWithDifferentValueValue<K, V, M1, M2>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct MapCommonKeysWithDifferentValueValue<K: EqualityComparableKind, V: EqualityComparableKind, M1: Expr<Map<K, V>>, M2: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        m1: PhantomData<M1>,
        m2: PhantomData<M2>,
    }

    impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault + EqualityComparableKind, M1: Expr<Map<K, V>>, M2: Expr<Map<K, V>>> MapValue<K, Pair<V, V>> for MapCommonKeysWithDifferentValueValue<K, V, M1, M2> {
        type Impl = AsMap<K, Pair<V, V>, <AsList<Pair<K, V>, <AsMap<K, V, M1> as MapTrait<K, V>>::GetList> as ListTrait<Pair<K, V>>>::Visit<Map<K, Pair<V, V>>, MapCommonKeysWithDifferentValueVisitor<K, V, M2, EmptyMap<K, Pair<V, V>>>>>;
    }

    pub struct MapCommonKeysWithDifferentValueVisitor<K: EqualityComparableKind, V: EqualityComparableKind, M: Expr<Map<K, V>>, ResultM: Expr<Map<K, Pair<V, V>>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        m: PhantomData<M>,
        result_m: PhantomData<ResultM>,
    }

    impl<K: KindWithDefault + EqualityComparableKind, V: KindWithDefault + EqualityComparableKind, M: Expr<Map<K, V>>, ResultM: Expr<Map<K, Pair<V, V>>>> ListVisitor<Pair<K, V>, Map<K, Pair<V, V>>> for MapCommonKeysWithDifferentValueVisitor<K, V, M, ResultM> {
        type VisitEmptyList = EmptyMap<K, Pair<V, V>>;
        type VisitCons<Elem: Expr<Pair<K, V>>, Tail: Expr<List<Pair<K, V>>>> =
            <AsList<Pair<K, V>, Tail> as ListTrait<Pair<K, V>>>::Visit<Map<K, Pair<V, V>>, MapCommonKeysWithDifferentValueVisitor<K, V, M,
                // If<Map<K, Pair<V, V>>,
                //     And<
                //         IsInMap<K, V, GetFirst<K, V, Elem>, M>,
                //         Not<Equals<V,
                //             MapGet<K, V, GetFirst<K, V, Elem>, M>,
                //             GetSecond<K, V, Elem>>>>,
                    Put<K, Pair<V, V>, GetFirst<K, V, Elem>, ConsPair<V, V, MapGet<K, V, GetFirst<K, V, Elem>, M>, GetSecond<K, V, Elem>>, ResultM>,
                //     ResultM
                // >
        >>;
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

    #[test]
    fn keys_subset() {
        type M1 = type_map! {
            i32: (i32,),
            f64: (u64,),
        };
        type M2 = type_map! {
            u32: (u32,),
            i32: (i32,),
            u64: (u64,),
            f64: (f64,),
        };
        type ResultM = MapCommonKeysWithDifferentValue<Type, Type, M1, M2>;
        assert_type_eq!(GetFirst<Type, Type, MapGet<Type, Pair<Type, Type>, WrapType<f64>, ResultM>>, WrapType<(u64,)>);
        assert_type_eq!(GetSecond<Type, Type, MapGet<Type, Pair<Type, Type>, WrapType<f64>, ResultM>>, WrapType<(f64,)>);
        assert_type_eq!(
            ToTypeTripleNestedTuple<MapToList<Type, Pair<Type, Type>, ResultM>>,
            WrapType<((f64, ((u64,), (f64,))), ())>);
    }

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
