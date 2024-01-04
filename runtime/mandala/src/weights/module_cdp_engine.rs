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

//! Autogenerated weights for module_cdp_engine
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

/// Weight functions for module_cdp_engine.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_cdp_engine::WeightInfo for WeightInfo<T> {
	// Storage: Aura CurrentSlot (r:1 w:1)
	// Proof: Aura CurrentSlot (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Aura Authorities (r:1 w:0)
	// Proof: Aura Authorities (max_values: Some(1), max_size: Some(1025), added: 1520, mode: MaxEncodedLen)
	// Storage: CdpEngine LastAccumulationSecs (r:1 w:1)
	// Proof: CdpEngine LastAccumulationSecs (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Proof: EmergencyShutdown IsShutdown (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: CdpEngine CollateralParams (r:5 w:0)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	// Storage: Loans TotalPositions (r:3 w:0)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	// Storage: System ParentHash (r:0 w:1)
	// Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: System Digest (r:0 w:1)
	// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: System BlockHash (r:0 w:1)
	// Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	// Storage: unknown `0x3a65787472696e7369635f696e646578` (r:0 w:1)
	// Proof Skipped: unknown `0x3a65787472696e7369635f696e646578` (r:0 w:1)
	// Storage: Timestamp Now (r:0 w:1)
	// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Timestamp DidUpdate (r:0 w:1)
	// Proof: Timestamp DidUpdate (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 5]`.
	fn on_initialize(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1471 + c * (47 ±0)`
		//  Estimated: `30952 + c * (642 ±1)`
		// Minimum execution time: 44_681 nanoseconds.
		Weight::from_parts(49_236_785, 30952)
			// Standard Error: 69_544
			.saturating_add(Weight::from_parts(4_021_915, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(8))
			.saturating_add(Weight::from_parts(0, 642).saturating_mul(c.into()))
	}
	// Storage: CdpEngine CollateralParams (r:1 w:1)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	fn set_collateral_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1332`
		//  Estimated: `3600`
		// Minimum execution time: 38_981 nanoseconds.
		Weight::from_parts(39_906_000, 3600)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Proof: EmergencyShutdown IsShutdown (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Loans Positions (r:1 w:1)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Proof: AcalaOracle Values (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Proof: CdpTreasury DebitPool (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:1)
	// Proof: AuctionManager TotalCollateralInAuction (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Dex TradingPairStatuses (r:2 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:1 w:0)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: StableAsset Pools (r:1 w:0)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Proof Skipped: AggregatedDex AggregatedSwapPaths (max_values: None, max_size: None, mode: Measured)
	// Storage: CdpEngine LiquidationContracts (r:1 w:0)
	// Proof: CdpEngine LiquidationContracts (max_values: Some(1), max_size: Some(201), added: 696, mode: MaxEncodedLen)
	// Storage: CdpTreasury ExpectedCollateralAuctionSize (r:1 w:0)
	// Proof: CdpTreasury ExpectedCollateralAuctionSize (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AuctionManager TotalTargetInAuction (r:1 w:1)
	// Proof: AuctionManager TotalTargetInAuction (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Auction AuctionsIndex (r:1 w:1)
	// Proof: Auction AuctionsIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: AuctionManager CollateralAuctions (r:0 w:50)
	// Proof: AuctionManager CollateralAuctions (max_values: None, max_size: Some(139), added: 2614, mode: MaxEncodedLen)
	// Storage: Auction AuctionEndTime (r:0 w:50)
	// Proof: Auction AuctionEndTime (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	// Storage: Auction Auctions (r:0 w:50)
	// Proof: Auction Auctions (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 50]`.
	fn liquidate_by_auction(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4862`
		//  Estimated: `111198`
		// Minimum execution time: 230_858 nanoseconds.
		Weight::from_parts(237_835_581, 111198)
			// Standard Error: 25_671
			.saturating_add(Weight::from_parts(13_562_630, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(29))
			.saturating_add(T::DbWeight::get().writes(15))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(b.into())))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Proof: EmergencyShutdown IsShutdown (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Loans Positions (r:1 w:1)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Proof: AcalaOracle Values (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:6 w:6)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:4 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Proof: CdpTreasury DebitPool (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:0)
	// Proof: AuctionManager TotalCollateralInAuction (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:2 w:2)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: StableAsset Pools (r:1 w:0)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Proof Skipped: AggregatedDex AggregatedSwapPaths (max_values: None, max_size: None, mode: Measured)
	fn liquidate_by_dex() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5683`
		//  Estimated: `150493`
		// Minimum execution time: 332_621 nanoseconds.
		Weight::from_parts(343_596_000, 150493)
			.saturating_add(T::DbWeight::get().reads(36))
			.saturating_add(T::DbWeight::get().writes(16))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Proof: EmergencyShutdown IsShutdown (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Loans Positions (r:1 w:1)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Proof: CdpTreasury DebitPool (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	fn settle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3690`
		//  Estimated: `53624`
		// Minimum execution time: 127_256 nanoseconds.
		Weight::from_parts(130_551_000, 53624)
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: CdpEngine LiquidationContracts (r:1 w:1)
	// Proof: CdpEngine LiquidationContracts (max_values: Some(1), max_size: Some(201), added: 696, mode: MaxEncodedLen)
	fn register_liquidation_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1211`
		//  Estimated: `1686`
		// Minimum execution time: 20_772 nanoseconds.
		Weight::from_parts(21_773_000, 1686)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CdpEngine LiquidationContracts (r:1 w:1)
	// Proof: CdpEngine LiquidationContracts (max_values: Some(1), max_size: Some(201), added: 696, mode: MaxEncodedLen)
	fn deregister_liquidation_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1252`
		//  Estimated: `1686`
		// Minimum execution time: 20_982 nanoseconds.
		Weight::from_parts(21_601_000, 1686)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
