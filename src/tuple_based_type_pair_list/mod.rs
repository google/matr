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

mod list_to_tuple_based_type_pair_list;
mod tuple_based_type_pair_list_to_list;

pub use list_to_tuple_based_type_pair_list::*;
pub use tuple_based_type_pair_list_to_list::*;

use internal::*;

// This is conceptually similar to List<Pair<Type, Type>>, but it represents the list as a nested tuple:
// ((T0, U0), ((T1, U1), ..., () )...)
// This can be useful to "encode" a metatype used as a type parameter in a way that's readable in
// compile error messages, while at the same time being able to constrain the tuple type with a
// trait (TupleBasedTypePairListTrait) that guarantees the ability to reconstruct the
// List<Pair<Type, Type>>.
// For lists of other meta-types (not Pair<Type, Type>), TupleBasedList<K> is a more general
// alternative (but that results in more verbose types when K=Pair<Type, Type>).
pub struct TupleBasedTypePairList {}

impl Kind for TupleBasedTypePairList {}

impl EqualityComparableKind for TupleBasedTypePairList {
    type Eq<X: Expr<TupleBasedTypePairList>, Y: Expr<TupleBasedTypePairList>> = TupleBasedTypePairListEquals<X, Y>;
}

impl KindWithDefault for TupleBasedTypePairList {
    type Default = WrapTupleBasedTypePairList<()>;
}

impl KindWithDebugForm for TupleBasedTypePairList {
    type DebugForm<L: Expr<TupleBasedTypePairList>> = VisitTupleBasedTypePairList<ExprWrapper<TupleBasedTypePairList>, L, ToTupleBasedTypePairListDebugFormVisitor>;
}

pub trait TupleBasedTypePairListVisitor<OutK: Kind> {
    type VisitEmptyTupleBasedTypePairList: Expr<OutK>;
    type VisitCons<Elem: Expr<Pair<Type, Type>>, Tail: Expr<TupleBasedTypePairList>>: Expr<OutK>;
}

meta!{
    pub type VisitTupleBasedTypePairList<
        OutK: Kind,
        L: Expr<TupleBasedTypePairList>,
        V: TupleBasedTypePairListVisitor<OutK>
    >: Expr<OutK> =
        <UnwrapTupleBasedTypePairList<L> as TupleBasedTypePairListTrait>::Visit<OutK, V>;
}

meta!{
    pub struct WrapTupleBasedTypePairList<
        L: TupleBasedTypePairListTrait
    >: Expr<TupleBasedTypePairList> {
        type Eval = WrapTupleBasedTypePairListValue<WrapTupleBasedTypePairListOuterTraitAsValue<WrapTupleBasedTypePairListTraitAsOuterTrait<L>>>;
    }
}

// This expects:
// L: Expr<TupleBasedTypePairList>
// and the unwrapped type implements TupleBasedTypePairListTrait.
pub type UnwrapTupleBasedTypePairList<L> = <AsTupleBasedTypePairList<L> as TupleBasedTypePairListOuterTrait>::GetTuple;

pub trait TupleBasedTypePairListTrait {
    type Visit<OutK: Kind, V: TupleBasedTypePairListVisitor<OutK>>: Expr<OutK>;
}

impl TupleBasedTypePairListTrait for () {
    type Visit<OutK: Kind, V: TupleBasedTypePairListVisitor<OutK>> = V::VisitEmptyTupleBasedTypePairList;
}

impl<HeadFirst, HeadSecond, Tail: TupleBasedTypePairListTrait> TupleBasedTypePairListTrait for ((HeadFirst, HeadSecond), Tail) {
    type Visit<OutK: Kind, V: TupleBasedTypePairListVisitor<OutK>> = V::VisitCons<ConsPair<Type, Type, WrapType<HeadFirst>, WrapType<HeadSecond>>, WrapTupleBasedTypePairList<Tail>>;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub trait TupleBasedTypePairListValue {
        type Impl: TupleBasedTypePairListOuterTrait;
    }

    meta!{
        pub struct WrapTupleBasedTypePairListTraitAsOuterTrait<
            U: TupleBasedTypePairListTrait
        >: TupleBasedTypePairListOuterTrait {
            type GetTuple = U;
        }

        pub struct WrapTupleBasedTypePairListOuterTraitAsValue<
            U: TupleBasedTypePairListOuterTrait
        >: TupleBasedTypePairListValue {
            type Impl = U;
        }

        pub struct WrapTupleBasedTypePairListValue<
            U: TupleBasedTypePairListValue
        >: Value<TupleBasedTypePairList> {
            type UnconstrainedImpl = <U as TupleBasedTypePairListValue>::Impl;
        }
    }

    pub struct AsTupleBasedTypePairList<L: Expr<TupleBasedTypePairList>> {
        l: PhantomData<L>,
    }

    pub trait TupleBasedTypePairListOuterTrait {
        type GetTuple: TupleBasedTypePairListTrait;
    }

    impl<L: Expr<TupleBasedTypePairList>> TupleBasedTypePairListOuterTrait for AsTupleBasedTypePairList<L> {
        default type GetTuple = ();
    }

