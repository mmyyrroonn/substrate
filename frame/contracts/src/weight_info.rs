// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

//! This module contains the `WeightInfo` trait and its unsafe implementation on `()`.

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

/// Should be implemented by automatically generated code of the benchmarking system for
/// every runtime that makes use of this pallet.
/// This trait is also implemented on `()`. The implemention on `()` is **unsafe** and must
/// only be used during development. Proper weights can be generated by running the
/// pallet_contracts benchmark suite for the runtime in question.
pub trait WeightInfo {
	fn update_schedule() -> Weight;
	fn put_code(n: u32, ) -> Weight;
	fn instantiate(n: u32, ) -> Weight;
	fn call() -> Weight;
	fn claim_surcharge() -> Weight;
	fn seal_caller(r: u32, ) -> Weight;
	fn seal_address(r: u32, ) -> Weight;
	fn seal_gas_left(r: u32, ) -> Weight;
	fn seal_balance(r: u32, ) -> Weight;
	fn seal_value_transferred(r: u32, ) -> Weight;
	fn seal_minimum_balance(r: u32, ) -> Weight;
	fn seal_tombstone_deposit(r: u32, ) -> Weight;
	fn seal_rent_allowance(r: u32, ) -> Weight;
	fn seal_block_number(r: u32, ) -> Weight;
	fn seal_now(r: u32, ) -> Weight;
	fn seal_weight_to_fee(r: u32, ) -> Weight;
	fn seal_gas(r: u32, ) -> Weight;
	fn seal_input(r: u32, ) -> Weight;
	fn seal_input_per_kb(n: u32, ) -> Weight;
	fn seal_return(r: u32, ) -> Weight;
	fn seal_return_per_kb(n: u32, ) -> Weight;
	fn seal_terminate(r: u32, ) -> Weight;
	fn seal_restore_to(r: u32, ) -> Weight;
	fn seal_restore_to_per_delta(d: u32, ) -> Weight;
	fn seal_random(r: u32, ) -> Weight;
	fn seal_deposit_event(r: u32, ) -> Weight;
	fn seal_deposit_event_per_topic_and_kb(t: u32, n: u32, ) -> Weight;
	fn seal_set_rent_allowance(r: u32, ) -> Weight;
	fn seal_set_storage(r: u32, ) -> Weight;
	fn seal_set_storage_per_kb(n: u32, ) -> Weight;
	fn seal_clear_storage(r: u32, ) -> Weight;
	fn seal_get_storage(r: u32, ) -> Weight;
	fn seal_get_storage_per_kb(n: u32, ) -> Weight;
	fn seal_transfer(r: u32, ) -> Weight;
	fn seal_call(r: u32, ) -> Weight;
	fn seal_call_per_transfer_input_output_kb(t: u32, i: u32, o: u32, ) -> Weight;
	fn seal_instantiate(r: u32, ) -> Weight;
	fn seal_instantiate_per_input_output_kb(i: u32, o: u32, ) -> Weight;
	fn seal_hash_sha2_256(r: u32, ) -> Weight;
	fn seal_hash_sha2_256_per_kb(n: u32, ) -> Weight;
	fn seal_hash_keccak_256(r: u32, ) -> Weight;
	fn seal_hash_keccak_256_per_kb(n: u32, ) -> Weight;
	fn seal_hash_blake2_256(r: u32, ) -> Weight;
	fn seal_hash_blake2_256_per_kb(n: u32, ) -> Weight;
	fn seal_hash_blake2_128(r: u32, ) -> Weight;
	fn seal_hash_blake2_128_per_kb(n: u32, ) -> Weight;
}

