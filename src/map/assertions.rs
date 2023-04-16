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
macro_rules! assert_type_map_eq {
($X:ty, $Y:ty) => {{
        type CommonKeysWithSameValue      = <$crate::GetType<$crate::ToTypePairNestedTuple<$crate::SetToList<$crate::Pair<$crate::Type, $crate::Type>, $crate::SetIntersection<$crate::Pair<$crate::Type, $crate::Type>, $crate::MapEntrySet<$crate::Type, $crate::Type, $X>, $crate::MapEntrySet<$crate::Type, $crate::Type, $Y>>>>> as $crate::GetTypeTrait>::Get;
        type CommonKeysWithDifferentValue = <$crate::GetType<$crate::ToTypeTripleNestedTuple<$crate::MapToList<$crate::Type, $crate::Pair<$crate::Type, $crate::Type>, $crate::MapCommonKeysWithDifferentValue<$crate::Type, $crate::Type, $X, $Y>>>> as $crate::GetTypeTrait>::Get;
        type KeysInXOnly = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetDifference<$crate::Type, $crate::MapKeySet<$crate::Type, $crate::Type, $X>, $crate::MapKeySet<$crate::Type, $crate::Type, $Y>>>>> as $crate::GetTypeTrait>::Get;
        type KeysInYOnly = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetDifference<$crate::Type, $crate::MapKeySet<$crate::Type, $crate::Type, $Y>, $crate::MapKeySet<$crate::Type, $crate::Type, $X>>>>> as $crate::GetTypeTrait>::Get;
        // Including CommonKeysWithSameValue to give more context when the assertion fails.
        $crate::assert_raw_type_eq!(
            (CommonKeysWithSameValue, CommonKeysWithDifferentValue, KeysInXOnly, KeysInYOnly),
            (CommonKeysWithSameValue, (), (), ()));
    }};
}
pub use assert_type_map_eq;

#[macro_export]
macro_rules! assert_type_map_not_eq {
($X:ty, $Y:ty) => {{
        type CommonKeysWithSameValue      = <$crate::GetType<$crate::ToTypePairNestedTuple<$crate::SetToList<$crate::Pair<$crate::Type, $crate::Type>, $crate::SetIntersection<$crate::Pair<$crate::Type, $crate::Type>, $crate::MapEntrySet<$crate::Type, $crate::Type, $X>, $crate::MapEntrySet<$crate::Type, $crate::Type, $Y>>>>> as $crate::GetTypeTrait>::Get;
        type CommonKeysWithDifferentValue = <$crate::GetType<$crate::ToTypeTripleNestedTuple<$crate::MapToList<$crate::Type, $crate::Pair<$crate::Type, $crate::Type>, $crate::MapCommonKeysWithDifferentValue<$crate::Type, $crate::Type, $X, $Y>>>> as $crate::GetTypeTrait>::Get;
        type KeysInXOnly = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetDifference<$crate::Type, $crate::MapKeySet<$crate::Type, $crate::Type, $X>, $crate::MapKeySet<$crate::Type, $crate::Type, $Y>>>>> as $crate::GetTypeTrait>::Get;
        type KeysInYOnly = <$crate::GetType<$crate::ToTypeNestedTuple<$crate::SetToList<$crate::Type, $crate::SetDifference<$crate::Type, $crate::MapKeySet<$crate::Type, $crate::Type, $Y>, $crate::MapKeySet<$crate::Type, $crate::Type, $X>>>>> as $crate::GetTypeTrait>::Get;
        // Including CommonKeysWithSameValue to give more context when the assertion fails.
        $crate::assert_raw_type_not_eq!(
            (CommonKeysWithSameValue, CommonKeysWithDifferentValue, KeysInXOnly, KeysInYOnly),
            (CommonKeysWithSameValue, (), (), ()));
    }};
}
pub use assert_type_map_not_eq;
