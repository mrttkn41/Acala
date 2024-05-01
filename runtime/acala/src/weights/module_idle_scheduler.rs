// This file is part of Acala.

// Copyright (C) 2020-2024 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_idle_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 35.0.1
//! DATE: 2024-04-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-41-141`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_idle_scheduler.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_idle_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `IdleScheduler::PreviousRelayBlockNumber` (r:0 w:1)
	// Proof: `IdleScheduler::PreviousRelayBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `681`
		//  Estimated: `2166`
		// Minimum execution time: 4_115 nanoseconds.
		Weight::from_parts(4_350_000, 2166)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `IdleScheduler::PreviousRelayBlockNumber` (r:1 w:0)
	// Proof: `IdleScheduler::PreviousRelayBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_idle_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `757`
		//  Estimated: `2242`
		// Minimum execution time: 5_892 nanoseconds.
		Weight::from_parts(6_114_000, 2242)
			.saturating_add(T::DbWeight::get().reads(3))
	}
	// Storage: `IdleScheduler::Tasks` (r:0 w:1)
	// Proof: `IdleScheduler::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn clear_tasks() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `865`
		//  Estimated: `865`
		// Minimum execution time: 9_007 nanoseconds.
		Weight::from_parts(9_335_000, 865)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `IdleScheduler::NextTaskId` (r:1 w:1)
	// Proof: `IdleScheduler::NextTaskId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `IdleScheduler::Tasks` (r:0 w:1)
	// Proof: `IdleScheduler::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn schedule_task() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1073`
		//  Estimated: `2558`
		// Minimum execution time: 16_303 nanoseconds.
		Weight::from_parts(16_595_000, 2558)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
