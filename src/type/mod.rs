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

mod unwrap_type;

pub use unwrap_type::*;

use std::marker::PhantomData;
use internal::*;

pub struct Type {}

impl Kind for Type {}

impl KindWithDefault for Type {
    type Default = WrapType<()>;
}

impl EqualityComparableKind for Type {
    type Eq<X: Expr<Type>, Y: Expr<Type>> = IsEqualToType<X, Y>;
}

impl KindWithDebugForm for Type {
    type DebugForm<T: Expr<Type>> = WrapExpr<Type, WrapType<UnwrapType<T>>>;
}

pub struct WrapType<T> {
    t: PhantomData<T>,
}

impl<T> Expr<Type> for WrapType<T> {
    type Eval = WrapTypeValue<WrapType<T>>;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use crate::*;
    use crate::bool::internal::WrapBoolValue;

    pub trait TypeValue {
        type UnconstrainedImpl;
    }

    impl<T> TypeValue for WrapType<T> {
        type UnconstrainedImpl = T;
    }

    meta!{
        pub struct WrapTypeValue<
            T: TypeValue
        >: Value<Type> {
            type UnconstrainedImpl = <T as TypeValue>::UnconstrainedImpl;
        }
    }

    pub struct IsEqualToTypeImplHelper<X, Y> {
        x: PhantomData<X>,
        y: PhantomData<Y>,
    }

    // This must implement BoolTrait instead of Value<Bool> due to the use of specialization.
    impl<X, Y> crate::bool::internal::BoolTrait for IsEqualToTypeImplHelper<X, Y> {
        default type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>> = IfFalse;
    }

    impl<X> crate::bool::internal::BoolTrait for IsEqualToTypeImplHelper<X, X> {
        type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>> = IfTrue;
    }

    pub struct IsEqualToTypeImpl<X: Value<Type>, Y: Value<Type>> {
        x: PhantomData<X>,
        y: PhantomData<Y>,
    }

    // This must implement BoolTrait instead of Value<Bool> due to the use of specialization.
    impl<X: Value<Type>, Y: Value<Type>> crate::bool::internal::BoolValue for IsEqualToTypeImpl<X, Y> {
        type Impl = IsEqualToTypeImplHelper<X::UnconstrainedImpl, Y::UnconstrainedImpl>;
    }

    meta!{
        pub struct IsEqualToType<
            X: Expr<Type>,
            Y: Expr<Type>
        >: Expr<Bool> {
            type Eval = WrapBoolValue<IsEqualToTypeImpl<X::Eval, Y::Eval>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn equals() {
        meta_assert_eq!(Type, WrapType<i32>, WrapType<i32>);
        meta_assert_not_eq!(Type, WrapType<i32>, WrapType<i64>);
    }

    #[test]
    fn default() {
        meta_assert_eq!(Type, Default<Type>, WrapType<()>);
    }

    #[test]
    fn debug_form() {
        meta_assert_eq!(ExprWrapper<Type>, DebugForm<Type, WrapType<i32>>, WrapExpr<Type, WrapType<i32>>);
    }
}
