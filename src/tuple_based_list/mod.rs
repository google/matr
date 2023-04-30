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

mod list_to_tuple_based_list;
mod tuple_based_list_to_list;

pub use list_to_tuple_based_list::*;
pub use tuple_based_list_to_list::*;

use std::marker::PhantomData;
use internal::*;

// This is conceptually similar to List<K>, but it represents the list as a nested tuple:
// (T0, (T1, ..., () )...)
// This can be useful to "encode" a metatype used as a type parameter in a way that's readable in
// compile error messages, while at the same time being able to constrain the tuple type with a
// trait (TupleBasedListTrait<K>) that guarantees the ability to reconstruct the List.
// When K=Type, prefer using the more specific TupleBasedTypeList, that avoids the WrapType wrappers
// too.
pub struct TupleBasedList<K: Kind> {
    k: PhantomData<K>,
}

impl<K: Kind> Kind for TupleBasedList<K> {}

impl<K: EqualityComparableKind> EqualityComparableKind for TupleBasedList<K> {
    type Eq<X: Expr<TupleBasedList<K>>, Y: Expr<TupleBasedList<K>>> = TupleBasedListEquals<K, X, Y>;
}

impl<K: Kind> KindWithDefault for TupleBasedList<K> {
    type Default = WrapTupleBasedList<K, ()>;
}

impl<K: KindWithDefault + EqualityComparableKind + KindWithDebugForm> KindWithDebugForm for TupleBasedList<K> {
    type DebugForm<L: Expr<TupleBasedList<K>>> = VisitTupleBasedList<K, ExprWrapper<TupleBasedList<K>>, L, ToTupleBasedListDebugFormVisitor<K>>;
}

pub trait TupleBasedListVisitor<ElemK: Kind, OutK: Kind> {
    type VisitEmptyTupleBasedList: Expr<OutK>;
    type VisitCons<Elem: Expr<ElemK>, Tail: Expr<TupleBasedList<ElemK>>>: Expr<OutK>;
}

meta!{
    pub type VisitTupleBasedList<
        ElemK: Kind,
        OutK: Kind,
        L: Expr<TupleBasedList<ElemK>>,
        V: TupleBasedListVisitor<ElemK, OutK>
    >: Expr<OutK> =
        <UnwrapTupleBasedList<ElemK, L> as TupleBasedListTrait<ElemK>>::Visit<OutK, V>;
}

meta!{
    pub struct WrapTupleBasedList<
        K: Kind,
        L: TupleBasedListTrait<K>
    >: Expr<TupleBasedList<K>> {
        type Eval = WrapTupleBasedListValue<K, WrapTupleBasedListOuterTraitAsValue<K, WrapTupleBasedListTraitAsOuterTrait<K, L>>>;
    }
}

// This expects:
// K: Kind
// L: Expr<TupleBasedList<K>>
// and the unwrapped type implements TupleBasedListTrait<K>.
pub type UnwrapTupleBasedList<K, L> = <AsTupleBasedList<K, L> as TupleBasedListOuterTrait<K>>::GetTuple;

pub trait TupleBasedListTrait<K: Kind> {
    type Visit<OutK: Kind, V: TupleBasedListVisitor<K, OutK>>: Expr<OutK>;
}

impl<K: Kind> TupleBasedListTrait<K> for () {
    type Visit<OutK: Kind, V: TupleBasedListVisitor<K, OutK>> = V::VisitEmptyTupleBasedList;
}

impl<K: Kind, Head: Expr<K>, Tail: TupleBasedListTrait<K>> TupleBasedListTrait<K> for (Head, Tail) {
    type Visit<OutK: Kind, V: TupleBasedListVisitor<K, OutK>> = V::VisitCons<Head, WrapTupleBasedList<K, Tail>>;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub trait TupleBasedListValue<K: Kind> {
        type Impl: TupleBasedListOuterTrait<K>;
    }

    meta!{
        pub struct WrapTupleBasedListTraitAsOuterTrait<
            K: Kind,
            U: TupleBasedListTrait<K>
        >: TupleBasedListOuterTrait<K> {
            type GetTuple = U;
        }

        pub struct WrapTupleBasedListOuterTraitAsValue<
            K: Kind,
            U: TupleBasedListOuterTrait<K>
        >: TupleBasedListValue<K> {
            type Impl = U;
        }

        pub struct WrapTupleBasedListValue<
            K: Kind,
            U: TupleBasedListValue<K>
        >: Value<TupleBasedList<K>> {
            type UnconstrainedImpl = <U as TupleBasedListValue<K>>::Impl;
        }
    }

    pub struct AsTupleBasedList<K: Kind, L: Expr<TupleBasedList<K>>> {
        k: PhantomData<K>,
        l: PhantomData<L>,
    }

