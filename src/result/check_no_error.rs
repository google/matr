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

// Use this at top-level when evaluating an Expr<Result<...>>, in conjunction with
// GetTypeResult or GetConstFn (for the same expression).
pub const fn check_no_error<K: Kind, V: Expr<Result<K>>>() {
    call_const_fn::<(), (), <AsResult<K, V> as ResultTrait<K>>::Visit<ConstFn<(), ()>, CheckNoError<K>>>(());
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct PanickingConstFnImpl<E>: const ConstFnTrait<(), ()> {
            fn apply(_: ()) -> () {
                panic!("Error found in check_no_error");
            }
        }
        
        pub struct PanickingConstFnValue<E>: ConstFnValue<(), ()> {
            type Impl = PanickingConstFnImpl<E>;
        }
        
        pub struct PanickingConstFn<E>: Expr<ConstFn<(), ()>> {
            type Eval = PanickingConstFnValue<E>;
        }
        
        pub struct CheckNoError<K: Kind>: ResultVisitor<K, ConstFn<(), ()>> {
            type VisitOk<V: Expr<K>> = NoOpConstFn;
            type VisitErr<E> = PanickingConstFn<E>;
        }
    }
}
