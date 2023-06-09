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
    pub type RemoveFromSet<
        K: EqualityComparableKind,
        X: Expr<K>,
        S: Expr<Set<K>>
    >: Expr<Set<K>> =
        ListToSetUnchecked<K, RemoveFromList<K, X, SetToList<K, S>>>;
}

mod internal {
    pub use super::super::internal::*;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::r#type::*;

    #[test]
    fn remove_from_empty_set() {
        meta_assert_eq!(
            Set<Type>,
            RemoveFromSet<Type, WrapType<f32>, EmptySet<Type>>,
            EmptySet<Type>);
    }

    #[test]
    fn remove_from_set_found() {
        type S = ListToSet<Type, type_list![usize, f32, u64]>;
        type S2 = ListToSet<Type, type_list![usize, u64]>;
        meta_assert_eq!(
            Set<Type>,
            RemoveFromSet<Type, WrapType<f32>, S>,
            S2);
    }

    #[test]
    fn remove_from_set_not_found() {
        type S = ListToSet<Type, type_list![usize, f32, u64]>;
        meta_assert_eq!(
            Set<Type>,
            RemoveFromSet<Type, WrapType<bool>, S>,
            S);
    }
}
