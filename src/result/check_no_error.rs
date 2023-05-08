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
use crate::const_fn::*;

// Use this at top-level when evaluating an Expr<Result<...>>, in conjunction with
// GetTypeResult or GetConstFn (for the same expression).
pub const fn check_no_error<K: Kind, V: Expr<Result<K>>>() {
    call_const_fn::<(), (), <AsResult<K, V> as ResultTrait<K>>::Visit<ConstFn<(), ()>, CheckNoError<K>>>(());
}

mod internal {
    pub use super::super::internal::*;
    use crate::const_fn::*;

    meta!{
        pub struct PanickingConstFnImpl<E>: const ConstFnTrait<(), ()> {
            fn apply(_: ()) -> () {
                panic!("Error found in check_no_error");
            }
        }

        pub struct CheckNoError<K: Kind>: ResultVisitor<K, ConstFn<(), ()>> {
            type VisitOk<V: Expr<K>> = NoOpConstFn;
            type VisitErr<E> = WrapConstFn<(), (), PanickingConstFnImpl<E>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::super::*;
    use crate::r#type::*;

    struct MyError {}

    #[test]
    fn check_no_error_test() {
        // This does not compile (as expected).
        // const _: () = check_no_error::<Type, Err<Type, MyError>>();

        const _: () = check_no_error::<Type, Ok<Type, WrapType<i32>>>();
    }
}
