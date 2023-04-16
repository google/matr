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

// E.g. list!(<USize>[Zero, Inc<Zero>, Inc<Inc<Inc<Zero>>>]) expands to:
// Cons<USize, Zero, Cons<USize, Inc<Zero>, Cons<USize, Inc<Inc<Inc<Zero>>>, EmptyList<USize>>>>
// When using this for Type, prefer using type_list! which is simpler to use (but this is more general)
#[macro_export]
macro_rules! list {
    (<$T: ty>[]) => {
        $crate::EmptyList<$T>
    };
    (<$T: ty>[$X:ty $( ,$Y:ty )*]) => {
        $crate::Cons<$T, $X, $crate::list!(<$T>[$($Y),*])>
    };
}
pub use list;
