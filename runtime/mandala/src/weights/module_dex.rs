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

//! Autogenerated weights for module_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-43-79`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_dex.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_dex::WeightInfo for WeightInfo<T> {
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn enable_trading_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1180`
		//  Estimated: `3660`
		// Minimum execution time: 23_677 nanoseconds.
		Weight::from_parts(24_388_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn disable_trading_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1180`
		//  Estimated: `3660`
		// Minimum execution time: 23_665 nanoseconds.
		Weight::from_parts(24_300_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Dex ProvisioningPool (r:1 w:0)
	// Proof: Dex ProvisioningPool (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	fn list_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1340`
		//  Estimated: `10823`
		// Minimum execution time: 34_816 nanoseconds.
		Weight::from_parts(35_943_000, 10823)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn update_provisioning_parameters() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `985`
		//  Estimated: `3660`
		// Minimum execution time: 16_197 nanoseconds.
		Weight::from_parts(16_936_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Dex InitialShareExchangeRates (r:0 w:1)
	// Proof: Dex InitialShareExchangeRates (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	fn end_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2587`
		//  Estimated: `17988`
		// Minimum execution time: 60_323 nanoseconds.
		Weight::from_parts(62_514_000, 17988)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Proof: Dex ProvisioningPool (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	fn add_provision() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2607`
		//  Estimated: `17118`
		// Minimum execution time: 96_627 nanoseconds.
		Weight::from_parts(98_740_000, 17118)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex ProvisioningPool (r:2 w:1)
	// Proof: Dex ProvisioningPool (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	// Storage: Dex InitialShareExchangeRates (r:1 w:1)
	// Proof: Dex InitialShareExchangeRates (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_dex_share() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2978`
		//  Estimated: `23350`
		// Minimum execution time: 81_015 nanoseconds.
		Weight::from_parts(84_317_000, 23350)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3034`
		//  Estimated: `26757`
		// Minimum execution time: 114_391 nanoseconds.
		Weight::from_parts(118_334_000, 26757)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	fn add_liquidity_and_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3474`
		//  Estimated: `43257`
		// Minimum execution time: 157_071 nanoseconds.
		Weight::from_parts(162_022_000, 43257)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2828`
		//  Estimated: `19572`
		// Minimum execution time: 105_526 nanoseconds.
		Weight::from_parts(108_816_000, 19572)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn remove_liquidity_by_unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3632`
		//  Estimated: `42516`
		// Minimum execution time: 171_125 nanoseconds.
		Weight::from_parts(174_985_000, 42516)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:3 w:3)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2602 + u * (108 ±0)`
		//  Estimated: `17078 + u * (1270 ±18)`
		// Minimum execution time: 90_439 nanoseconds.
		Weight::from_parts(68_886_704, 17078)
			// Standard Error: 76_532
			.saturating_add(Weight::from_parts(12_325_417, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 1270).saturating_mul(u.into()))
	}
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:3 w:3)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_target(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2602 + u * (108 ±0)`
		//  Estimated: `17078 + u * (1270 ±18)`
		// Minimum execution time: 90_307 nanoseconds.
		Weight::from_parts(69_243_875, 17078)
			// Standard Error: 79_659
			.saturating_add(Weight::from_parts(12_265_695, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 1270).saturating_mul(u.into()))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex InitialShareExchangeRates (r:1 w:0)
	// Proof: Dex InitialShareExchangeRates (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Proof: Dex ProvisioningPool (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn refund_provision() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3039`
		//  Estimated: `24234`
		// Minimum execution time: 101_945 nanoseconds.
		Weight::from_parts(103_554_000, 24234)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn abort_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1318`
		//  Estimated: `3660`
		// Minimum execution time: 29_131 nanoseconds.
		Weight::from_parts(30_190_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
