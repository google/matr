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
macro_rules! assert_false {
    ($value:ty) => {
        $crate::assert_type_eq!(
            $crate::If<$crate::Type, $value,
                $crate::WrapType<u32>,
                $crate::WrapType<f64>>,
            $crate::WrapType<f64>);
    };
}

#[macro_export]
macro_rules! assert_false_result {
    ($value:ty) => {
        $crate::assert_type_result_eq!(
            $crate::IfResult<$crate::Type, $value,
                $crate::Ok<$crate::Type, $crate::WrapType<u32>>,
                $crate::Ok<$crate::Type, $crate::WrapType<f64>>>,
            $crate::Ok<$crate::Type, $crate::WrapType<f64>>);
    };
}

#[macro_export]
macro_rules! assert_true {
    ($value:ty) => {
        $crate::assert_type_eq!(
            $crate::If<$crate::Type, $value,
                $crate::WrapType<u32>,
                $crate::WrapType<f64>>,
            $crate::WrapType<u32>);
    };
}

#[macro_export]
macro_rules! assert_true_result {
    ($value:ty) => {
        crate::assert_type_result_eq!(
            $crate::IfResult<$crate::Type, $value,
                $crate::Ok<$crate::Type, $crate::WrapType<u32>>,
                $crate::Ok<$crate::Type, $crate::WrapType<f64>>>,
            $crate::Ok<$crate::Type, $crate::WrapType<u32>>);
    };
}

pub use assert_false;
pub use assert_false_result;
pub use assert_true;
pub use assert_true_result;
