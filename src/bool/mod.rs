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

mod and;
mod r#true;
mod r#false;
mod r#if;
mod not;
mod or;
mod to_bool;
mod if_result;

pub use and::*;
pub use r#true::*;
pub use r#false::*;
pub use r#if::*;
pub use not::*;
pub use or::*;
pub use to_bool::*;
pub use if_result::*;

use internal::*;

pub struct Bool {}

impl Kind for Bool {}

impl EqualityComparableKind for Bool {
    type Eq<X: Expr<Bool>, Y: Expr<Bool>> = If<Bool, X, Y, Not<Y>>;
}

impl KindWithDefault for Bool {
    type Default = False;
}

impl KindWithDebugForm for Bool {
    type DebugForm<B: Expr<Bool>> = If<ExprWrapper<Bool>, B, WrapExpr<Bool, True>, WrapExpr<Bool, False>>;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
pub(crate) mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub trait BoolTrait {
        type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>>: Expr<ResultK>;
    }

    pub trait BoolValue {
        type Impl: BoolTrait;
    }

    impl<B: BoolValue> Value<Bool> for B {
        type UnconstrainedImpl = <B as BoolValue>::Impl;
    }

    pub struct AsBool<B: Expr<Bool>> {
        b: PhantomData<B>,
    }

    impl<B: Expr<Bool>> BoolTrait for AsBool<B> {
        default type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>> = IfTrue;
    }

    impl<B: Expr<Bool>> BoolTrait for AsBool<B> where <<B as Expr<Bool>>::Eval as Value<Bool>>::UnconstrainedImpl: BoolTrait {
        type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>> = <<<B as Expr<Bool>>::Eval as Value<Bool>>::UnconstrainedImpl as BoolTrait>::Cond<ResultK, IfTrue, IfFalse>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn as_bool() {
        meta_assert_eq!(Bool, False, False);
        meta_assert_eq!(Bool, True, True);
    }

    #[test]
    fn equals() {
        meta_assert_eq!(Bool, Equals<Bool, True, True>, True);
        meta_assert_eq!(Bool, Equals<Bool, False, False>, True);
        meta_assert_eq!(Bool, Equals<Bool, False, True>, False);
        meta_assert_eq!(Bool, Equals<Bool, True, False>, False);
    }

    #[test]
    fn default() {
        meta_assert_eq!(Bool, Default<Bool>, False);
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(ExprWrapper<Bool>, DebugForm<Bool, True>, WrapExpr<Bool, True>);
        meta_assert_eq!(ExprWrapper<Bool>, DebugForm<Bool, False>, WrapExpr<Bool, False>);
    }
}
