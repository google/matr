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

mod is_none;
mod none;
mod some;
mod option_or_value;

pub use is_none::*;
pub use none::*;
pub use some::*;
pub use option_or_value::*;

use std::marker::PhantomData;
use internal::*;

pub struct Option<K: Kind> {
    k: PhantomData<K>,
}

impl<K: Kind> Kind for Option<K> {}

impl<K: EqualityComparableKind> EqualityComparableKind for Option<K> {
    type Eq<X: Expr<Option<K>>, Y: Expr<Option<K>>> = OptionEquals<K, X, Y>;
}

impl<K: Kind> KindWithDefault for Option<K> {
    type Default = None<K>;
}

pub trait OptionVisitor<K: Kind, ResultK: Kind> {
    type VisitNone: Expr<ResultK>;
    type VisitSome<X: Expr<K>>: Expr<ResultK>;
}

meta!{
    pub type VisitOption<
        K: Kind, 
        OutK: Kind, 
        X: Expr<Option<K>>, 
        V: OptionVisitor<K, OutK>
    >: Expr<OutK> = 
        <AsOption<K, X> as OptionTrait<K>>::Visit<OutK, V>;
}

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub trait OptionValue<K: Kind> {
        type Impl: OptionTrait<K>;
    }

    impl<K: Kind, X: OptionValue<K>> Value<Option<K>> for X {
        type UnconstrainedImpl = <X as OptionValue<K>>::Impl;
    }

    pub struct AsOption<K: Kind, X: Expr<Option<K>>> {
        k: PhantomData<K>,
        x: PhantomData<X>,
    }

    impl<K: Kind, X: Expr<Option<K>>> OptionTrait<K> for AsOption<K, X> {
        default type Visit<ResultK: Kind, Visitor: OptionVisitor<K, ResultK>> = Visitor::VisitNone;
    }

    impl<K: Kind, X: Expr<Option<K>>> OptionTrait<K> for AsOption<K, X> where <<X as Expr<Option<K>>>::Eval as Value<Option<K>>>::UnconstrainedImpl: OptionTrait<K> {
        type Visit<ResultK: Kind, Visitor: OptionVisitor<K, ResultK>> = <<<X as Expr<Option<K>>>::Eval as Value<Option<K>>>::UnconstrainedImpl as OptionTrait<K>>::Visit<ResultK, Visitor>;
    }

    pub trait OptionTrait<K: Kind> {
        type Visit<ResultK: Kind, Visitor: OptionVisitor<K, ResultK>>: Expr<ResultK>;
    }
    
    meta!{
        pub type OptionEquals<
            K: EqualityComparableKind,
            X: Expr<Option<K>>,
            Y: Expr<Option<K>>
        >: Expr<Bool> =
            VisitOption<K, Bool, X, OptionEqualsVisitor<K, Y>>;

        pub struct OptionEqualsVisitor<
            K: EqualityComparableKind,
            Other: Expr<Option<K>>
        >: OptionVisitor<K, Bool> {
            type VisitNone = IsNone<K, Other>;
            type VisitSome<X: Expr<K>> = VisitOption<K, Bool, Other, OptionEqualsSomeVisitor<K, X>>;
        }

        pub struct OptionEqualsSomeVisitor<
            K: EqualityComparableKind, 
            Other: Expr<K>
        >: OptionVisitor<K, Bool> {
            type VisitNone = False;
            type VisitSome<X: Expr<K>> = Equals<K, X, Other>;
        }
    }
}
