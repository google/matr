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

meta!{
    pub type If<
        ResultK: Kind,
        B: Expr<Bool>,
        Then: Expr<ResultK>,
        Else: Expr<ResultK>
    >: Expr<ResultK> =
        <AsBool<B> as BoolTrait>::Cond<ResultK, Then, Else>;

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

    #[test]
    fn if_function() {
        meta_assert_eq!(Type, If<Type, True, WrapType<u32>, WrapType<f64>>, WrapType<u32>);
        meta_assert_eq!(Type, If<Type, False, WrapType<u32>, WrapType<f64>>, WrapType<f64>);

        meta_assert_eq!(Bool, If<Bool, False, False, True>, True);
        meta_assert_eq!(Bool, If<Bool, True, False, True>, False);
    }

    #[test]
    fn if_result_function() {
        meta_assert_eq!(Result<Type>, IfResult<Type, Ok<Bool, True>, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Ok<Type, WrapType<u32>>);
        meta_assert_eq!(Result<Type>, IfResult<Type, Ok<Bool, False>, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Ok<Type, WrapType<f64>>);

        meta_assert_eq!(Result<Bool>, IfResult<Bool, Ok<Bool, False>, Ok<Bool, False>, Ok<Bool, True>>, Ok<Bool, True>);
        meta_assert_eq!(Result<Bool>, IfResult<Bool, Ok<Bool, True>, Ok<Bool, False>, Ok<Bool, True>>, Ok<Bool, False>);
    }
}