    impl<L: Expr<TupleBasedTypePairList>> TupleBasedTypePairListOuterTrait for AsTupleBasedTypePairList<L> where <<L as Expr<TupleBasedTypePairList>>::Eval as Value<TupleBasedTypePairList>>::UnconstrainedImpl: TupleBasedTypePairListOuterTrait {
        type GetTuple = <<<L as Expr<TupleBasedTypePairList>>::Eval as Value<TupleBasedTypePairList>>::UnconstrainedImpl as TupleBasedTypePairListOuterTrait>::GetTuple;
    }

    meta!{
        pub struct ToTupleBasedTypePairListDebugFormVisitor: TupleBasedTypePairListVisitor<ExprWrapper<TupleBasedTypePairList>> {
            type VisitEmptyTupleBasedTypePairList = WrapExpr<TupleBasedTypePairList, WrapTupleBasedTypePairList<()>>;
            type VisitCons<Elem: Expr<Pair<Type, Type>>, Tail: Expr<TupleBasedTypePairList>> = WrapExpr<TupleBasedTypePairList, WrapTupleBasedTypePairList<(
                (
                    UnwrapType<GetFirst<Type, Type, UnwrapExpr<Pair<Type, Type>, DebugForm<Pair<Type, Type>, Elem>>>>,
                    UnwrapType<GetSecond<Type, Type, UnwrapExpr<Pair<Type, Type>, DebugForm<Pair<Type, Type>, Elem>>>>
                ),
                UnwrapTupleBasedTypePairList<UnwrapExpr<TupleBasedTypePairList, VisitTupleBasedTypePairList<ExprWrapper<TupleBasedTypePairList>, Tail, ToTupleBasedTypePairListDebugFormVisitor>>>
            )>>;
        }

        pub type TupleBasedTypePairListEquals<
            X: Expr<TupleBasedTypePairList>,
            Y: Expr<TupleBasedTypePairList>
        >: Expr<Bool> =
            VisitTupleBasedTypePairList<Bool, X, TupleBasedTypePairListEqualsVisitor<Y>>;

        pub struct TupleBasedTypePairListEqualsVisitor<
            Other: Expr<TupleBasedTypePairList>
        >: TupleBasedTypePairListVisitor<Bool> {
            type VisitEmptyTupleBasedTypePairList = <UnwrapTupleBasedTypePairList<Other> as TupleBasedTypePairListTrait>::Visit<Bool, IsTupleBasedTypePairListEmpty>;
            type VisitCons<Elem: Expr<Pair<Type, Type>>, Tail: Expr<TupleBasedTypePairList>> = <UnwrapTupleBasedTypePairList<Other> as TupleBasedTypePairListTrait>::Visit<Bool, TupleBasedTypePairListEqualsCons<Elem, Tail>>;
        }

        pub struct IsTupleBasedTypePairListEmpty: TupleBasedTypePairListVisitor<Bool> {
            type VisitEmptyTupleBasedTypePairList = True;
            type VisitCons<Elem: Expr<Pair<Type, Type>>, Tail: Expr<crate::TupleBasedTypePairList>> = False;
        }

        pub struct TupleBasedTypePairListEqualsCons<
            Elem: Expr<Pair<Type, Type>>,
            Tail: Expr<TupleBasedTypePairList>
        >: TupleBasedTypePairListVisitor<Bool> {
            type VisitEmptyTupleBasedTypePairList = False;
            type VisitCons<Elem2: Expr<Pair<Type, Type>>, Tail2: Expr<TupleBasedTypePairList>> = And<Equals<Pair<Type, Type>, Elem, Elem2>, Equals<TupleBasedTypePairList, Tail, Tail2>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn equals() {
        meta_assert_eq!(TupleBasedTypePairList, WrapTupleBasedTypePairList<()>, WrapTupleBasedTypePairList<()>);
        meta_assert_eq!(TupleBasedTypePairList, WrapTupleBasedTypePairList<((i32, &i32), ((u64, &u64), ()))>, WrapTupleBasedTypePairList<((i32, &i32), ((u64, &u64), ()))>);
        meta_assert_not_eq!(TupleBasedTypePairList, WrapTupleBasedTypePairList<()>, WrapTupleBasedTypePairList<((i32, &i32), ())>);
        meta_assert_not_eq!(TupleBasedTypePairList, WrapTupleBasedTypePairList<((i32, &i32), ((u64, &u64), ()))>, WrapTupleBasedTypePairList<((u64, &u64), ((i32, &i32), ()))>);
    }

    #[test]
    fn default() {
        meta_assert_eq!(TupleBasedTypePairList, Default<TupleBasedTypePairList>, WrapTupleBasedTypePairList<()>);
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(
            ExprWrapper<TupleBasedTypePairList>,
            DebugForm<TupleBasedTypePairList, WrapTupleBasedTypePairList<((i32, &i32), ())>>,
            WrapExpr<TupleBasedTypePairList, WrapTupleBasedTypePairList<((i32, &i32), ())>>);
    }
}
