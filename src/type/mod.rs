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

pub mod assertions;
mod get_type;

pub use get_type::*;

use std::marker::PhantomData;
use internal::*;

pub struct Type {}

impl Kind for Type {}

pub trait TypeValue {
    type Impl;
}

impl EqualityComparableKind for Type {
    type Eq<X: Expr<Type>, Y: Expr<Type>> = IsEqualToType<X, Y>;
}

pub struct WrapType<T> {
    t: PhantomData<T>,
}

impl<T> Expr<Type> for WrapType<T> {
    type Eval = WrapType<T>;
}

impl<T> TypeValue for WrapType<T> {
    type Impl = T;
}

impl<T: TypeValue> Value<Type> for T {
    type UnconstrainedImpl = <T as TypeValue>::Impl;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use crate::*;

    pub struct IsEqualToTypeImplHelper<X, Y> {
        x: PhantomData<X>,
        y: PhantomData<Y>,
    }

    // This must implement BoolTrait instead of Value<Bool> due to the use of specialization.
    impl<X, Y> BoolTrait for IsEqualToTypeImplHelper<X, Y> {
        default type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>> = IfFalse;
    }

    impl<X> BoolTrait for IsEqualToTypeImplHelper<X, X> {
        type Cond<ResultK: Kind, IfTrue: Expr<ResultK>, IfFalse: Expr<ResultK>> = IfTrue;
    }

    pub struct IsEqualToTypeImpl<X: Value<Type>, Y: Value<Type>> {
        x: PhantomData<X>,
        y: PhantomData<Y>,
    }

    // This must implement BoolTrait instead of Value<Bool> due to the use of specialization.
    impl<X: Value<Type>, Y: Value<Type>> BoolValue for IsEqualToTypeImpl<X, Y> {
        type Impl = IsEqualToTypeImplHelper<X::UnconstrainedImpl, Y::UnconstrainedImpl>;
    }

    pub struct IsEqualToType<X: Expr<Type>, Y: Expr<Type>> {
        x: PhantomData<X>,
        y: PhantomData<Y>,
    }

    impl<X: Expr<Type>, Y: Expr<Type>> Expr<Bool> for IsEqualToType<X, Y> {
        type Eval = IsEqualToTypeImpl<X::Eval, Y::Eval>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::internal::*;
    use crate::bool::assertions::*;

    #[test]
    fn is_equal_to_type() {
        assert_true!(IsEqualToType<WrapType<i32>, WrapType<i32>>);
        assert_false!(IsEqualToType<WrapType<i32>, WrapType<i64>>);
    }
}

//
// impl EqualityComparableKind for TypeKind {
//     type Eq<X: Value<Self>, Y: Value<Self>> = IsEqualToType<X, Y>;
// }

// pub struct TypeHolder<T> {
//     value: PhantomData<T>,
// }
//
// pub trait Type {
//     type Unwrap;
// }
//
// impl<T> Type for TypeHolder<T> {
//     type Unwrap = T;
// }
//
// pub struct AsType<T: Value<TypeKind>> {
//     t: PhantomData<T>,
// }
//
// impl<T: Value<TypeKind>> Type for AsType<T> {
//     default type Unwrap = BottomValue;
// }
//
// impl<T: Type> Type for AsType<T> {
//     type Unwrap = T::Unwrap;
// }

// impl<T> Type for TypeHolder<T> {
//     type Type = T;
//     type IsEqualTo<U: Type> = <TypeHolderIsEqualTo<T, U> as TypeHolderIsEqualToTrait>::Result;
// }
