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

pub struct IsSubset<K: EqualityComparableKind, CandidateSubset: Expr<Set<K>>, CandidateSuperset: Expr<Set<K>>> {
    k: PhantomData<K>,
    candidate_subset: PhantomData<CandidateSubset>,
    candidate_superset: PhantomData<CandidateSuperset>,
}

impl<K: EqualityComparableKind, CandidateSubset: Expr<Set<K>>, CandidateSuperset: Expr<Set<K>>> Expr<Bool> for IsSubset<K, CandidateSubset, CandidateSuperset> {
    type Eval = IsSubsetValue<K, CandidateSubset, CandidateSuperset>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct IsSubsetValue<K: EqualityComparableKind, CandidateSubset: Expr<Set<K>>, CandidateSuperset: Expr<Set<K>>> {
        k: PhantomData<K>,
        candidate_subset: PhantomData<CandidateSubset>,
        candidate_superset: PhantomData<CandidateSuperset>,
    }

    impl<K: EqualityComparableKind, CandidateSubset: Expr<Set<K>>, CandidateSuperset: Expr<Set<K>>> BoolValue for IsSubsetValue<K, CandidateSubset, CandidateSuperset> {
        type Impl = AsBool<<AsList<K, <AsSet<K, CandidateSubset> as SetTrait<K>>::GetList> as ListTrait<K>>::Visit<Bool, IsSubsetVisitor<K, CandidateSuperset>>>;
    }

    pub struct IsSubsetVisitor<K: EqualityComparableKind, CandidateSuperset: Expr<Set<K>>> {
        k: PhantomData<K>,
        candidate_superset: PhantomData<CandidateSuperset>,
    }

    impl<K: EqualityComparableKind, CandidateSuperset: Expr<Set<K>>> ListVisitor<K, Bool> for IsSubsetVisitor<K, CandidateSuperset> {
        type VisitEmptyList = True;
        type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = And<IsInSet<K, Elem, CandidateSuperset>, <AsList<K, Tail> as ListTrait<K>>::Visit<Bool, IsSubsetVisitor<K, CandidateSuperset>>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn is_empty_set_in_empty_set() {
        assert_true!(IsSubset<Type, EmptySet<Type>, EmptySet<Type>>);
    }

    #[test]
    fn is_non_empty_set_in_empty_set() {
        type S = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        assert_false!(IsSubset<Type, S, EmptySet<Type>>);
    }

    #[test]
    fn is_empty_set_in_non_empty_set() {
        type S = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        assert_true!(IsSubset<Type, EmptySet<Type>, S>);
    }

    #[test]
    fn subset() {
        type S1 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>>;
        assert_true!(IsSubset<Type, S1, S2>);
    }

    #[test]
    fn not_subset() {
        type S1 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>>;
        type S2 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        assert_false!(IsSubset<Type, S1, S2>);
    }
}
