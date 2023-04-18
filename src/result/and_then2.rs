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
    pub type AndThen2<
        K1: Kind,
        K2: Kind, 
        ResultK: Kind,
        V1: Expr<Result<K1>>,
        V2: Expr<Result<K2>>, 
        F: Functor2<K1, K2, Result<ResultK>>
    >: Expr<Result<ResultK>> =
        AndThen<K1, ResultK, V1, AndThen2ImplStage1<K1, K2, ResultK, V2, F>>;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct AndThen2ImplStage1<
            K1: Kind, 
            K2: Kind, 
            ResultK: Kind, 
            Expr2: Expr<Result<K2>>,
            F: Functor2<K1, K2, Result<ResultK>>
        >: Functor1<K1, Result<ResultK>> {
            type Apply<V1: Expr<K1>> = AndThen<K2, ResultK, Expr2, AndThen2ImplStage2<K1, K2, ResultK, V1, F>>;
        }
        
        pub struct AndThen2ImplStage2<
            K1: Kind, 
            K2: Kind,
            ResultK: Kind, 
            V1: Expr<K1>,
            F: Functor2<K1, K2, Result<ResultK>>
        >: Functor1<K2, Result<ResultK>> {
            type Apply<V2: Expr<K2>> = F::Apply<V1, V2>;
        }
    }
}
