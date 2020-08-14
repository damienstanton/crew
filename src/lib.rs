// Copyright 2020 Damien Stanton
// Portions of this source are derivative of https://crates.io/crates/waitgroup.
// That work is copyright @laizy, also distributed under this license.
// See https://github.com/laizy/waitgroup-rs for details.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A "batteries-included" wokerpool/job-runner using Rust futures
mod crew;
mod workgroup;

pub use crate::crew::{assemble_crew, CrewError};
