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

// Concatenates L and Tail.
pub struct ListSize<K: Kind, L: Expr<List<K>>> {
    k: PhantomData<K>,
    l: PhantomData<L>,
}

impl<K: Kind, L: Expr<List<K>>> Expr<USize> for ListSize<K, L> {
    type Eval = ListSizeValue<K, L>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ListSizeValue<K: Kind, L: Expr<List<K>>> {
        k: PhantomData<K>,
        l: PhantomData<L>,
    }

    impl<K: Kind, L: Expr<List<K>>> USizeValue for ListSizeValue<K, L> {
        type Impl = AsUSize<<AsList<K, L> as ListTrait<K>>::Visit<USize, ListSizeVisitor<K>>>;
    }

    pub struct ListSizeVisitor<K: Kind> {
        k: PhantomData<K>,
    }

    impl<K: Kind> ListVisitor<K, USize> for ListSizeVisitor<K> {
        type VisitEmptyList = Zero;
        type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = Increment<ListSize<K, Tail>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::type_list::type_list;

    #[test]
    fn empty_list() {
        const N: usize = to_usize::<ListSize<Type, EmptyList<Type>>>();
        assert_eq!(N, 0);
    }

    #[test]
    fn list_size() {
        type L = type_list![u32, f64, u32];
        const N: usize = to_usize::<ListSize<Type, L>>();
        assert_eq!(N, 3);
    }
}
