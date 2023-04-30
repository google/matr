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

// E.g. meta_set!(<USize>{Zero, Inc<Zero>, Inc<Inc<Inc<Zero>>>}) expands to:
// AddToSet<USize, Zero, AddToSet<USize, Inc<Zero>, AddToSet<USize, Inc<Inc<Inc<Zero>>>, EmptySet<USize>>>>
// When using this for Type, prefer using type_set! which is simpler to use (but this is more general)
#[macro_export]
macro_rules! meta_set {
    (<$T: ty>{}) => {
        $crate::EmptySet<$T>
    };
    (<$T: ty>{$Head:ty $(, $Tail:ty)*}) => {
        $crate::AddToSet<$T, $Head, $crate::meta_set!(<$T>{$($Tail),*})>
    };
}
pub use meta_set;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn empty_set() {
        meta_assert_eq!(Set<Type>, meta_set!(<Type>{}), EmptySet<Type>);
    }

    #[test]
    fn set() {
        meta_assert_eq!(Set<Type>, meta_set!(<Type>{WrapType<i32>, WrapType<u32>}), AddToSet<Type, WrapType<i32>, AddToSet<Type, WrapType<u32>, EmptySet<Type>>>);
    }
}
