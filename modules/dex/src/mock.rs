//! Mocks for the dex module.

#![cfg(test)]

use frame_support::{impl_outer_event, impl_outer_origin, ord_parameter_types, parameter_types};
use primitives::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, Perbill};
use support::{AuctionManager, Rate};
use system::EnsureSignedBy;

use super::*;

mod dex {
	pub use super::super::*;
}

impl_outer_event! {
	pub enum TestEvent for Runtime {
		system<T>,
		dex<T>,
		orml_tokens<T>,
		cdp_treasury<T>,
	}
}

impl_outer_origin! {
	pub enum Origin for Runtime {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Runtime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
	pub const GetBaseCurrencyId: CurrencyId = AUSD;
	pub const GetExchangeFee: Rate = Rate::from_rational(1, 100);
	pub const ExistentialDeposit: u128 = 0;
	pub const EnabledCurrencyIds : Vec<CurrencyId> = vec![BTC, DOT];
}

pub type AccountId = u64;
pub type BlockNumber = u64;
pub type CurrencyId = u32;
pub type Share = u128;
pub type Balance = u128;
pub type Amount = i128;
pub type AuctionId = u64;

impl system::Trait for Runtime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = ();
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
}
pub type System = system::Module<Runtime>;

impl orml_tokens::Trait for Runtime {
	type Event = TestEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type ExistentialDeposit = ExistentialDeposit;
	type DustRemoval = ();
}
pub type Tokens = orml_tokens::Module<Runtime>;

ord_parameter_types! {
	pub const One: AccountId = 1;
}

parameter_types! {
	pub const GetStableCurrencyId: CurrencyId = AUSD;
}

impl cdp_treasury::Trait for Runtime {
	type Event = TestEvent;
	type Currency = Tokens;
	type GetStableCurrencyId = GetStableCurrencyId;
	type AuctionManagerHandler = MockAuctionManagerHandler;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type DEX = ();
}
pub type CDPTreasuryModule = cdp_treasury::Module<Runtime>;

pub struct MockAuctionManagerHandler;
impl AuctionManager<AccountId> for MockAuctionManagerHandler {
	type CurrencyId = CurrencyId;
	type Balance = Balance;
	type AuctionId = AuctionId;
	fn new_collateral_auction(
		_who: &AccountId,
		_currency_id: Self::CurrencyId,
		_amount: Self::Balance,
		_target: Self::Balance,
	) {
		unimplemented!()
	}
	fn new_debit_auction(_amount: Self::Balance, _fix: Self::Balance) {
		unimplemented!()
	}
	fn new_surplus_auction(_amount: Self::Balance) {
		unimplemented!()
	}
	fn cancel_auction(_id: Self::AuctionId) -> DispatchResult {
		unimplemented!()
	}

	fn get_total_collateral_in_auction(_id: Self::CurrencyId) -> Self::Balance {
		unimplemented!()
	}
	fn get_total_surplus_in_auction() -> Self::Balance {
		unimplemented!()
	}
	fn get_total_debit_in_auction() -> Self::Balance {
		unimplemented!()
	}
	fn get_total_target_in_auction() -> Self::Balance {
		unimplemented!()
	}
}

impl Trait for Runtime {
	type Event = TestEvent;
	type Currency = Tokens;
	type Share = Share;
	type EnabledCurrencyIds = EnabledCurrencyIds;
	type GetBaseCurrencyId = GetBaseCurrencyId;
	type GetExchangeFee = GetExchangeFee;
	type CDPTreasury = CDPTreasuryModule;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
}
pub type DexModule = Module<Runtime>;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CAROL: AccountId = 3;

pub const AUSD: CurrencyId = 1;
pub const BTC: CurrencyId = 2;
pub const DOT: CurrencyId = 3;
pub const ACA: CurrencyId = 4;

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
	liquidity_incentive_rate: Vec<(CurrencyId, Rate)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![
				(ALICE, AUSD, 1_000_000_000_000_000_000u128),
				(BOB, AUSD, 1_000_000_000_000_000_000u128),
				(ALICE, BTC, 1_000_000_000_000_000_000u128),
				(BOB, BTC, 1_000_000_000_000_000_000u128),
				(ALICE, DOT, 1_000_000_000_000_000_000u128),
				(BOB, DOT, 1_000_000_000_000_000_000u128),
			],
			liquidity_incentive_rate: vec![(BTC, Rate::from_rational(1, 100)), (DOT, Rate::from_rational(1, 100))],
		}
	}
}

impl ExtBuilder {
	pub fn set_balance(mut self, who: AccountId, currency_id: CurrencyId, balance: Balance) -> Self {
		self.endowed_accounts.push((who, currency_id, balance));
		self
	}
	pub fn build(self) -> runtime_io::TestExternalities {
		let mut t = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		orml_tokens::GenesisConfig::<Runtime> {
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		dex::GenesisConfig::<Runtime> {
			liquidity_incentive_rate: self.liquidity_incentive_rate,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
