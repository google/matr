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

use internal::*;

// Use this at top-level when evaluating an Expr<Result<Type>>, in conjunction with
// check_no_error (for the same expression).
//
// Intended to be used with:
// E: Expr<Result<Type>>
pub type UnwrapTypeResult<E> = <UnwrapTypeResultHelper<E> as UnwrapTypeResultTrait>::Get;

mod internal {
    pub use super::super::internal::*;
    use crate::r#type::*;

    pub const trait UnwrapTypeResultTrait {
        type Get;
    }

    meta!{
        pub struct UnwrapTypeResultHelper<E: Expr<Result<Type>>>: UnwrapTypeResultTrait {
            type Get = UnwrapType<ResultOrValue<Type, E, WrapType<()>>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::r#type::*;

    struct MyError {}

    #[test]
    fn unwrap_type_result() {
        meta_assert_eq!(Type, WrapType<UnwrapTypeResult<Err<Type, MyError>>>, WrapType<()>);
        meta_assert_eq!(Type, WrapType<UnwrapTypeResult<Ok<Type, WrapType<i32>>>>, WrapType<i32>);
    }
}
