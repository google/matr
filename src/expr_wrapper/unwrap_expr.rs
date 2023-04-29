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

use internal::*;

// Intended to be used with:
// K: KindWithDefault
// E: Expr<ExprWrapper<K>>
// Then UnwrapExpr<K, E>: Expr<K>
pub type UnwrapExpr<K, E> = <UnwrapExprHelper<K, E> as UnwrapExprTrait<K>>::Get;

mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub trait UnwrapExprTrait<K: KindWithDefault> {
        type Get: Expr<K>;
    }

    pub struct UnwrapExprHelper<K: KindWithDefault, E: Expr<ExprWrapper<K>>> {
        k: PhantomData<K>,
        e: PhantomData<E>,
    }

    impl<K: KindWithDefault, E: Expr<ExprWrapper<K>>> UnwrapExprTrait<K> for UnwrapExprHelper<K, E> {
        default type Get = K::Default;
    }

    impl<K: KindWithDefault, E: Expr<ExprWrapper<K>>> UnwrapExprTrait<K> for UnwrapExprHelper<K, E> where <<E as Expr<ExprWrapper<K>>>::Eval as Value<ExprWrapper<K>>>::UnconstrainedImpl: Expr<K> {
        type Get = <<E as Expr<ExprWrapper<K>>>::Eval as Value<ExprWrapper<K>>>::UnconstrainedImpl;
    }
}


#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn unwrap_expr() {
        meta_assert_eq!(ExprWrapper<List<Type>>, WrapExpr<List<Type>, UnwrapExpr<List<Type>, WrapExpr<List<Type>, EmptyList<Type>>>>, WrapExpr<List<Type>, EmptyList<Type>>);
        meta_assert_not_eq!(ExprWrapper<Bool>, WrapExpr<Bool, UnwrapExpr<Bool, WrapExpr<Bool, And<True, True>>>>, WrapExpr<Bool, True>);
    }
}
