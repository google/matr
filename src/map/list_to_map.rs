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
use crate::pair::*;

meta!{
    pub type ListToMap<
        K: KindWithDefault + EqualityComparableKind,
        V: KindWithDefault,
        L: Expr<List<Pair<K, V>>>
    >: Expr<Map<K, V>> =
        VisitList<Pair<K, V>, Map<K, V>, L, ListToMapVisitor<K, V>>;
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct ListToMapVisitor<
            K: KindWithDefault + EqualityComparableKind, 
            V: KindWithDefault
        >: ListVisitor<Pair<K, V>, Map<K, V>> {
            type VisitEmptyList = EmptyMap<K, V>;
            type VisitCons<Elem: Expr<Pair<K, V>>, Tail: Expr<List<Pair<K, V>>>> = Put<K, V, GetFirst<K, V, Elem>, GetSecond<K, V, Elem>, ListToMap<K, V, Tail>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::r#type::*;
    use crate::pair::*;

    #[test]
    fn list_to_map() {
        meta_assert_eq!(Map<Type, Type>,
            ListToMap<Type, Type, meta_list!(<Pair<Type, Type>>[])>,
            type_map!{});
        meta_assert_eq!(Map<Type, Type>,
            ListToMap<Type, Type, meta_list!(<Pair<Type, Type>>[ConsPair<Type, Type, WrapType<i32>, WrapType<u32>>, ConsPair<Type, Type, WrapType<i64>, WrapType<u64>>])>,
            type_map!{i32: u32, i64: u64});
    }
}
