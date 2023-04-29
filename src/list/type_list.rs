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

// E.g. type_list![u32, i64, usize] expands to:
// Cons<Type, WrapType<u32>, Cons<Type, WrapType<i64>, Cons<Type, WrapType<usize>, EmptyList<Type>>>>
#[macro_export]
macro_rules! type_list {
    () => {
        $crate::EmptyList<$crate::Type>
    };
    ($X:ty $( ,$Y:ty )*) => {
        $crate::Cons<$crate::Type, $crate::WrapType<$X>, type_list![$($Y),*]>
    };
}
pub use type_list;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn empty_list() {
        meta_assert_eq!(List<Type>, type_list![], EmptyList<Type>);
    }

    #[test]
    fn list() {
        meta_assert_eq!(List<Type>, type_list![i32, u32], Cons<Type, WrapType<i32>, Cons<Type, WrapType<u32>, EmptyList<Type>>>);
    }
}
