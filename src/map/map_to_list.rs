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
    pub type MapToList<
        K: KindWithDefault + EqualityComparableKind,
        V: Kind,
        M: Expr<Map<K, V>>
    >: Expr<List<Pair<K, V>>> =
        <AsMap<K, V, M> as MapTrait<K, V>>::GetList;
}

mod internal {
    pub use super::super::internal::*;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn map_to_list() {
        meta_assert_eq!(List<Pair<Type, Type>>,
            MapToList<Type, Type, type_map!{}>,
            meta_list!(<Pair<Type, Type>>[]));
        meta_assert_eq!(List<Pair<Type, Type>>,
            MapToList<Type, Type, type_map!{i32: u32, i64: u64}>,
            meta_list!(<Pair<Type, Type>>[ConsPair<Type, Type, WrapType<i32>, WrapType<u32>>, ConsPair<Type, Type, WrapType<i64>, WrapType<u64>>]));
    }
}
