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

mod empty_set;
mod add_to_set;
mod is_in_set;
mod is_subset;
mod set_union;
mod set_to_list;
mod set_intersection;
mod set_difference;
mod list_to_set;
mod to_hash_set;
mod to_usize_hash_set;
mod remove_from_set;
pub mod meta_set;
mod type_set;

pub use empty_set::*;
pub use add_to_set::*;
pub use is_in_set::*;
pub use is_subset::*;
pub use set_union::*;
pub use set_to_list::*;
pub use set_intersection::*;
pub use set_difference::*;
pub use list_to_set::*;
pub use to_hash_set::*;
pub use to_usize_hash_set::*;
pub use remove_from_set::*;
pub use type_set::*;

use std::marker::PhantomData;
use internal::*;

pub struct Set<K: EqualityComparableKind> {
    k: PhantomData<K>,
}

impl<K: EqualityComparableKind> Kind for Set<K> {}

impl<K: EqualityComparableKind> EqualityComparableKind for Set<K> {
    type Eq<X: Expr<Set<K>>, Y: Expr<Set<K>>> = SetEquals<K, X, Y>;
}

impl<K: EqualityComparableKind> KindWithDefault for Set<K> {
    type Default = EmptySet<K>;
}

impl<K: KindWithDefault + EqualityComparableKind + KindWithDebugForm> KindWithDebugForm for Set<K> {
    type DebugForm<S: Expr<Set<K>>> =
        VisitSet<K, ExprWrapper<Set<K>>, S, ToSetDebugFormVisitor<K>>;
}

pub trait SetVisitor<ElemK: EqualityComparableKind, OutK: Kind> {
    type VisitEmptySet: Expr<OutK>;
    type VisitCons<Elem: Expr<ElemK>, Tail: Expr<Set<ElemK>>>: Expr<OutK>;
}

meta!{
    pub type VisitSet<
        ElemK: EqualityComparableKind,
        OutK: Kind,
        S: Expr<Set<ElemK>>,
        V: SetVisitor<ElemK, OutK>
    >: Expr<OutK> =
        VisitList<ElemK, OutK, <AsSet<ElemK, S> as SetTrait<ElemK>>::GetList, SetVisitorToListVisitorAdapter<ElemK, OutK, V>>;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    meta!{
        pub struct ToSetDebugFormVisitor<K: KindWithDefault + EqualityComparableKind + KindWithDebugForm>: SetVisitor<K, ExprWrapper<Set<K>>> {
            type VisitEmptySet = WrapExpr<Set<K>, EmptySet<K>>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<Set<K>>> =
                WrapExpr<Set<K>, AddToSet<K,
                    UnwrapExpr<K, K::DebugForm<Elem>>,
                    UnwrapExpr<Set<K>, VisitSet<K, ExprWrapper<Set<K>>, Tail, ToSetDebugFormVisitor<K>>>
                >>;
        }

        pub struct SetVisitorToListVisitorAdapter<
            ElemK: EqualityComparableKind,
            OutK: Kind,
            V: SetVisitor<ElemK, OutK>
        >: ListVisitor<ElemK, OutK> {
            type VisitEmptyList = V::VisitEmptySet;
            type VisitCons<Elem: Expr<ElemK>, Tail: Expr<List<ElemK>>> = V::VisitCons<Elem, ListToSetUnchecked<ElemK, Tail>>;
        }

        pub struct ListToSetUnchecked<
            ElemK: EqualityComparableKind,
            L: Expr<List<ElemK>>
        >: Expr<Set<ElemK>> {
            type Eval = WrapSetValue<ElemK, ListToSetUncheckedValue<ElemK, L>>;
        }

        pub struct ListToSetUncheckedValue<
            ElemK: EqualityComparableKind,
            L: Expr<List<ElemK>>
        >: SetValue<ElemK> {
            type Impl = ListToSetUncheckedImpl<ElemK, L>;
        }

        pub struct ListToSetUncheckedImpl<
            ElemK: EqualityComparableKind,
            L: Expr<List<ElemK>>
        >: SetTrait<ElemK> {
            type GetList = L;
        }
    }

    pub trait SetValue<K: EqualityComparableKind> {
        type Impl: SetTrait<K>;
    }

    meta!{
        pub struct WrapSetValue<
            K: EqualityComparableKind,
            U: SetValue<K>
        >: Value<Set<K>> {
            type UnconstrainedImpl = <U as SetValue<K>>::Impl;
        }
    }

    pub trait SetTrait<K: EqualityComparableKind> {
        type GetList: Expr<List<K>>;
    }

    pub struct AsSet<K: EqualityComparableKind, S: Expr<Set<K>>> {
        k: PhantomData<K>,
        s: PhantomData<S>,
    }

    impl<K: EqualityComparableKind, S: Expr<Set<K>>> SetTrait<K> for AsSet<K, S> {
        default type GetList = EmptyList<K>;
    }

    impl<K:EqualityComparableKind, S: Expr<Set<K>>> SetTrait<K> for AsSet<K, S> where <<S as Expr<Set<K>>>::Eval as Value<Set<K>>>::UnconstrainedImpl: SetTrait<K> {
        type GetList = <<<S as Expr<Set<K>>>::Eval as Value<Set<K>>>::UnconstrainedImpl as SetTrait<K>>::GetList;
    }

    meta!{
        pub type SetEquals<
            K: EqualityComparableKind,
            X: Expr<Set<K>>,
            Y: Expr<Set<K>>
        >: Expr<Bool> =
            And<IsSubset<K, X, Y>, IsSubset<K, Y, X>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn equals() {
        meta_assert_eq!(Set<Type>, type_set!{}, type_set![]);
        meta_assert_eq!(Set<Type>, type_set!{i32}, type_set!{i32});
        meta_assert_eq!(Set<Type>, type_set!{i32, u32}, type_set!{u32, i32});
        meta_assert_not_eq!(Set<Type>, type_set!{}, type_set!{i32});
        meta_assert_not_eq!(Set<Type>, type_set!{i32}, type_set!{u64});
    }

    #[test]
    fn default() {
        meta_assert_eq!(Set<Type>, Default<Set<Type>>, type_set!{});
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(ExprWrapper<Set<Bool>>,
            DebugForm<Set<Bool>, AddToSet<Bool, And<True, False>, EmptySet<Bool>>>,
            WrapExpr<Set<Bool>, AddToSet<Bool, False, EmptySet<Bool>>>);
    }
}
