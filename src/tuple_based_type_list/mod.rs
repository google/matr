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

mod list_to_tuple_based_type_list;
mod tuple_based_type_list_to_list;

pub use list_to_tuple_based_type_list::*;
pub use tuple_based_type_list_to_list::*;

use internal::*;

pub struct TupleBasedTypeList {}

impl Kind for TupleBasedTypeList {}

impl EqualityComparableKind for TupleBasedTypeList {
    type Eq<X: Expr<TupleBasedTypeList>, Y: Expr<TupleBasedTypeList>> = TupleBasedTypeListEquals<X, Y>;
}

impl KindWithDefault for TupleBasedTypeList {
    type Default = WrapTupleBasedTypeList<()>;
}

impl KindWithDebugForm for TupleBasedTypeList {
    type DebugForm<L: Expr<TupleBasedTypeList>> = VisitTupleBasedTypeList<ExprWrapper<TupleBasedTypeList>, L, ToTupleBasedTypeListDebugFormVisitor>;
}

pub trait TupleBasedTypeListVisitor<OutK: Kind> {
    type VisitEmptyTupleBasedTypeList: Expr<OutK>;
    type VisitCons<Elem: Expr<Type>, Tail: Expr<TupleBasedTypeList>>: Expr<OutK>;
}

meta!{
    pub type VisitTupleBasedTypeList<
        OutK: Kind,
        L: Expr<TupleBasedTypeList>,
        V: TupleBasedTypeListVisitor<OutK>
    >: Expr<OutK> =
        <UnwrapTupleBasedTypeList<L> as TupleBasedTypeListTrait>::Visit<OutK, V>;
}

meta!{
    pub struct WrapTupleBasedTypeList<
        L: TupleBasedTypeListTrait
    >: Expr<TupleBasedTypeList> {
        type Eval = WrapTupleBasedTypeListValue<WrapTupleBasedTypeListOuterTraitAsValue<WrapTupleBasedTypeListTraitAsOuterTrait<L>>>;
    }
}

// This expects:
// L: Expr<TupleBasedTypeList>
// and the unwrapped type implements TupleBasedTypeListTrait.
pub type UnwrapTupleBasedTypeList<L> = <AsTupleBasedTypeList<L> as TupleBasedTypeListOuterTrait>::GetTuple;

pub trait TupleBasedTypeListTrait {
    type Visit<OutK: Kind, V: TupleBasedTypeListVisitor<OutK>>: Expr<OutK>;
}

impl TupleBasedTypeListTrait for () {
    type Visit<OutK: Kind, V: TupleBasedTypeListVisitor<OutK>> = V::VisitEmptyTupleBasedTypeList;
}

impl<Head, Tail: TupleBasedTypeListTrait> TupleBasedTypeListTrait for (Head, Tail) {
    type Visit<OutK: Kind, V: TupleBasedTypeListVisitor<OutK>> = V::VisitCons<WrapType<Head>, WrapTupleBasedTypeList<Tail>>;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub trait TupleBasedTypeListValue {
        type Impl: TupleBasedTypeListOuterTrait;
    }

    meta!{
        pub struct WrapTupleBasedTypeListTraitAsOuterTrait<
            U: TupleBasedTypeListTrait
        >: TupleBasedTypeListOuterTrait {
            type GetTuple = U;
        }

        pub struct WrapTupleBasedTypeListOuterTraitAsValue<
            U: TupleBasedTypeListOuterTrait
        >: TupleBasedTypeListValue {
            type Impl = U;
        }

        pub struct WrapTupleBasedTypeListValue<
            U: TupleBasedTypeListValue
        >: Value<TupleBasedTypeList> {
            type UnconstrainedImpl = <U as TupleBasedTypeListValue>::Impl;
        }
    }

    pub struct AsTupleBasedTypeList<L: Expr<TupleBasedTypeList>> {
        l: PhantomData<L>,
    }

    pub trait TupleBasedTypeListOuterTrait {
        type GetTuple: TupleBasedTypeListTrait;
    }

    impl<L: Expr<TupleBasedTypeList>> TupleBasedTypeListOuterTrait for AsTupleBasedTypeList<L> {
        default type GetTuple = ();
    }

