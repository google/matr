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
    pub type IsSubset<
        K: EqualityComparableKind, 
        CandidateSubset: Expr<Set<K>>, 
        CandidateSuperset: Expr<Set<K>>
    >: Expr<Bool> =
        VisitSet<K, Bool, CandidateSubset, IsSubsetVisitor<K, CandidateSuperset>>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct IsSubsetVisitor<
            K: EqualityComparableKind,
            CandidateSuperset: Expr<Set<K>>
        >: SetVisitor<K, Bool> {
            type VisitEmptySet = True;
            type VisitCons<Elem: Expr<K>, Tail: Expr<Set<K>>> = And<IsInSet<K, Elem, CandidateSuperset>, VisitSet<K, Bool, Tail, IsSubsetVisitor<K, CandidateSuperset>>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn is_empty_set_in_empty_set() {
        meta_assert_eq!(Bool, IsSubset<Type, EmptySet<Type>, EmptySet<Type>>, True);
    }

    #[test]
    fn is_non_empty_set_in_empty_set() {
        type S = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        meta_assert_eq!(Bool, IsSubset<Type, S, EmptySet<Type>>, False);
    }

    #[test]
    fn is_empty_set_in_non_empty_set() {
        type S = AddToSet<Type, WrapType<i32>, EmptySet<Type>>;
        meta_assert_eq!(Bool, IsSubset<Type, EmptySet<Type>, S>, True);
    }

    #[test]
    fn subset() {
        type S1 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        type S2 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>>;
        meta_assert_eq!(Bool, IsSubset<Type, S1, S2>, True);
    }

    #[test]
    fn not_subset() {
        type S1 = AddToSet<Type, WrapType<u32>, AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u64>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>>>;
        type S2 = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, EmptySet<Type>>>;
        meta_assert_eq!(Bool, IsSubset<Type, S1, S2>, False);
    }
}
