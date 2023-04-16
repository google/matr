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

pub const fn result_to_usize<N: Expr<Result<USize>>>() -> std::result::Result<usize, &'static str> {
    return call_const_fn::<std::result::Result<usize, &'static str>, (), 
        OrValue<
            ConstFn<std::result::Result<usize, &'static str>, ()>,
            AndThen<
                USize,
                ConstFn<std::result::Result<usize, &'static str>, ()>,
                N,
                ToUSizeFunctor>,
            ToUSizeError
        >
    >(());
}

pub const fn to_usize<N: Expr<USize>>() -> usize {
    return call_const_fn::<usize, (), VisitUSize<ConstFn<usize, ()>, N, ToUSizeVisitor>>(());
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ToUSizeError {}

    impl Expr<ConstFn<std::result::Result<usize, &'static str>, ()>> for ToUSizeError {
        type Eval = ToUSizeErrorValue;
    }

    pub struct ToUSizeErrorValue {}

    impl ConstFnValue<std::result::Result<usize, &'static str>, ()> for ToUSizeErrorValue {
        type Impl = ToUSizeErrorImpl;
    }

    pub struct ToUSizeErrorImpl {}

    impl const ConstFnTrait<std::result::Result<usize, &'static str>, ()> for ToUSizeErrorImpl {
        fn apply(_: ()) -> std::result::Result<usize, &'static str> {
            panic!("result_to_usize called on an error")
        }
    }

    pub struct ToUSizeFunctor {}

    impl Functor1<USize, Result<ConstFn<std::result::Result<usize, &'static str>, ()>>> for ToUSizeFunctor {
        type Apply<N: Expr<USize>> = Ok<ConstFn<std::result::Result<usize, &'static str>, ()>, ToResultConstFn<usize, (), Ok<ConstFn<usize, ()>, VisitUSize<ConstFn<usize, ()>, N, ToUSizeVisitor>>>>;
    }

    pub struct ToUSizeVisitor {}

    impl USizeVisitor<ConstFn<usize, ()>> for ToUSizeVisitor {
        type VisitZero = ZeroValueConstFn;
        type VisitIncrement<N: Expr<USize>> = IncValueConstFn<<AsUSize<N> as USizeTrait>::Visit<ConstFn<usize, ()>, ToUSizeVisitor>>;
    }

    pub struct ZeroValueConstFn {}

    impl Expr<ConstFn<usize, ()>> for ZeroValueConstFn {
        type Eval = ZeroValueConstFnValue;
    }

    pub struct ZeroValueConstFnValue {}

    impl ConstFnValue<usize, ()> for ZeroValueConstFnValue {
        type Impl = ZeroValueConstFnImpl;
    }

    pub struct ZeroValueConstFnImpl {}

    impl const ConstFnTrait<usize, ()> for ZeroValueConstFnImpl {
        fn apply(_: ()) -> usize {
            return 0;
        }
    }

    pub struct IncValueConstFn<F: Expr<ConstFn<usize, ()>>> {
        f: PhantomData<F>,
    }

    impl<F: Expr<ConstFn<usize, ()>>> Expr<ConstFn<usize, ()>> for IncValueConstFn<F> {
        type Eval = IncValueConstFnValue<F>;
    }

    pub struct IncValueConstFnValue<F: Expr<ConstFn<usize, ()>>> {
        f: PhantomData<F>,
    }

    impl<F: Expr<ConstFn<usize, ()>>> ConstFnValue<usize, ()> for IncValueConstFnValue<F> {
        type Impl = IncValueConstFnImpl<F>;
    }

    pub struct IncValueConstFnImpl<F: Expr<ConstFn<usize, ()>>> {
        f: PhantomData<F>,
    }

    impl<F: Expr<ConstFn<usize, ()>>> const ConstFnTrait<usize, ()> for IncValueConstFnImpl<F> {
        fn apply(_: ()) -> usize {
            return 1 + call_const_fn::<usize, (), F>(());
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn result_to_usize_test() {
        const N: std::result::Result<usize, &'static str> = result_to_usize::<Ok<USize, Increment<Increment<Increment<Zero>>>>>();
        assert_eq!(N, Ok(3));
    }

    #[test]
    fn to_usize_test() {
        const N: usize = to_usize::<Increment<Increment<Increment<Zero>>>>();
        assert_eq!(N, 3);
    }
}
