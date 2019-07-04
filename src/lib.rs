// Copyright 2019 Mats Kindahl
//
// Licensed under the Apache License, Version 2.0 (the "License"); you
// may not use this file except in compliance with the License.  You
// may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.  See the License for the specific language governing
// permissions and limitations under the License.

//! Chatter is a simple to use distributed monitoring system.
//!
//! Chatter is a distributed monitoring system that is deployed as a
//! set of collaborating Chatter agents. Each agent will run on a
//! machine and monitor the devices on that machine. The status of the
//! devices on each machine is then disseminated to the other chatter
//! agents using a gossipping protocol.
//!
//! Chatter is implemented using Tokio.

extern crate chrono;
extern crate serde_cbor;
extern crate uuid;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

pub mod devices;
pub mod error;
pub mod gossip;
pub mod state;
pub mod view;
