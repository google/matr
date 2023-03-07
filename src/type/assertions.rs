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

macro_rules! assert_type_eq {
($X:ty, $Y:ty) => {{
        crate::r#type::assertions::assert_raw_type_eq!(
            <GetType<$X> as GetTypeTrait>::Get,
            <GetType<$Y> as GetTypeTrait>::Get);
    }};
}
pub(crate) use assert_type_eq;

macro_rules! assert_type_result_eq {
($X:ty, $Y:ty) => {{
        const _: () = crate::check_no_error::<Type, $X>();
        const _: () = crate::check_no_error::<Type, $Y>();
        crate::r#type::assertions::assert_raw_type_eq!(
            <GetType<OrValue<Type, $X, WrapType<()>>> as GetTypeTrait>::Get,
            <GetType<OrValue<Type, $Y, WrapType<()>>> as GetTypeTrait>::Get);
    }};
}
pub(crate) use assert_type_result_eq;

macro_rules! assert_type_not_eq {
($X:ty, $Y:ty) => {{
        crate::r#type::assertions::assert_raw_type_not_eq!(
            <GetType<$X> as GetTypeTrait>::Get,
            <GetType<$Y> as GetTypeTrait>::Get);
    }};
}
pub(crate) use assert_type_not_eq;

macro_rules! assert_type_result_not_eq {
($X:ty, $Y:ty) => {{
        const _: () = crate::check_no_error::<Type, $X>();
        const _: () = crate::check_no_error::<Type, $Y>();
        crate::r#type::assertions::assert_raw_type_not_eq!(
            <GetType<OrValue<Type, $X, WrapType<()>>> as GetTypeTrait>::Get,
            <GetType<OrValue<Type, $Y, WrapType<()>>> as GetTypeTrait>::Get);
    }};
}
pub(crate) use assert_type_result_not_eq;

macro_rules! assert_raw_type_eq {
        ($X:ty, $Y:ty) => {{
            const _: () = <crate::r#type::assertions::internal::AssertRawTypeEq<$X, $Y> as crate::r#type::assertions::internal::AssertRawTypeEqTrait>::check_equal();
        }};
    }
pub(crate) use assert_raw_type_eq;

macro_rules! assert_raw_type_not_eq {
        ($X:ty, $Y:ty) => {{
            const _: () = <crate::r#type::assertions::internal::AssertRawTypeNotEq<$X, $Y> as crate::r#type::assertions::internal::AssertRawTypeNotEqTrait>::check_not_equal();
        }};
    }
pub(crate) use assert_raw_type_not_eq;

pub mod internal {
    use std::marker::PhantomData;

    pub struct AssertRawTypeEq<X, Y> {
        x: PhantomData<X>,
        y: PhantomData<Y>,
    }

    #[const_trait]
    pub trait AssertRawTypeEqTrait {
        fn check_equal();
    }

    impl<X, Y> const AssertRawTypeEqTrait for AssertRawTypeEq<X, Y> {
        default fn check_equal() {
            panic!("Found different types");
        }
    }

    impl<X> const AssertRawTypeEqTrait for AssertRawTypeEq<X, X> {
        fn check_equal() {}
    }

    pub struct AssertRawTypeNotEq<X, Y> {
        x: PhantomData<X>,
        y: PhantomData<Y>,
    }

    #[const_trait]
    pub trait AssertRawTypeNotEqTrait {
        fn check_not_equal();
    }

    impl<X, Y> const AssertRawTypeNotEqTrait for AssertRawTypeNotEq<X, Y> {
        default fn check_not_equal() {
        }
    }

    impl<X> const AssertRawTypeNotEqTrait for AssertRawTypeNotEq<X, X> {
        fn check_not_equal() {
            panic!("Found equal types");
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::bool::assertions::*;
    use crate::r#type::assertions::*;

    #[test]
    fn assert_raw_type_eq() {
        assert_raw_type_eq!(i32, i32);
    }

    #[test]
    fn assert_type_eq() {
        assert_type_eq!(WrapType<i32>, WrapType<i32>);
    }

    #[test]
    fn assert_type_equals() {
        assert_true!(Equals<Type, WrapType<i32>, WrapType<i32>>);
        assert_false!(Equals<Type, WrapType<i32>, WrapType<i64>>);
    }

    #[test]
    fn assert_raw_type_not_eq() {
        assert_raw_type_not_eq!(i32, f64);
    }

    #[test]
    fn assert_type_not_eq() {
        assert_type_not_eq!(WrapType<i32>, WrapType<f64>);
    }
}