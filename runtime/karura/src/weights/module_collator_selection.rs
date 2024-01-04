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

//! Autogenerated weights for module_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-37-73`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_collator_selection.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_collator_selection::WeightInfo for WeightInfo<T> {
	// Storage: `CollatorSelection::Invulnerables` (r:0 w:1)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(321), added: 816, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 10]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `965`
		//  Estimated: `0`
		// Minimum execution time: 14_647 nanoseconds.
		Weight::from_parts(15_183_860, 0)
			// Standard Error: 1_879
			.saturating_add(Weight::from_parts(6_034, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `CollatorSelection::DesiredCandidates` (r:0 w:1)
	// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `965`
		//  Estimated: `0`
		// Minimum execution time: 14_243 nanoseconds.
		Weight::from_parts(14_610_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `CollatorSelection::CandidacyBond` (r:0 w:1)
	// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_candidacy_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `965`
		//  Estimated: `0`
		// Minimum execution time: 14_414 nanoseconds.
		Weight::from_parts(14_846_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `CollatorSelection::NonCandidates` (r:1 w:1)
	// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
	// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(321), added: 816, mode: `MaxEncodedLen`)
	// Storage: `Session::NextKeys` (r:1 w:0)
	// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[5, 50]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2293 + c * (60 ±0)`
		//  Estimated: `5704 + c * (61 ±0)`
		// Minimum execution time: 61_734 nanoseconds.
		Weight::from_parts(61_649_370, 5704)
			// Standard Error: 3_650
			.saturating_add(Weight::from_parts(470_899, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 61).saturating_mul(c.into()))
	}
	// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
	// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(321), added: 816, mode: `MaxEncodedLen`)
	// Storage: `Session::NextKeys` (r:1 w:0)
	// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:0)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 50]`.
	fn register_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1806 + c * (55 ±0)`
		//  Estimated: `5263 + c * (55 ±0)`
		// Minimum execution time: 34_642 nanoseconds.
		Weight::from_parts(39_361_743, 5263)
			// Standard Error: 3_266
			.saturating_add(Weight::from_parts(425_138, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 55).saturating_mul(c.into()))
	}
	// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `Session::CurrentIndex` (r:1 w:0)
	// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::NonCandidates` (r:0 w:1)
	// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[6, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1596 + c * (39 ±0)`
		//  Estimated: `3086 + c * (39 ±0)`
		// Minimum execution time: 27_714 nanoseconds.
		Weight::from_parts(27_472_930, 3086)
			// Standard Error: 1_774
			.saturating_add(Weight::from_parts(321_543, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 39).saturating_mul(c.into()))
	}
	// Storage: `CollatorSelection::NonCandidates` (r:1 w:1)
	// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	// Storage: `Session::CurrentIndex` (r:1 w:0)
	// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	fn withdraw_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3161`
		//  Estimated: `4646`
		// Minimum execution time: 60_455 nanoseconds.
		Weight::from_parts(61_792_000, 4646)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::SessionPoints` (r:1 w:0)
	// Proof: `CollatorSelection::SessionPoints` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2106`
		//  Estimated: `6196`
		// Minimum execution time: 62_834 nanoseconds.
		Weight::from_parts(64_406_000, 6196)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `CollatorSelection::Candidates` (r:1 w:0)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(321), added: 816, mode: `MaxEncodedLen`)
	fn new_session() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2655`
		//  Estimated: `3086`
		// Minimum execution time: 24_538 nanoseconds.
		Weight::from_parts(25_724_000, 3086)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: `Session::Validators` (r:1 w:0)
	// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::Candidates` (r:1 w:0)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::SessionPoints` (r:0 w:50)
	// Proof: `CollatorSelection::SessionPoints` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[5, 50]`.
	/// The range of component `c` is `[5, 50]`.
	fn start_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1379 + c * (64 ±0)`
		//  Estimated: `3086 + c * (64 ±0)`
		// Minimum execution time: 22_067 nanoseconds.
		Weight::from_parts(16_345_954, 3086)
			// Standard Error: 1_735
			.saturating_add(Weight::from_parts(11_983, 0).saturating_mul(r.into()))
			// Standard Error: 1_735
			.saturating_add(Weight::from_parts(1_294_691, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(c.into()))
	}
	// Storage: `CollatorSelection::SessionPoints` (r:51 w:50)
	// Proof: `CollatorSelection::SessionPoints` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `Session::CurrentIndex` (r:1 w:0)
	// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::NonCandidates` (r:0 w:1)
	// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[5, 50]`.
	/// The range of component `c` is `[5, 50]`.
	fn end_session(_r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3426 + c * (49 ±0)`
		//  Estimated: `3509 + c * (2519 ±0)`
		// Minimum execution time: 32_499 nanoseconds.
		Weight::from_parts(460_647_523, 3509)
			// Standard Error: 16_327
			.saturating_add(Weight::from_parts(3_716_677, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(48))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2519).saturating_mul(c.into()))
	}
}
