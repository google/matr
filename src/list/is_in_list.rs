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
    pub type IsInList<
        K: EqualityComparableKind,
        X: Expr<K>,
        L: Expr<List<K>>
    >: Expr<Bool> =
        VisitList<K, Bool, L, IsInListVisitor<K, X>>;
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct IsInListVisitor<
            K: EqualityComparableKind,
            X: Expr<K>
        >: ListVisitor<K, Bool> {
            type VisitEmptyList = False;
            type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = Or<Equals<K, Elem, X>, IsInList<K, X, Tail>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn is_in_empty_list() {
        meta_assert_eq!(Bool, IsInList<Type, WrapType<i32>, EmptyList<Type>>, False);
    }

    #[test]
    fn is_in_list_found() {
        type L = type_list![i32, f64, usize];
        meta_assert_eq!(Bool, IsInList<Type, WrapType<f64>, L>, True);
    }

    #[test]
    fn is_in_list_not_found() {
        type L = type_list![i32, f64, usize];
        meta_assert_eq!(Bool, IsInList<Type, WrapType<u32>, L>, False);
    }
}
