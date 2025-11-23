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

mod zero;
mod increment;
mod sum;
mod multiply;
mod is_even;
mod to_usize;
mod long_recursion;

pub use zero::*;
pub use increment::*;
pub use sum::*;
pub use multiply::*;
pub use is_even::*;
pub use to_usize::*;
pub use long_recursion::*;

use internal::*;
use crate::expr_wrapper::*;

pub struct USize {}

impl Kind for USize {}

impl EqualityComparableKind for USize {
    type Eq<X: Expr<USize>, Y: Expr<USize>> = USizeEquals<X, Y>;
}

impl KindWithDefault for USize {
    type Default = Zero;
}

impl KindWithDebugForm for USize {
    type DebugForm<E: Expr<Self>> = VisitUSize<ExprWrapper<USize>, E, ToUSizeDebugFormVisitor>;
}

pub const trait USizeVisitor<ResultK: Kind> {
    type VisitZero: Expr<ResultK>;
    type VisitIncrement<N: Expr<USize>>: Expr<ResultK>;
}

meta!{
    pub type VisitUSize<
        K: Kind,
        N: Expr<USize>,
        V: USizeVisitor<K>
    >: Expr<K> =
        <AsUSize<N> as USizeTrait>::Visit<K, V>;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;
    pub use super::*;
    use crate::bool::*;

    pub const trait USizeValue {
        type Impl: USizeTrait;
    }

    meta!{
        pub struct WrapUSizeValue<U: USizeValue>: Value<USize> {
            type UnconstrainedImpl = <U as USizeValue>::Impl;
        }
    }

    pub struct AsUSize<N: Expr<USize>> {
        n: PhantomData<N>,
    }

    impl<N: Expr<USize>> USizeTrait for AsUSize<N> {
        default type Visit<ResultK: Kind, V: USizeVisitor<ResultK>> = V::VisitZero;
    }

    impl<N: Expr<USize>> USizeTrait for AsUSize<N> where <<N as Expr<USize>>::Eval as Value<USize>>::UnconstrainedImpl: USizeTrait {
        type Visit<ResultK: Kind, V: USizeVisitor<ResultK>> = <<<N as Expr<USize>>::Eval as Value<USize>>::UnconstrainedImpl as USizeTrait>::Visit<ResultK, V>;
    }

    pub const trait USizeTrait {
        type Visit<ResultK: Kind, V: USizeVisitor<ResultK>>: Expr<ResultK>;
    }

    meta!{
        pub type USizeEquals<
            X: Expr<USize>,
            Y: Expr<USize>
        >: Expr<Bool> =
            VisitUSize<Bool, X, USizeEqualsVisitor<Y>>;

        pub struct USizeEqualsVisitor<
            Other: Expr<USize>
        >: USizeVisitor<Bool> {
            type VisitZero = <AsUSize<Other> as USizeTrait>::Visit<Bool, IsZero>;
            type VisitIncrement<N: Expr<USize>> = <AsUSize<Other> as USizeTrait>::Visit<Bool, USizeEqualsOnePlus<N>>;
        }

        pub struct IsZero: USizeVisitor<Bool> {
            type VisitZero = True;
            type VisitIncrement<N: Expr<USize>> = False;
        }

        pub struct USizeEqualsOnePlus<
            Other: Expr<USize>
        >: USizeVisitor<Bool> {
            type VisitZero = False;
            type VisitIncrement<N: Expr<USize>> = <AsUSize<N> as USizeTrait>::Visit<Bool, USizeEqualsVisitor<Other>>;
        }

        pub struct ToUSizeDebugFormVisitor: USizeVisitor<ExprWrapper<USize>> {
            type VisitZero = WrapExpr<USize, Zero>;
            type VisitIncrement<N: Expr<USize>> = WrapExpr<USize, Increment<UnwrapExpr<USize, VisitUSize<ExprWrapper<USize>, N, ToUSizeDebugFormVisitor>>>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::usize::*;

    #[test]
    fn equals() {
        meta_assert_eq!(USize, Zero, Zero);
        meta_assert_eq!(USize, Increment<Zero>, Increment<Zero>);
        meta_assert_not_eq!(USize, Zero, Increment<Zero>);
        meta_assert_not_eq!(USize, Increment<Increment<Zero>>, Increment<Zero>);
    }

    #[test]
    fn default() {
        meta_assert_eq!(USize, Default<USize>, Zero);
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(ExprWrapper<USize>, DebugForm<USize, Sum<Zero, Increment<Zero>>>, WrapExpr<USize, Increment<Zero>>);
    }
}
