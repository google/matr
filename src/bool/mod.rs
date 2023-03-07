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
pub mod assertions;
mod r#false;
mod r#if;
mod not;
mod or;

pub use and::*;
pub use r#true::*;
pub use r#false::*;
pub use r#if::*;
pub use not::*;
pub use or::*;

use std::marker::PhantomData;
use internal::*;

pub struct Bool {}

impl Kind for Bool {}

impl EqualityComparableKind for Bool {
    type Eq<X: Expr<Bool>, Y: Expr<Bool>> = If<Bool, X, Y, Not<Y>>;
}

impl KindWithDefault for Bool {
    type Default = False;
}

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

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use crate::*;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::assertions::*;

    #[test]
    fn as_bool() {
        assert_false!(False);
        assert_true!(True);
    }

    #[test]
    fn equals() {
        assert_true!(Equals<Bool, True, True>);
        assert_true!(Equals<Bool, False, False>);
        assert_false!(Equals<Bool, False, True>);
        assert_false!(Equals<Bool, True, False>);
    }
}
