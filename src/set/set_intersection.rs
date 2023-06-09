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
    pub type SetIntersection<
        K: EqualityComparableKind,
        S1: Expr<Set<K>>,
        S2: Expr<Set<K>>
    >: Expr<Set<K>> =
        VisitSet<K, Set<K>, S1, SetIntersectionVisitor<K, S2, EmptySet<K>>>;
}

mod internal {
    pub use super::super::internal::*;
    use crate::bool::*;

    meta!{
        pub struct SetIntersectionVisitor<
            K: EqualityComparableKind,
            S: Expr<Set<K>>, 
            ResultS: Expr<Set<K>>
        >: SetVisitor<K, Set<K>> {
            type VisitEmptySet = ResultS;
            type VisitCons<Elem: Expr<K>, Tail: Expr<Set<K>>> =
                VisitSet<K, Set<K>, Tail, SetIntersectionVisitor<K, S,
                    If<Set<K>,
                        IsInSet<K, Elem, S>,
                        AddToSet<K, Elem, ResultS>,
                        ResultS
                    >>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::r#type::*;

    #[test]
    fn is_empty_set_and_empty_set() {
        meta_assert_eq!(Set<Type>, SetIntersection<Type, EmptySet<Type>, EmptySet<Type>>, EmptySet<Type>);
    }

    #[test]
    fn is_non_empty_set_and_empty_set() {
        type S = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        meta_assert_eq!(Set<Type>, SetIntersection<Type, S, EmptySet<Type>>, EmptySet<Type>);
    }

    #[test]
    fn is_empty_set_and_non_empty_set() {
        type S = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        meta_assert_eq!(Set<Type>, SetIntersection<Type, EmptySet<Type>, S>, EmptySet<Type>);
    }

    #[test]
    fn intersection_subset() {
        type S1 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>>;
        meta_assert_eq!(Set<Type>, SetIntersection<Type, S1, S2>, S1);
    }

    #[test]
    fn intersection_superset() {
        type S1 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>>;
        type S2 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        meta_assert_eq!(Set<Type>, SetIntersection<Type, S1, S2>, S2);
    }

    #[test]
    fn intersection_general() {
        type S1 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        type S3 = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        meta_assert_eq!(Set<Type>, SetIntersection<Type, S1, S2>, S3);
    }
}
