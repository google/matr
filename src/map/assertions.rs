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

macro_rules! assert_type_map_eq {
($X:ty, $Y:ty) => {{
    // TODO
        type Common = <GetType<ToTypeNestedTuple<SetToList<Type, SetIntersection<Type, $X, $Y>>>> as GetTypeTrait>::Get;
        type XOnly = <GetType<ToTypeNestedTuple<SetToList<Type, SetDifference<Type, $X, $Y>>>> as GetTypeTrait>::Get;
        type YOnly = <GetType<ToTypeNestedTuple<SetToList<Type, SetDifference<Type, $Y, $X>>>> as GetTypeTrait>::Get;
        // Including Common to give more context when the assertion fails.
        crate::r#type::assertions::assert_raw_type_eq!(
            (Common, XOnly, YOnly),
            (Common, (), ()));
    }};
}
pub(crate) use assert_type_map_eq;

macro_rules! assert_type_map_not_eq {
($X:ty, $Y:ty) => {{
    // TODO
        type Common = <GetType<ToTypeNestedTuple<SetToList<Type, SetIntersection<Type, $X, $Y>>>> as GetTypeTrait>::Get;
        type XOnly = <GetType<ToTypeNestedTuple<SetToList<Type, SetDifference<Type, $X, $Y>>>> as GetTypeTrait>::Get;
        type YOnly = <GetType<ToTypeNestedTuple<SetToList<Type, SetDifference<Type, $Y, $X>>>> as GetTypeTrait>::Get;
        // Including Common to give more context when the assertion fails.
        crate::r#type::assertions::assert_raw_type_not_eq!(
            (Common, XOnly, YOnly),
            (Common, (), ()));
    }};
}
pub(crate) use assert_type_map_not_eq;
