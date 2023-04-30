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

mod unwrap_expr;

pub use unwrap_expr::*;

use std::marker::PhantomData;
use internal::*;

pub struct ExprWrapper<K: Kind + ?Sized> {
    k: PhantomData<K>,
}

impl<K: Kind + ?Sized> Kind for ExprWrapper<K> {}

impl<K: KindWithDefault> EqualityComparableKind for ExprWrapper<K> {
    type Eq<X: Expr<ExprWrapper<K>>, Y: Expr<ExprWrapper<K>>> = Equals<Type,
        WrapType<UnwrapExpr<K, X>>,
        WrapType<UnwrapExpr<K, Y>>
    >;
}

impl<K: KindWithDefault> KindWithDebugForm for ExprWrapper<K> {
    type DebugForm<E: Expr<ExprWrapper<K>>> = WrapExpr<ExprWrapper<K>, WrapExpr<K, UnwrapExpr<K, E>>>;
}

impl<K: KindWithDefault> KindWithDefault for ExprWrapper<K> {
    type Default = WrapExpr<K, K::Default>;
}

meta!{
    pub struct WrapExpr<
        K: Kind,
        E: Expr<K>
    >: Expr<ExprWrapper<K>> {
        type Eval = WrapExprValue<K, E>;
    }
}


// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use crate::*;

    pub trait ExprWrapperValue<K: Kind> {
        type UnconstrainedImpl;
    }

    impl<K: Kind, E: ExprWrapperValue<K>> Value<ExprWrapper<K>> for E {
        type UnconstrainedImpl = <E as ExprWrapperValue<K>>::UnconstrainedImpl;
    }

    meta!{
        pub struct WrapExprValue<
            K: Kind,
            E: Expr<K>
        >: ExprWrapperValue<K> {
            type UnconstrainedImpl = E;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn equals() {
        meta_assert_eq!(ExprWrapper<List<Type>>, WrapExpr<List<Type>, EmptyList<Type>>, WrapExpr<List<Type>, EmptyList<Type>>);
        meta_assert_not_eq!(ExprWrapper<Bool>, WrapExpr<Bool, And<True, True>>, WrapExpr<Bool, True>);
    }

    #[test]
    fn default() {
        meta_assert_eq!(ExprWrapper<List<Type>>, Default<ExprWrapper<List<Type>>>, WrapExpr<List<Type>, EmptyList<Type>>);
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(ExprWrapper<ExprWrapper<Type>>, DebugForm<ExprWrapper<Type>, WrapExpr<Type, WrapType<i32>>>, WrapExpr<ExprWrapper<Type>, WrapExpr<Type, WrapType<i32>>>);
    }
}
