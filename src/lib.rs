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

mod bool;
mod const_fn;
mod result;
mod r#type;
mod equals;
mod expr;
mod functors;
mod kind;
mod value;
mod usize;
mod runtime_fn;
mod kind_with_default;
mod list;
mod set;
mod map;
mod pair;
mod option;

pub use crate::bool::*;
pub use const_fn::*;
pub use runtime_fn::*;
pub use result::*;
pub use r#type::*;
pub use equals::*;
pub use expr::*;
pub use functors::*;
pub use kind::*;
pub use expr::*;
pub use list::*;
pub use set::*;
pub use map::*;
pub use pair::*;
pub use value::*;
pub use kind_with_default::*;
pub use crate::usize::*;
pub use option::*;
