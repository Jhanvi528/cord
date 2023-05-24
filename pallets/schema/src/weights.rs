// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_schema
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `RohitsMacBook14`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/cord
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_schema
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/schema/src/weights.rs
// --header=./HEADER-GPL3
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_schema.
pub trait WeightInfo {
	fn create(l: u32, ) -> Weight;
}

/// Weights for pallet_schema using the CORD node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Schema Schemas (r:1 w:1)
	/// Proof: Schema Schemas (max_values: None, max_size: Some(15496), added: 17971, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 15360]`.
	fn create(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `18961`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(24_263_116, 18961)
			// Standard Error: 56
			.saturating_add(Weight::from_parts(79_703, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Schema Schemas (r:1 w:1)
	/// Proof: Schema Schemas (max_values: None, max_size: Some(15496), added: 17971, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 15360]`.
	fn create(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `18961`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(24_263_116, 18961)
			// Standard Error: 56
			.saturating_add(Weight::from_parts(79_703, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
