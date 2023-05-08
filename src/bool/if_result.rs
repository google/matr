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
use crate::result::*;

meta!{
    pub type IfResult<
        ResultK: Kind,
        B: Expr<Result<Bool>>,
        Then: Expr<Result<ResultK>>,
        Else: Expr<Result<ResultK>>
    >: Expr<Result<ResultK>> =
        AndThen<Bool, ResultK, B, IfResultImpl<ResultK, Then, Else>>;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use super::super::internal::*;
    use crate::result::*;

    meta!{
        pub struct IfResultImpl<
            ResultK: Kind,
            Then: Expr<Result<ResultK>>,
            Else: Expr<Result<ResultK>>
        >: Functor1<Bool, Result<ResultK>> {
            type Apply<B: Expr<Bool>> = <AsBool<B> as BoolTrait>::Cond<Result<ResultK>, Then, Else>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::r#type::*;
    use crate::result::*;
    use crate::usize::*;

    #[test]
    fn ok() {
        meta_assert_eq!(Result<Type>, IfResult<Type, Ok<Bool, True>, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Ok<Type, WrapType<u32>>);
        meta_assert_eq!(Result<Type>, IfResult<Type, Ok<Bool, False>, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Ok<Type, WrapType<f64>>);
    }

    #[test]
    fn err_condition() {
        struct E {}

        meta_assert_eq!(Result<Type>, IfResult<Type, Err<Bool, E>, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Err<Type, E>);
        meta_assert_eq!(Result<Type>, IfResult<Type, Err<Bool, E>, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Err<Type, E>);
    }

    #[test]
    fn err_in_non_evaluated_branch() {
        struct E {}

        meta_assert_eq!(Result<Bool>, IfResult<Bool, Ok<Bool, True>, Ok<Bool, False>, Ok<Bool, LongRecursion<OneBillion>>>, Ok<Bool, False>);
        meta_assert_eq!(Result<Bool>, IfResult<Bool, Ok<Bool, False>, Ok<Bool, LongRecursion<OneBillion>>, Ok<Bool, False>>, Ok<Bool, False>);
    }

    #[test]
    fn err_in_evaluated_branch() {
        struct E {}

        meta_assert_eq!(Result<USize>, IfResult<USize, Ok<Bool, True>, Err<USize, E>, Ok<USize, Zero>>, Err<USize, E>);
        meta_assert_eq!(Result<USize>, IfResult<USize, Ok<Bool, False>, Ok<USize, Zero>, Err<USize, E>>, Err<USize, E>);
    }

    #[test]
    fn all_err() {
        struct CondError {}
        struct ThenError {}
        struct ElseError {}

        meta_assert_eq!(Result<USize>, IfResult<USize, Err<Bool, CondError>, Err<USize, ThenError>, Err<USize, ElseError>>, Err<USize, CondError>);
    }
}
