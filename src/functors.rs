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

use crate::*;

pub trait Functor0<ResultK: Kind> {
    type Apply: Expr<ResultK>;
}

pub trait Functor1<K1: Kind, ResultK: Kind> {
    type Apply<X: Expr<K1>>: Expr<ResultK>;
}

pub trait Functor2<K1: Kind, K2: Kind, ResultK: Kind> {
    type Apply<X: Expr<K1>, Y: Expr<K2>>: Expr<ResultK>;
}
