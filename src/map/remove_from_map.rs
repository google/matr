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

pub struct RemoveFromMap<K: EqualityComparableKind, V: Kind, X: Expr<K>, S: Expr<Map<K, V>>> {
    k: PhantomData<K>,
    v: PhantomData<V>,
    x: PhantomData<X>,
    s: PhantomData<S>,
}

impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>, S: Expr<Map<K, V>>> Expr<Map<K, V>> for RemoveFromMap<K, V, X, S> {
    type Eval = RemoveFromMapValue<K, V, X, S>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct RemoveFromMapValue<K: EqualityComparableKind, V: Kind, X: Expr<K>, S: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        x: PhantomData<X>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>, S: Expr<Map<K, V>>> MapValue<K, V> for RemoveFromMapValue<K, V, X, S> {
        type Impl = RemoveFromMapImpl<K, V, X, S>;
    }

    pub struct RemoveFromMapImpl<K: EqualityComparableKind, V: Kind, X: Expr<K>, S: Expr<Map<K, V>>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        x: PhantomData<X>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>, S: Expr<Map<K, V>>> MapTrait<K, V> for RemoveFromMapImpl<K, V, X, S> {
        type GetList = <AsList<Pair<K, V>, <AsMap<K, V, S> as MapTrait<K, V>>::GetList> as ListTrait<Pair<K, V>>>::Visit<List<Pair<K, V>>, RemoveFromMapVisitor<K, V, X>>;
    }

    pub struct RemoveFromMapVisitor<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>> {
        k: PhantomData<K>,
        v: PhantomData<V>,
        x: PhantomData<X>,
    }

    impl<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, X: Expr<K>> ListVisitor<Pair<K, V>, List<Pair<K, V>>> for RemoveFromMapVisitor<K, V, X> {
        type VisitEmptyList = EmptyList<Pair<K, V>>;
        type VisitCons<Elem: Expr<Pair<K, V>>, Tail: Expr<List<Pair<K, V>>>> = AsList<Pair<K, V>, If<
            List<Pair<K, V>>,
            Equals<K, GetFirst<K, V, Elem>, X>,
            <AsList<Pair<K, V>, Tail> as ListTrait<Pair<K, V>>>::Visit<List<Pair<K, V>>, RemoveFromMapVisitor<K, V, X>>,
            Cons<Pair<K, V>, Elem, <AsList<Pair<K, V>, Tail> as ListTrait<Pair<K, V>>>::Visit<List<Pair<K, V>>, RemoveFromMapVisitor<K, V, X>>>
        >>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::r#type::assertions::assert_type_eq;
    use crate::map::assertions::assert_type_map_eq;
    use crate::type_list::type_list;

    #[test]
    fn remove_from_empty_map() {
        assert_type_eq!(ToTypeNestedTuple<RemoveFromList<Type, WrapType<f32>, EmptyList<Type>>>, WrapType<()>);
    }

    #[test]
    fn remove_from_map_found() {
        type S = ListToMap<Type, Type, type_list![usize, f32, u64]>;
        type S2 = ListToMap<Type, Type, type_list![usize, u64]>;
        assert_type_map_eq!(RemoveFromMap<Type, Type, WrapType<f32>, S>, S2);
    }

    #[test]
    fn remove_from_map_not_found() {
        type S = ListToMap<Type, Type, type_list![usize, f32, u64]>;
        assert_type_map_eq!(RemoveFromMap<Type, Type, WrapType<bool>, S>, S);
    }
}
