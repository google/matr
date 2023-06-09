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

mod get_first;
mod get_second;
mod cons_pair;

pub use get_first::*;
pub use get_second::*;
pub use cons_pair::*;

use std::marker::PhantomData;
use internal::*;
use crate::expr_wrapper::*;

pub struct Pair<FirstK: Kind, SecondK: Kind> {
    first_k: PhantomData<FirstK>,
    second_k: PhantomData<SecondK>,
}

impl<FirstK: Kind, SecondK: Kind> Kind for Pair<FirstK, SecondK> {}

impl<FirstK: EqualityComparableKind + KindWithDefault, SecondK: EqualityComparableKind + KindWithDefault> EqualityComparableKind for Pair<FirstK, SecondK> {
    type Eq<X: Expr<Pair<FirstK, SecondK>>, Y: Expr<Pair<FirstK, SecondK>>> = PairEquals<FirstK, SecondK, X, Y>;
}

impl<FirstK: KindWithDefault, SecondK: KindWithDefault> KindWithDefault for Pair<FirstK, SecondK> {
    type Default = ConsPair<FirstK, SecondK, FirstK::Default, SecondK::Default>;
}

impl<FirstK: KindWithDefault + KindWithDebugForm, SecondK: KindWithDefault + KindWithDebugForm> KindWithDebugForm for Pair<FirstK, SecondK> {
    type DebugForm<E: Expr<Self>> = WrapExpr<Pair<FirstK, SecondK>, ConsPair<FirstK, SecondK,
        UnwrapExpr<FirstK, FirstK::DebugForm<GetFirst<FirstK, SecondK, E>>>,
        UnwrapExpr<SecondK, SecondK::DebugForm<GetSecond<FirstK, SecondK, E>>>
    >>;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;
    pub use super::*;
    use crate::bool::*;

    pub trait PairValue<FirstK: Kind, SecondK: Kind> {
        type Impl: PairTrait<FirstK, SecondK>;
    }

    meta!{
        pub struct WrapPairValue<
            FirstK: Kind, 
            SecondK: Kind, 
            U: PairValue<FirstK, SecondK>
        >: Value<Pair<FirstK, SecondK>> {
            type UnconstrainedImpl = <U as PairValue<FirstK, SecondK>>::Impl;
        }
    }

    pub struct AsPair<FirstK: Kind, SecondK: Kind, N: Expr<Pair<FirstK, SecondK>>> {
        first_k: PhantomData<FirstK>,
        second_k: PhantomData<SecondK>,
        n: PhantomData<N>,
    }

    impl<FirstK: KindWithDefault, SecondK: KindWithDefault, P: Expr<Pair<FirstK, SecondK>>> PairTrait<FirstK, SecondK> for AsPair<FirstK, SecondK, P> {
        default type Visit<ResultK: Kind, Visitor: PairVisitor<FirstK, SecondK, ResultK>> = Visitor::Visit<FirstK::Default, SecondK::Default>;
    }

    impl<FirstK: KindWithDefault, SecondK: KindWithDefault, P: Expr<Pair<FirstK, SecondK>>> PairTrait<FirstK, SecondK> for AsPair<FirstK, SecondK, P> where <<P as Expr<Pair<FirstK, SecondK>>>::Eval as Value<Pair<FirstK, SecondK>>>::UnconstrainedImpl: PairTrait<FirstK, SecondK> {
        type Visit<ResultK: Kind, Visitor: PairVisitor<FirstK, SecondK, ResultK>> = <<<P as Expr<Pair<FirstK, SecondK>>>::Eval as Value<Pair<FirstK, SecondK>>>::UnconstrainedImpl as PairTrait<FirstK, SecondK>>::Visit<ResultK, Visitor>;
    }

    pub trait PairTrait<FirstK: Kind, SecondK: Kind> {
        type Visit<ResultK: Kind, Visitor: PairVisitor<FirstK, SecondK, ResultK>>: Expr<ResultK>;
    }

    pub trait PairVisitor<FirstK: Kind, SecondK: Kind, ResultK: Kind> {
        type Visit<First: Expr<FirstK>, Second: Expr<SecondK>>: Expr<ResultK>;
    }
    
    meta!{
        pub type PairEquals<
            FirstK: EqualityComparableKind + KindWithDefault, 
            SecondK: EqualityComparableKind + KindWithDefault, 
            X: Expr<Pair<FirstK, SecondK>>, 
            Y: Expr<Pair<FirstK, SecondK>>
        >: Expr<Bool> =
            And<
                Equals<FirstK, GetFirst<FirstK, SecondK, X>, GetFirst<FirstK, SecondK, Y>>,
                Equals<SecondK, GetSecond<FirstK, SecondK, X>, GetSecond<FirstK, SecondK, Y>>>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::bool::*;
    use crate::option::*;
    use crate::pair::*;
    use crate::r#type::*;

    #[test]
    fn equals() {
        meta_assert_eq!(Pair<Type, Type>, ConsPair<Type, Type, WrapType<i32>, WrapType<i64>>, ConsPair<Type, Type, WrapType<i32>, WrapType<i64>>);
        meta_assert_not_eq!(Pair<Type, Type>, ConsPair<Type, Type, WrapType<i32>, WrapType<i64>>, ConsPair<Type, Type, WrapType<i64>, WrapType<i32>>);
    }

    #[test]
    fn default() {
        meta_assert_eq!(Pair<Option<Type>, Option<Type>>,
            Default<Pair<Option<Type>, Option<Type>>>, ConsPair<Option<Type>, Option<Type>, None<Type>, None<Type>>);
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(ExprWrapper<Pair<Bool, Bool>>,
            DebugForm<Pair<Bool, Bool>, ConsPair<Bool, Bool, And<True, False>, Or<True, False>>>,
            WrapExpr<Pair<Bool, Bool>, ConsPair<Bool, Bool, False, True>>);
    }
}
