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
    pub struct Some<
        K: Kind,
        X: Expr<K>
    >: Expr<Option<K>> {
        type Eval = SomeValue<K, X>;
    }
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct SomeValue<
            K: Kind,
            X: Expr<K>
        >: OptionValue<K> {
            type Impl = SomeImpl<K, X>;
        }

        pub struct SomeImpl<
            K: Kind,
            X: Expr<K>
        >: OptionTrait<K> {
            type Visit<ResultK: Kind, Visitor: OptionVisitor<K, ResultK>> = Visitor::VisitSome<X>;
        }
    }
}
