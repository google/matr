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

pub struct RemoveFromSet<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> {
    k: PhantomData<K>,
    x: PhantomData<X>,
    s: PhantomData<S>,
}

impl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> Expr<Set<K>> for RemoveFromSet<K, X, S> {
    type Eval = RemoveFromSetValue<K, X, S>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct RemoveFromSetValue<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> SetValue<K> for RemoveFromSetValue<K, X, S> {
        type Impl = RemoveFromSetImpl<K, X, S>;
    }

    pub struct RemoveFromSetImpl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind, X: Expr<K>, S: Expr<Set<K>>> SetTrait<K> for RemoveFromSetImpl<K, X, S> {
        type GetList = RemoveFromList<K, X, <AsSet<K, S> as SetTrait<K>>::GetList>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::r#type::assertions::assert_type_eq;
    use crate::set::assertions::assert_type_set_eq;
    use crate::type_list::type_list;

    #[test]
    fn remove_from_empty_set() {
        assert_type_eq!(ToTypeNestedTuple<RemoveFromList<Type, WrapType<f32>, EmptyList<Type>>>, WrapType<()>);
    }

    #[test]
    fn remove_from_set_found() {
        type S = ListToSet<Type, type_list![usize, f32, u64]>;
        type S2 = ListToSet<Type, type_list![usize, u64]>;
        assert_type_set_eq!(RemoveFromSet<Type, WrapType<f32>, S>, S2);
    }

    #[test]
    fn remove_from_set_not_found() {
        type S = ListToSet<Type, type_list![usize, f32, u64]>;
        assert_type_set_eq!(RemoveFromSet<Type, WrapType<bool>, S>, S);
    }
}
