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

pub struct SetIntersection<K: EqualityComparableKind, S1: Expr<Set<K>>, S2: Expr<Set<K>>> {
    k: PhantomData<K>,
    s1: PhantomData<S1>,
    s2: PhantomData<S2>,
}

impl<K: EqualityComparableKind, S1: Expr<Set<K>>, S2: Expr<Set<K>>> Expr<Set<K>> for SetIntersection<K, S1, S2> {
    type Eval = SetIntersectionValue<K, S1, S2>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct SetIntersectionValue<K: EqualityComparableKind, S1: Expr<Set<K>>, S2: Expr<Set<K>>> {
        k: PhantomData<K>,
        s1: PhantomData<S1>,
        s2: PhantomData<S2>,
    }

    impl<K: EqualityComparableKind, S1: Expr<Set<K>>, S2: Expr<Set<K>>> SetValue<K> for SetIntersectionValue<K, S1, S2> {
        type Impl = AsSet<K, <AsList<K, <AsSet<K, S1> as SetTrait<K>>::GetList> as ListTrait<K>>::Visit<Set<K>, SetIntersectionVisitor<K, S2, EmptySet<K>>>>;
    }

    pub struct SetIntersectionVisitor<K: EqualityComparableKind, S: Expr<Set<K>>, ResultS: Expr<Set<K>>> {
        k: PhantomData<K>,
        s: PhantomData<S>,
        result_s: PhantomData<ResultS>,
    }

    impl<K: EqualityComparableKind, S: Expr<Set<K>>, ResultS: Expr<Set<K>>> ListVisitor<K, Set<K>> for SetIntersectionVisitor<K, S, ResultS> {
        type VisitEmptyList = EmptySet<K>;
        type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> =
            <AsList<K, Tail> as ListTrait<K>>::Visit<Set<K>, SetIntersectionVisitor<K, S,
                If<Set<K>,
                    IsInSet<K, Elem, S>,
                    AddToSet<K, Elem, ResultS>,
                    ResultS
                >>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn is_empty_set_and_empty_set() {
        assert_type_set_eq!(SetIntersection<Type, EmptySet<Type>, EmptySet<Type>>, EmptySet<Type>);
    }

    #[test]
    fn is_non_empty_set_and_empty_set() {
        type S = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        assert_type_set_eq!(SetIntersection<Type, S, EmptySet<Type>>, S);
    }

    #[test]
    fn is_empty_set_and_non_empty_set() {
        type S = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        assert_type_set_eq!(SetIntersection<Type, EmptySet<Type>, S>, S);
    }

    #[test]
    fn union_subset() {
        type S1 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>>;
        assert_type_set_eq!(SetIntersection<Type, S1, S2>, S2);
    }

    #[test]
    fn union_superset() {
        type S1 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>>;
        type S2 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        assert_type_set_eq!(SetIntersection<Type, S1, S2>, S1);
    }

    #[test]
    fn union_general() {
        type S1 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        type S3 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>;
        assert_type_set_eq!(SetIntersection<Type, S1, S2>, S3);
    }
}
