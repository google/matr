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
macro_rules! assert_type_set_eq {
($X:ty, $Y:ty) => {{
        type Common = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetIntersection<$crate::Type, $X, $Y>>>> as $crate::GetTypeTrait>::Get;
        type XOnly = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetDifference<$crate::Type, $X, $Y>>>> as $crate::GetTypeTrait>::Get;
        type YOnly = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetDifference<$crate::Type, $Y, $X>>>> as $crate::GetTypeTrait>::Get;
        // Including Common to give more context when the assertion fails.
        $crate::assert_raw_type_eq!(
            (Common, XOnly, YOnly),
            (Common, (), ()));
    }};
}
pub use assert_type_set_eq;

#[macro_export]
macro_rules! assert_type_set_not_eq {
($X:ty, $Y:ty) => {{
        type Common = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetIntersection<$crate::Type, $X, $Y>>>> as $crate::GetTypeTrait>::Get;
        type XOnly = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetDifference<$crate::Type, $X, $Y>>>> as $crate::GetTypeTrait>::Get;
        type YOnly = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetDifference<$crate::Type, $Y, $X>>>> as $crate::GetTypeTrait>::Get;
        // Including Common to give more context when the assertion fails.
        $crate::assert_raw_type_not_eq!(
            (Common, XOnly, YOnly),
            (Common, (), ()));
    }};
}
pub use assert_type_set_not_eq;