    pub trait TupleBasedListOuterTrait<K: Kind> {
        type GetTuple: TupleBasedListTrait<K>;
    }

    impl<K: Kind, L: Expr<TupleBasedList<K>>> TupleBasedListOuterTrait<K> for AsTupleBasedList<K, L> {
        default type GetTuple = ();
    }

    impl<K:Kind, L: Expr<TupleBasedList<K>>> TupleBasedListOuterTrait<K> for AsTupleBasedList<K, L> where <<L as Expr<TupleBasedList<K>>>::Eval as Value<TupleBasedList<K>>>::UnconstrainedImpl: TupleBasedListOuterTrait<K> {
        type GetTuple = <<<L as Expr<TupleBasedList<K>>>::Eval as Value<TupleBasedList<K>>>::UnconstrainedImpl as TupleBasedListOuterTrait<K>>::GetTuple;
    }

    meta!{
        pub struct ToTupleBasedListDebugFormVisitor<K: KindWithDefault + KindWithDebugForm>: TupleBasedListVisitor<K, ExprWrapper<TupleBasedList<K>>> {
            type VisitEmptyTupleBasedList = WrapExpr<TupleBasedList<K>, WrapTupleBasedList<K, ()>>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<TupleBasedList<K>>> = WrapExpr<TupleBasedList<K>, WrapTupleBasedList<K, (
                UnwrapExpr<K, K::DebugForm<Elem>>,
                UnwrapTupleBasedList<K, UnwrapExpr<TupleBasedList<K>, VisitTupleBasedList<K, ExprWrapper<TupleBasedList<K>>, Tail, ToTupleBasedListDebugFormVisitor<K>>>>
            )>>;
        }

        pub type TupleBasedListEquals<
            K: EqualityComparableKind,
            X: Expr<TupleBasedList<K>>,
            Y: Expr<TupleBasedList<K>>
        >: Expr<Bool> =
            VisitTupleBasedList<K, Bool, X, TupleBasedListEqualsVisitor<K, Y>>;

        pub struct TupleBasedListEqualsVisitor<
            K: EqualityComparableKind,
            Other: Expr<TupleBasedList<K>>
        >: TupleBasedListVisitor<K, Bool> {
            type VisitEmptyTupleBasedList = <UnwrapTupleBasedList<K, Other> as TupleBasedListTrait<K>>::Visit<Bool, IsTupleBasedListEmpty<K>>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<TupleBasedList<K>>> = <UnwrapTupleBasedList<K, Other> as TupleBasedListTrait<K>>::Visit<Bool, TupleBasedListEqualsCons<K, Elem, Tail>>;
        }

        pub struct IsTupleBasedListEmpty<
            K: EqualityComparableKind
        >: TupleBasedListVisitor<K, Bool> {
            type VisitEmptyTupleBasedList = True;
            type VisitCons<Elem: Expr<K>, Tail: Expr<crate::TupleBasedList<K>>> = False;
        }

        pub struct TupleBasedListEqualsCons<
            K: EqualityComparableKind,
            Elem: Expr<K>,
            Tail: Expr<TupleBasedList<K>>
        >: TupleBasedListVisitor<K, Bool> {
            type VisitEmptyTupleBasedList = False;
            type VisitCons<Elem2: Expr<K>, Tail2: Expr<TupleBasedList<K>>> = And<Equals<K, Elem, Elem2>, Equals<TupleBasedList<K>, Tail, Tail2>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn equals() {
        meta_assert_eq!(TupleBasedList<Type>, WrapTupleBasedList<Type, ()>, WrapTupleBasedList<Type, ()>);
        meta_assert_eq!(TupleBasedList<Type>, WrapTupleBasedList<Type, (WrapType<i32>, (WrapType<u64>, ()))>, WrapTupleBasedList<Type, (WrapType<i32>, (WrapType<u64>, ()))>);
        meta_assert_not_eq!(TupleBasedList<Type>, WrapTupleBasedList<Type, ()>, WrapTupleBasedList<Type, (WrapType<i32>, ())>);
        meta_assert_not_eq!(TupleBasedList<Type>, WrapTupleBasedList<Type, (WrapType<i32>, (WrapType<u64>, ()))>, WrapTupleBasedList<Type, (WrapType<u64>, (WrapType<i32>, ()))>);
    }

    #[test]
    fn default() {
        meta_assert_eq!(TupleBasedList<Type>, Default<TupleBasedList<Type>>, WrapTupleBasedList<Type, ()>);
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(
            ExprWrapper<TupleBasedList<Bool>>,
            DebugForm<TupleBasedList<Bool>, WrapTupleBasedList<Bool, (False, ())>>,
            WrapExpr<TupleBasedList<Bool>, WrapTupleBasedList<Bool, (False, ())>>);
    }
}
