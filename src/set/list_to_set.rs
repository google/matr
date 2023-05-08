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
use crate::list::*;

meta!{
    pub type ListToSet<
        K: EqualityComparableKind, 
        L: Expr<List<K>>
    >: Expr<Set<K>> =
        VisitList<K, Set<K>, L, ListToSetVisitor<K>>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct ListToSetVisitor<K: EqualityComparableKind>: ListVisitor<K, Set<K>> {
            type VisitEmptyList = EmptySet<K>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = AddToSet<K, Elem, ListToSet<K, Tail>>;
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
    fn list_to_set() {
        meta_assert_eq!(Set<Type>,
            ListToSet<Type, type_list![]>,
            type_set!{});
        meta_assert_eq!(Set<Type>,
            ListToSet<Type, type_list![i32, u32, i32]>,
            type_set!{i32, u32});
    }
}
