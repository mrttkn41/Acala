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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::all)]

use sp_runtime::codec::Codec;

sp_api::decl_runtime_apis! {
	#[api_version(2)]
	pub trait CurrenciesApi<CurrencyId, AccountId, Balance> where
		CurrencyId: Codec,
		AccountId: Codec,
		Balance: Codec,
	{
		fn query_free_balance(currency_id: CurrencyId, who: AccountId) -> Balance;
	}
}
