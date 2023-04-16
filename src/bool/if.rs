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

use std::marker::PhantomData;
use internal::*;

pub struct If<ResultK: Kind, B: Expr<Bool>, Then: Expr<ResultK>, Else: Expr<ResultK>> {
    result_tag: PhantomData<ResultK>,
    b: PhantomData<B>,
    then: PhantomData<Then>,
    else_data: PhantomData<Else>,
}

impl<ResultK: Kind, B: Expr<Bool>, Then: Expr<ResultK>, Else: Expr<ResultK>> Expr<ResultK> for If<ResultK, B, Then, Else> {
    type Eval = <<AsBool<B> as BoolTrait>::Cond<ResultK, Then, Else> as Expr<ResultK>>::Eval;
}

pub struct IfResult<ResultK: Kind, B: Expr<Result<Bool>>, Then: Expr<Result<ResultK>>, Else: Expr<Result<ResultK>>> {
    result_tag: PhantomData<ResultK>,
    b: PhantomData<B>,
    then: PhantomData<Then>,
    else_data: PhantomData<Else>,
}

impl<ResultK: Kind, B: Expr<Result<Bool>>, Then: Expr<Result<ResultK>>, Else: Expr<Result<ResultK>>> Expr<Result<ResultK>> for IfResult<ResultK, B, Then, Else> {
    type Eval = <AndThen<Bool, ResultK, B, IfResultImpl<ResultK, Then, Else>> as Expr<Result<ResultK>>>::Eval;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct IfResultImpl<ResultK: Kind, Then: Expr<Result<ResultK>>, Else: Expr<Result<ResultK>>> {
        result_tag: PhantomData<ResultK>,
        then: PhantomData<Then>,
        else_data: PhantomData<Else>,
    }

    impl<ResultK: Kind, Then: Expr<Result<ResultK>>, Else: Expr<Result<ResultK>>> Functor1<Bool, Result<ResultK>> for IfResultImpl<ResultK, Then, Else> {
        type Apply<B: Expr<Bool>> = <AsBool<B> as BoolTrait>::Cond<Result<ResultK>, Then, Else>;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn if_function() {
        assert_type_eq!(If<Type, True, WrapType<u32>, WrapType<f64>>, WrapType<u32>);
        assert_type_eq!(If<Type, False, WrapType<u32>, WrapType<f64>>, WrapType<f64>);

        assert_true!(If<Bool, False, False, True>);
        assert_false!(If<Bool, True, False, True>);
    }

    #[test]
    fn if_result_function() {
        assert_type_result_eq!(IfResult<Type, Ok<Bool, True>, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Ok<Type, WrapType<u32>>);
        assert_type_result_eq!(IfResult<Type, Ok<Bool, False>, Ok<Type, WrapType<u32>>, Ok<Type, WrapType<f64>>>, Ok<Type, WrapType<f64>>);

        assert_true_result!(IfResult<Bool, Ok<Bool, False>, Ok<Bool, False>, Ok<Bool, True>>);
        assert_false_result!(IfResult<Bool, Ok<Bool, True>, Ok<Bool, False>, Ok<Bool, True>>);
    }
}
