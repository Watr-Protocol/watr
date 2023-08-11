// Copyright 2023 Watr Foundation
// This file is part of Watr.

// Watr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Watr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Watr.  If not, see <http://www.gnu.org/licenses/>.

// This file was originally forked from Substrate Parachain Template
// which is generated directly to the upstream Parachain Template in Cumulus
// https://github.com/paritytech/cumulus/tree/master/parachain-template

use std::{sync::Arc, time::Duration};

use cumulus_primitives_parachain_inherent::MockValidationDataInherentDataProvider;
use sc_client_api::BlockBackend;
use sp_core::{Encode, Pair};
use sp_inherents::{InherentData, InherentDataProvider};
use sp_keyring::Sr25519Keyring;
use sp_runtime::{generic, OpaqueExtrinsic, SaturatedConversion};

use crate::service::{ParachainClient, WatrDevnetRuntimeExecutor, WatrRuntimeExecutor};

/// Generates `System::Remark` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct RemarkBuilder<RuntimeApi, Executor: sc_executor::NativeExecutionDispatch> {
	client: Arc<ParachainClient<RuntimeApi, Executor>>,
}

impl<RuntimeApi, Executor: sc_executor::NativeExecutionDispatch>
	RemarkBuilder<RuntimeApi, Executor>
{
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<ParachainClient<RuntimeApi, Executor>>) -> Self {
		Self { client }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder
	for RemarkBuilder<watr_devnet_runtime::RuntimeApi, WatrDevnetRuntimeExecutor>
{
	fn pallet(&self) -> &str {
		"system"
	}

	fn extrinsic(&self) -> &str {
		"remark"
	}

	fn build(&self, nonce: u32) -> Result<OpaqueExtrinsic, &'static str> {
		use watr_devnet_runtime as runtime;

		let call: runtime::RuntimeCall = runtime::SystemCall::remark { remark: vec![] }.into();
		let period = runtime::BlockHashCount::get()
			.checked_next_power_of_two()
			.map(|c| c / 2)
			.unwrap_or(2) as u64;
		let best_block = self.client.chain_info().best_number;
		let tip = 0;
		let extra: runtime::SignedExtra = (
			frame_system::CheckNonZeroSender::<runtime::Runtime>::new(),
			frame_system::CheckSpecVersion::<runtime::Runtime>::new(),
			frame_system::CheckTxVersion::<runtime::Runtime>::new(),
			frame_system::CheckGenesis::<runtime::Runtime>::new(),
			frame_system::CheckEra::<runtime::Runtime>::from(generic::Era::mortal(
				period,
				best_block.saturated_into(),
			)),
			frame_system::CheckNonce::<runtime::Runtime>::from(nonce.into()),
			frame_system::CheckWeight::<runtime::Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<runtime::Runtime>::from(tip),
		);

		let genesis_hash = self.client.block_hash(0).ok().flatten().expect("Genesis block exists");
		let best_hash = self.client.chain_info().best_hash;
		let payload = runtime::SignedPayload::from_raw(
			call.clone(),
			extra.clone(),
			(
				(),
				runtime::VERSION.spec_version,
				runtime::VERSION.transaction_version,
				genesis_hash,
				best_hash,
				(),
				(),
				(),
			),
		);

		let sender = Sr25519Keyring::Bob.pair();
		let signature = payload.using_encoded(|x| sender.sign(x));
		let extrinsic = runtime::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::AccountId32::from(sender.public()).into(),
			runtime::Signature::Sr25519(signature),
			extra,
		);

		Ok(extrinsic.into())
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder
	for RemarkBuilder<watr_runtime::RuntimeApi, WatrRuntimeExecutor>
{
	fn pallet(&self) -> &str {
		"system"
	}

	fn extrinsic(&self) -> &str {
		"remark"
	}

	fn build(&self, nonce: u32) -> Result<OpaqueExtrinsic, &'static str> {
		use watr_runtime as runtime;

		let call: runtime::RuntimeCall = runtime::SystemCall::remark { remark: vec![] }.into();
		let period = runtime::BlockHashCount::get()
			.checked_next_power_of_two()
			.map(|c| c / 2)
			.unwrap_or(2) as u64;
		let best_block = self.client.chain_info().best_number;
		let tip = 0;
		let extra: runtime::SignedExtra = (
			frame_system::CheckNonZeroSender::<runtime::Runtime>::new(),
			frame_system::CheckSpecVersion::<runtime::Runtime>::new(),
			frame_system::CheckTxVersion::<runtime::Runtime>::new(),
			frame_system::CheckGenesis::<runtime::Runtime>::new(),
			frame_system::CheckEra::<runtime::Runtime>::from(generic::Era::mortal(
				period,
				best_block.saturated_into(),
			)),
			frame_system::CheckNonce::<runtime::Runtime>::from(nonce.into()),
			frame_system::CheckWeight::<runtime::Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<runtime::Runtime>::from(tip),
		);

		let genesis_hash = self.client.block_hash(0).ok().flatten().expect("Genesis block exists");
		let best_hash = self.client.chain_info().best_hash;
		let payload = runtime::SignedPayload::from_raw(
			call.clone(),
			extra.clone(),
			(
				(),
				runtime::VERSION.spec_version,
				runtime::VERSION.transaction_version,
				genesis_hash,
				best_hash,
				(),
				(),
				(),
			),
		);

		let sender = Sr25519Keyring::Bob.pair();
		let signature = payload.using_encoded(|x| sender.sign(x));
		let extrinsic = runtime::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::AccountId32::from(sender.public()).into(),
			runtime::Signature::Sr25519(signature),
			extra,
		);

		Ok(extrinsic.into())
	}
}

/// Generates inherent data for the `benchmark overhead` command.
pub fn inherent_benchmark_data() -> sc_cli::Result<InherentData> {
	let mut inherent_data = InherentData::new();

	let timestamp = sp_timestamp::InherentDataProvider::new(Duration::ZERO.into());
	futures::executor::block_on(timestamp.provide_inherent_data(&mut inherent_data))
		.map_err(|e| format!("creating inherent data: {e:?}"))?;

	let parachain_inherent = MockValidationDataInherentDataProvider {
		current_para_block: 1,
		relay_offset: 0,
		relay_blocks_per_para_block: 1,
		para_blocks_per_relay_epoch: 0,
		relay_randomness_config: (),
		xcm_config: Default::default(),
		raw_downward_messages: Default::default(),
		raw_horizontal_messages: Default::default(),
	};

	futures::executor::block_on(parachain_inherent.provide_inherent_data(&mut inherent_data))
		.map_err(|e| format!("creating inherent data: {e:?}"))?;

	Ok(inherent_data)
}