/// Unsafe implementation that must only be used for development.
impl WeightInfo for () {
	fn update_schedule() -> Weight {
		(45000000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn put_code(n: u32, ) -> Weight {
		(263409000 as Weight)
			.saturating_add((169269000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn instantiate(n: u32, ) -> Weight {
		(309311000 as Weight)
			.saturating_add((1018000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn call() -> Weight {
		(291000000 as Weight)
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn claim_surcharge() -> Weight {
		(766000000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn seal_caller(r: u32, ) -> Weight {
		(182241000 as Weight)
			.saturating_add((697428000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_address(r: u32, ) -> Weight {
		(193846000 as Weight)
			.saturating_add((695989000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_gas_left(r: u32, ) -> Weight {
		(166031000 as Weight)
			.saturating_add((702533000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_balance(r: u32, ) -> Weight {
		(251892000 as Weight)
			.saturating_add((1392900000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
	}
	fn seal_value_transferred(r: u32, ) -> Weight {
		(178472000 as Weight)
			.saturating_add((694921000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_minimum_balance(r: u32, ) -> Weight {
		(191301000 as Weight)
			.saturating_add((697871000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_tombstone_deposit(r: u32, ) -> Weight {
		(241315000 as Weight)
			.saturating_add((686403000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_rent_allowance(r: u32, ) -> Weight {
		(104958000 as Weight)
			.saturating_add((1459573000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_block_number(r: u32, ) -> Weight {
		(174140000 as Weight)
			.saturating_add((698152000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_now(r: u32, ) -> Weight {
		(203157000 as Weight)
			.saturating_add((713595000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_weight_to_fee(r: u32, ) -> Weight {
		(178413000 as Weight)
			.saturating_add((1071275000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
	}
	fn seal_gas(r: u32, ) -> Weight {
		(171395000 as Weight)
			.saturating_add((371653000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_input(r: u32, ) -> Weight {
		(184462000 as Weight)
			.saturating_add((10538000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_input_per_kb(n: u32, ) -> Weight {
		(194668000 as Weight)
			.saturating_add((301000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_return(r: u32, ) -> Weight {
		(175538000 as Weight)
			.saturating_add((7462000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_return_per_kb(n: u32, ) -> Weight {
		(189759000 as Weight)
			.saturating_add((754000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_terminate(r: u32, ) -> Weight {
		(184385000 as Weight)
			.saturating_add((542615000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((2 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_restore_to(r: u32, ) -> Weight {
		(380385000 as Weight)
			.saturating_add((160308000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().reads((3 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes((4 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_restore_to_per_delta(d: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((4786197000 as Weight).saturating_mul(d as Weight))
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(d as Weight)))
			.saturating_add(DbWeight::get().writes(5 as Weight))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(d as Weight)))
	}
	fn seal_random(r: u32, ) -> Weight {
		(187944000 as Weight)
			.saturating_add((1592530000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
	}
	fn seal_deposit_event(r: u32, ) -> Weight {
		(126517000 as Weight)
			.saturating_add((2346945000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_deposit_event_per_topic_and_kb(t: u32, n: u32, ) -> Weight {
		(2953428000 as Weight)
			.saturating_add((1117651000 as Weight).saturating_mul(t as Weight))
			.saturating_add((299890000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(t as Weight)))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(t as Weight)))
	}
	fn seal_set_rent_allowance(r: u32, ) -> Weight {
		(142094000 as Weight)
			.saturating_add((1726665000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn seal_set_storage(r: u32, ) -> Weight {
		(4091409000 as Weight)
			.saturating_add((26440116000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_set_storage_per_kb(n: u32, ) -> Weight {
		(3683270000 as Weight)
			.saturating_add((233826000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn seal_clear_storage(r: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((7152747000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_get_storage(r: u32, ) -> Weight {
		(19007000 as Weight)
			.saturating_add((1774675000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_get_storage_per_kb(n: u32, ) -> Weight {
		(1477332000 as Weight)
			.saturating_add((176601000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
	}
	fn seal_transfer(r: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((10274385000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_call(r: u32, ) -> Weight {
		(241916000 as Weight)
			.saturating_add((14633108000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_call_per_transfer_input_output_kb(t: u32, i: u32, o: u32, ) -> Weight {
		(15664107000 as Weight)
			.saturating_add((8529984000 as Weight).saturating_mul(t as Weight))
			.saturating_add((52860000 as Weight).saturating_mul(i as Weight))
			.saturating_add((81175000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(105 as Weight))
			.saturating_add(DbWeight::get().reads((101 as Weight).saturating_mul(t as Weight)))
			.saturating_add(DbWeight::get().writes((101 as Weight).saturating_mul(t as Weight)))
	}
	fn seal_instantiate(r: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((32247550000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().reads((300 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((200 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_instantiate_per_input_output_kb(i: u32, o: u32, ) -> Weight {
		(34376003000 as Weight)
			.saturating_add((151350000 as Weight).saturating_mul(i as Weight))
			.saturating_add((82364000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(207 as Weight))
			.saturating_add(DbWeight::get().writes(202 as Weight))
	}
	fn seal_hash_sha2_256(r: u32, ) -> Weight {
		(164203000 as Weight)
			.saturating_add((565206000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_sha2_256_per_kb(n: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((330063000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_keccak_256(r: u32, ) -> Weight {
		(219038000 as Weight)
			.saturating_add((567992000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_keccak_256_per_kb(n: u32, ) -> Weight {
		(434654000 as Weight)
			.saturating_add((271134000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_blake2_256(r: u32, ) -> Weight {
		(116374000 as Weight)
			.saturating_add((566612000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_blake2_256_per_kb(n: u32, ) -> Weight {
		(756028000 as Weight)
			.saturating_add((150363000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_blake2_128(r: u32, ) -> Weight {
		(150126000 as Weight)
			.saturating_add((564827000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_blake2_128_per_kb(n: u32, ) -> Weight {
		(1021689000 as Weight)
			.saturating_add((149452000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
}