    impl<L: Expr<TupleBasedTypeList>> TupleBasedTypeListOuterTrait for AsTupleBasedTypeList<L> where <<L as Expr<TupleBasedTypeList>>::Eval as Value<TupleBasedTypeList>>::UnconstrainedImpl: TupleBasedTypeListOuterTrait {
        type GetTuple = <<<L as Expr<TupleBasedTypeList>>::Eval as Value<TupleBasedTypeList>>::UnconstrainedImpl as TupleBasedTypeListOuterTrait>::GetTuple;
    }

    meta!{
        pub struct ToTupleBasedTypeListDebugFormVisitor: TupleBasedTypeListVisitor<ExprWrapper<TupleBasedTypeList>> {
            type VisitEmptyTupleBasedTypeList = WrapExpr<TupleBasedTypeList, WrapTupleBasedTypeList<()>>;
            type VisitCons<Elem: Expr<Type>, Tail: Expr<TupleBasedTypeList>> = WrapExpr<TupleBasedTypeList, WrapTupleBasedTypeList<(
                UnwrapType<UnwrapExpr<Type, DebugForm<Type, Elem>>>,
                UnwrapTupleBasedTypeList<UnwrapExpr<TupleBasedTypeList, VisitTupleBasedTypeList<ExprWrapper<TupleBasedTypeList>, Tail, ToTupleBasedTypeListDebugFormVisitor>>>
            )>>;
        }

        pub type TupleBasedTypeListEquals<
            X: Expr<TupleBasedTypeList>,
            Y: Expr<TupleBasedTypeList>
        >: Expr<Bool> =
            VisitTupleBasedTypeList<Bool, X, TupleBasedTypeListEqualsVisitor<Y>>;

        pub struct TupleBasedTypeListEqualsVisitor<
            Other: Expr<TupleBasedTypeList>
        >: TupleBasedTypeListVisitor<Bool> {
            type VisitEmptyTupleBasedTypeList = <UnwrapTupleBasedTypeList<Other> as TupleBasedTypeListTrait>::Visit<Bool, IsTupleBasedTypeListEmpty>;
            type VisitCons<Elem: Expr<Type>, Tail: Expr<TupleBasedTypeList>> = <UnwrapTupleBasedTypeList<Other> as TupleBasedTypeListTrait>::Visit<Bool, TupleBasedTypeListEqualsCons<Elem, Tail>>;
        }

        pub struct IsTupleBasedTypeListEmpty: TupleBasedTypeListVisitor<Bool> {
            type VisitEmptyTupleBasedTypeList = True;
            type VisitCons<Elem: Expr<Type>, Tail: Expr<crate::TupleBasedTypeList>> = False;
        }

        pub struct TupleBasedTypeListEqualsCons<
            Elem: Expr<Type>,
            Tail: Expr<TupleBasedTypeList>
        >: TupleBasedTypeListVisitor<Bool> {
            type VisitEmptyTupleBasedTypeList = False;
            type VisitCons<Elem2: Expr<Type>, Tail2: Expr<TupleBasedTypeList>> = And<Equals<Type, Elem, Elem2>, Equals<TupleBasedTypeList, Tail, Tail2>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn equals() {
        meta_assert_eq!(TupleBasedTypeList, WrapTupleBasedTypeList<()>, WrapTupleBasedTypeList<()>);
        meta_assert_eq!(TupleBasedTypeList, WrapTupleBasedTypeList<(i32, (u64, ()))>, WrapTupleBasedTypeList<(i32, (u64, ()))>);
        meta_assert_not_eq!(TupleBasedTypeList, WrapTupleBasedTypeList<()>, WrapTupleBasedTypeList<(i32, ())>);
        meta_assert_not_eq!(TupleBasedTypeList, WrapTupleBasedTypeList<(i32, (u64, ()))>, WrapTupleBasedTypeList<(u64, (i32, ()))>);
    }

    #[test]
    fn default() {
        meta_assert_eq!(TupleBasedTypeList, Default<TupleBasedTypeList>, WrapTupleBasedTypeList<()>);
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(
            ExprWrapper<TupleBasedTypeList>,
            DebugForm<TupleBasedTypeList, WrapTupleBasedTypeList<(i32, ())>>,
            WrapExpr<TupleBasedTypeList, WrapTupleBasedTypeList<(i32, ())>>);
    }
}
