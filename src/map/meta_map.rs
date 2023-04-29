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

// E.g. map!(<USize, USize>{Zero: Inc<Zero>, Inc<Inc<Zero>>, Inc<Inc<Inc<Zero>>>}) expands to:
// Put<USize, USize, Zero, Inc<Zero>, Put<USize, USize, Inc<Inc<Zero>>, Inc<Inc<Inc<Zero>>>, EmptyMap<USize, USize>>>
// When using this for Type, prefer using type_map! which is simpler to use (but this is more general)
#[macro_export]
macro_rules! meta_map {
    (<$K: ty, $V: ty>{}) => {
        $crate::EmptyMap<$K, $V>
    };
    (<$K: ty, $V: ty>{$K1:ty : $V1:ty $( ,$Ks:ty : $Vs: ty )*}) => {
        $crate::Put<$K, $V, $K1, $V1, $crate::meta_map!(<$K, $V>{$($Ks: $Vs),*})>
    };
}
pub use meta_map;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn empty_map() {
        meta_assert_eq!(
            Map<Type, Type>,
            meta_map!(<Type, Type>{}),
            EmptyMap<Type, Type>);
    }

    #[test]
    fn map() {
        meta_assert_eq!(
            Map<Type, Type>,
            meta_map!(<Type, Type>{WrapType<i32>: WrapType<u32>}), Put<Type, Type, WrapType<i32>, WrapType<u32>, EmptyMap<Type, Type>>);
    }
}
