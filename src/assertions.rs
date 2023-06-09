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

#[macro_export]
macro_rules! meta_assert_eq {
    ($K:ty, $X:ty, $Y:ty) => {{
        const EQ: bool = $crate::bool::to_bool::<$crate::Equals<$K, $X, $Y>>();
        const _: () = $crate::assertions::internal::check_equal::<
            $crate::expr_wrapper::UnwrapExpr<$K, $crate::DebugForm<$K, $X>>,
            $crate::expr_wrapper::UnwrapExpr<$K, $crate::DebugForm<$K, $Y>>
        >(EQ);
    }};
}
pub use meta_assert_eq;

#[macro_export]
macro_rules! meta_assert_not_eq {
    ($K:ty, $X:ty, $Y:ty) => {{
        const EQ: bool = $crate::bool::to_bool::<$crate::Equals<$K, $X, $Y>>();
        const _: () = $crate::assertions::internal::check_not_equal::<
            $crate::expr_wrapper::UnwrapExpr<$K, $crate::DebugForm<$K, $X>>,
            $crate::expr_wrapper::UnwrapExpr<$K, $crate::DebugForm<$K, $Y>>
        >(EQ);
    }};
}
pub use meta_assert_not_eq;

pub mod internal {
    pub const fn check_equal<XDebug, YDebug>(eq: bool) {
        if !eq {
            panic!("Found different types");
        }
    }

    pub const fn check_not_equal<XDebug, YDebug>(eq: bool) {
        if eq {
            panic!("Found equal types");
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::r#type::*;
    use crate::bool::*;

    #[test]
    fn assert_type_eq() {
        meta_assert_eq!(Type, WrapType<i32>, WrapType<i32>);
    }

    #[test]
    fn assert_bool_equals() {
        meta_assert_eq!(Bool, Equals<Type, WrapType<i32>, WrapType<i32>>, True);
        meta_assert_eq!(Bool, Equals<Type, WrapType<i32>, WrapType<i64>>, False);
    }

    #[test]
    fn assert_type_not_eq() {
        meta_assert_not_eq!(Type, WrapType<i32>, WrapType<f64>);
    }
}
