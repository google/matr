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

#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(const_trait_impl)]

pub mod assertions;
pub mod meta;

mod equals;
mod expr;
mod functors;
mod kind;
mod kind_with_default;
mod kind_with_debug_form;
mod value;

pub use equals::*;
pub use expr::*;
pub use functors::*;
pub use kind::*;
pub use kind_with_default::*;
pub use kind_with_debug_form::*;
pub use value::*;

pub mod bool;
pub mod const_fn;
pub mod result;
pub mod r#type;
pub mod usize;
pub mod runtime_fn;
pub mod list;
pub mod set;
pub mod map;
pub mod pair;
pub mod option;
pub mod expr_wrapper;
pub mod tuple_based_list;
pub mod tuple_based_type_list;
pub mod tuple_based_type_pair_list;
