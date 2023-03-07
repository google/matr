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

macro_rules! assert_false {
    ($value:ty) => {{
        use crate::WrapType;
        use crate::If;
        crate::r#type::assertions::assert_type_eq!(If<Type, $value, WrapType<u32>, WrapType<f64>>, WrapType<f64>);
    }};
}

macro_rules! assert_false_result {
    ($value:ty) => {{
        use crate::WrapType;
        use crate::IfResult;
        crate::r#type::assertions::assert_type_result_eq!(IfResult<Type, $value, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Ok<Type, WrapType<f64>>);
    }};
}

macro_rules! assert_true {
    ($value:ty) => {{
        use crate::WrapType;
        use crate::If;
        crate::r#type::assertions::assert_type_eq!(If<Type, $value, WrapType<u32>, WrapType<f64>>, WrapType<u32>);
    }};
}

macro_rules! assert_true_result {
    ($value:ty) => {{
        use crate::WrapType;
        use crate::IfResult;
        crate::r#type::assertions::assert_type_result_eq!(IfResult<Type, $value, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Ok<Type, WrapType<u32>>);
    }};
}

pub(crate) use assert_false;
pub(crate) use assert_false_result;
pub(crate) use assert_true;
pub(crate) use assert_true_result;
