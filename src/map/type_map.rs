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

// E.g. type_map!{i32: u32, i64: u64} expands to:
// Put<Type, Type, WrapType<i32>, WrapType<u32>, Put<Type, Type, WrapType<i64>, WrapType<u64>, EmptyMap<Type, Type>>>
#[macro_export]
macro_rules! type_map {
    {} => {
        $crate::EmptyMap<$crate::Type, $crate::Type>
    };
    {$K1:ty : $V1:ty $( ,$Ks:ty : $Vs: ty )*} => {
        $crate::Put<$crate::Type, $crate::Type, $crate::WrapType<$K1>, $crate::WrapType<$V1>, $crate::type_map!{$($Ks: $Vs),*}>
    };
    {$K1:ty : $V1:ty $( ,$Ks:ty : $Vs: ty )* ,} => {
        $crate::Put<$crate::Type, $crate::Type, $crate::WrapType<$K1>, $crate::WrapType<$V1>, $crate::type_map!{$($Ks: $Vs),*}>
    };
}
pub use type_map;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn empty_map() {
        meta_assert_eq!(
            Map<Type, Type>,
            type_map!{},
            EmptyMap<Type, Type>);
    }

    #[test]
    fn map() {
        meta_assert_eq!(
            Map<Type, Type>,
            type_map!{i32: u32},
            Put<Type, Type, WrapType<i32>, WrapType<u32>, EmptyMap<Type, Type>>);
    }
}
