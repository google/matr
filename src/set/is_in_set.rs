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
use crate::bool::*;

meta!{
    pub type IsInSet<
        K: EqualityComparableKind, 
        X: Expr<K>,
        S: Expr<Set<K>>
    >: Expr<Bool> =
        VisitSet<K, Bool, S, IsInSetVisitor<K, X>>;
}

mod internal {
    pub use super::super::internal::*;
    use crate::bool::*;

    meta!{
        pub struct IsInSetVisitor<
            K: EqualityComparableKind, 
            X: Expr<K>
        >: SetVisitor<K, Bool> {
            type VisitEmptySet = False;
            type VisitCons<Elem: Expr<K>, Tail: Expr<Set<K>>> = Or<Equals<K, Elem, X>, IsInSet<K, X, Tail>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::usize::*;
    use crate::bool::*;
    use crate::r#type::*;

    type N0 = Zero;
    type N1 = Increment<N0>;
    type N2 = Increment<N1>;
    type N3 = Increment<N2>;
    type N4 = Increment<N3>;
    type N5 = Increment<N4>;
    type N6 = Increment<N5>;

    #[test]
    fn is_in_empty_set() {
        meta_assert_eq!(Bool, IsInSet<Type, WrapType<i32>, EmptySet<Type>>, False);
    }

    #[test]
    fn is_in_set_found() {
        type S = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, AddToSet<Type, WrapType<usize>, EmptySet<Type>>>>;
        meta_assert_eq!(Bool, IsInSet<Type, WrapType<f64>, S>, True);
    }

    #[test]
    fn is_in_set_not_found() {
        type S = AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<f64>, AddToSet<Type, WrapType<usize>, EmptySet<Type>>>>;
        meta_assert_eq!(Bool, IsInSet<Type, WrapType<u32>, S>, False);
    }
}
