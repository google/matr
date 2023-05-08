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

mod empty_list;
mod cons;
mod to_vec;
mod reverse_list;
mod reversed_list_concat;
mod to_usize_vec;
mod list_concat;
mod is_in_list;
mod remove_from_list;
mod to_type_nested_tuple;
mod list_size;
pub mod type_list;
mod to_type_pair_nested_tuple;
mod to_type_triple_nested_tuple;
mod meta_list;

pub use empty_list::*;
pub use cons::*;
pub use to_vec::*;
pub use reverse_list::*;
pub use reversed_list_concat::*;
pub use list_concat::*;
pub use to_usize_vec::*;
pub use is_in_list::*;
pub use remove_from_list::*;
pub use to_type_nested_tuple::*;
pub use list_size::*;
pub use to_type_pair_nested_tuple::*;
pub use crate::set::meta_set::*;
pub use to_type_triple_nested_tuple::*;
pub use meta_list::*;

use std::marker::PhantomData;
use internal::*;
use crate::expr_wrapper::*;

pub struct List<K: Kind> {
    k: PhantomData<K>,
}

impl<K: Kind> Kind for List<K> {}

impl<K: EqualityComparableKind> EqualityComparableKind for List<K> {
    type Eq<X: Expr<List<K>>, Y: Expr<List<K>>> = ListEquals<K, X, Y>;
}

impl<K: Kind> KindWithDefault for List<K> {
    type Default = EmptyList<K>;
}

impl<K: KindWithDefault + EqualityComparableKind + KindWithDebugForm> KindWithDebugForm for List<K> {
    type DebugForm<L: Expr<List<K>>> = VisitList<K, ExprWrapper<List<K>>, L, ToListDebugFormVisitor<K>>;
}

pub trait ListVisitor<ElemK: Kind, OutK: Kind> {
    type VisitEmptyList: Expr<OutK>;
    type VisitCons<Elem: Expr<ElemK>, Tail: Expr<List<ElemK>>>: Expr<OutK>;
}

meta!{
    pub type VisitList<
        ElemK: Kind,
        OutK: Kind,
        L: Expr<List<ElemK>>,
        V: ListVisitor<ElemK, OutK>
    >: Expr<OutK> =
        <AsList<ElemK, L> as ListTrait<ElemK>>::Visit<OutK, V>;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;
    pub use super::*;
    pub use super::super::*;
    use crate::bool::*;

    pub trait ListValue<K: Kind> {
        type Impl: ListTrait<K>;
    }

    meta!{
        pub struct WrapListValue<
            K: Kind,
            U: ListValue<K>
        >: Value<List<K>> {
            type UnconstrainedImpl = <U as ListValue<K>>::Impl;
        }
    }

    pub struct AsList<K: Kind, L: Expr<List<K>>> {
        k: PhantomData<K>,
        l: PhantomData<L>,
    }

    impl<K: Kind, L: Expr<List<K>>> ListTrait<K> for AsList<K, L> {
        default type Visit<OutK: Kind, V: ListVisitor<K, OutK>> = V::VisitEmptyList;
    }

    impl<K:Kind, L: Expr<List<K>>> ListTrait<K> for AsList<K, L> where <<L as Expr<List<K>>>::Eval as Value<List<K>>>::UnconstrainedImpl: ListTrait<K> {
        type Visit<OutK: Kind, V: ListVisitor<K, OutK>> = <<<L as Expr<List<K>>>::Eval as Value<List<K>>>::UnconstrainedImpl as ListTrait<K>>::Visit<OutK, V>;
    }

    pub trait ListTrait<K: Kind> {
        type Visit<OutK: Kind, V: ListVisitor<K, OutK>>: Expr<OutK>;
    }

    meta!{
        pub struct ToListDebugFormVisitor<K: KindWithDefault + KindWithDebugForm>: ListVisitor<K, ExprWrapper<List<K>>> {
            type VisitEmptyList = WrapExpr<List<K>, EmptyList<K>>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = WrapExpr<List<K>, Cons<K,
                UnwrapExpr<K, K::DebugForm<Elem>>,
                UnwrapExpr<List<K>, VisitList<K, ExprWrapper<List<K>>, Tail, ToListDebugFormVisitor<K>>>
            >>;
        }

        pub type ListEquals<
            K: EqualityComparableKind,
            X: Expr<List<K>>,
            Y: Expr<List<K>>
        >: Expr<Bool> =
            VisitList<K, Bool, X, ListEqualsVisitor<K, Y>>;

        pub struct ListEqualsVisitor<
            K: EqualityComparableKind,
            Other: Expr<List<K>>
        >: ListVisitor<K, Bool> {
            type VisitEmptyList = <AsList<K, Other> as ListTrait<K>>::Visit<Bool, IsEmpty<K>>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = <AsList<K, Other> as ListTrait<K>>::Visit<Bool, ListEqualsCons<K, Elem, Tail>>;
        }

        pub struct IsEmpty<
            K: EqualityComparableKind
        >: ListVisitor<K, Bool> {
            type VisitEmptyList = True;
            type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = False;
        }

        pub struct ListEqualsCons<
            K: EqualityComparableKind,
            Elem: Expr<K>,
            Tail: Expr<List<K>>
        >: ListVisitor<K, Bool> {
            type VisitEmptyList = False;
            type VisitCons<Elem2: Expr<K>, Tail2: Expr<List<K>>> = And<Equals<K, Elem, Elem2>, Equals<List<K>, Tail, Tail2>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::*;
    use crate::bool::*;
    use crate::r#type::*;

    #[test]
    fn equals() {
        meta_assert_eq!(List<Type>, type_list![], type_list![]);
        meta_assert_eq!(List<Type>, type_list![i32, u64], type_list![i32, u64]);
        meta_assert_not_eq!(List<Type>, type_list![], type_list![i32]);
        meta_assert_not_eq!(List<Type>, type_list![i32, u64], type_list![u64, i32]);
    }

    #[test]
    fn default() {
        meta_assert_eq!(List<Type>, Default<List<Type>>, type_list![]);
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(ExprWrapper<List<Bool>>, DebugForm<List<Bool>, Cons<Bool, And<True, False>, EmptyList<Bool>>>, WrapExpr<List<Bool>, Cons<Bool, False, EmptyList<Bool>>>);
    }
}
