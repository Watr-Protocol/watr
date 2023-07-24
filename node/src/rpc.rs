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

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use jsonrpsee::RpcModule;
use std::{collections::BTreeMap, sync::Arc};

use watr_common::{opaque::Block, AccountId, Balance, Hash, Index as Nonce};

use sc_client_api::{
	backend::{AuxStore, Backend, StateBackend, StorageProvider},
	client::BlockchainEvents,
};
use sc_network::NetworkService;
use sc_network_sync::SyncingService;
use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use sp_api::{CallApiAt, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{
	Backend as BlockchainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
};

use sp_runtime::traits::{BlakeTwo256, Block as BlockT};

// Frontier
use fc_rpc::{
	EthBlockDataCacheTask, OverrideHandle, RuntimeApiStorageOverride, SchemaV1Override,
	SchemaV2Override, SchemaV3Override, TxPool, TxPoolApiServer,
};
use fc_rpc_core::types::{FeeHistoryCache, FilterPool};
use fp_rpc::EthereumRuntimeRPCApi;
use fp_storage::EthereumStorageSchema;

/// Full client dependencies.
pub struct FullDeps<C, P, A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// The Node authority flag
	pub is_authority: bool,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// Chain syncing service
	pub sync: Arc<SyncingService<Block>>,
	/// EthFilterApi pool.
	pub filter_pool: Option<FilterPool>,
	/// Backend.
	pub backend: Arc<fc_db::kv::Backend<Block>>,
	/// Fee history cache.
	pub fee_history_cache: FeeHistoryCache,
	/// Maximum fee history cache size.
	pub fee_history_cache_limit: u64,
	/// Ethereum data access overrides.
	pub overrides: Arc<OverrideHandle<Block>>,
	/// Cache for Ethereum block data.
	pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
}

pub fn overrides_handle<B, C, BE>(client: Arc<C>) -> Arc<OverrideHandle<B>>
where
	B: BlockT,
	C: ProvideRuntimeApi<B>,
	C::Api: EthereumRuntimeRPCApi<B>,
	C: HeaderBackend<B> + StorageProvider<B, BE> + 'static,
	BE: Backend<B> + 'static,
{
	Arc::new(OverrideHandle {
		schemas: BTreeMap::from([
			(EthereumStorageSchema::V1, Box::new(SchemaV1Override::new(client.clone())) as Box<_>),
			(EthereumStorageSchema::V2, Box::new(SchemaV2Override::new(client.clone())) as Box<_>),
			(EthereumStorageSchema::V3, Box::new(SchemaV3Override::new(client.clone())) as Box<_>),
		]),
		fallback: Box::new(RuntimeApiStorageOverride::new(client)),
	})
}

// Do not import directly from Frontier.
// The DB path will be different, and the chains must be synced
// from the start again.
pub fn open_frontier_backend<C>(
	client: Arc<C>,
	config: &sc_service::Configuration,
) -> Result<Arc<fc_db::kv::Backend<Block>>, String>
where
	C: sp_blockchain::HeaderBackend<Block>,
{
	let config_dir = config.base_path.config_dir(config.chain_spec.id());
	let path = config_dir.join("frontier").join("db");

	Ok(Arc::new(fc_db::kv::Backend::<Block>::new(
		client,
		&fc_db::kv::DatabaseSettings {
			source: fc_db::DatabaseSource::RocksDb { path, cache_size: 0 },
		},
	)?))
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P, BE, A>(
	deps: FullDeps<C, P, A>,
	subscription_task_executor: SubscriptionTaskExecutor,
	pubsub_notification_sinks: Arc<
		fc_mapping_sync::EthereumBlockNotificationSinks<
			fc_mapping_sync::EthereumBlockNotification<Block>,
		>,
	>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ StorageProvider<Block, BE>
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ BlockchainEvents<Block>
		+ Send
		+ Sync
		+ 'static,
	C: sc_client_api::BlockBackend<Block>,
	C: CallApiAt<Block>,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ BlockBuilder<Block>,
	P: TransactionPool<Block = Block> + Sync + Send + 'static,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	BE::Blockchain: BlockchainBackend<Block>,
	A: ChainApi<Block = Block> + 'static,
{
	use fc_rpc::{
		Eth, EthApiServer, EthFilter, EthFilterApiServer, EthPubSub, EthPubSubApiServer, Net,
		NetApiServer, Web3, Web3ApiServer,
	};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut io = RpcModule::new(());
	let FullDeps {
		client,
		pool,
		graph,
		deny_unsafe,
		is_authority,
		network,
		sync,
		filter_pool,
		backend,
		fee_history_cache,
		fee_history_cache_limit,
		overrides,
		block_data_cache,
	} = deps;

	io.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	io.merge(TransactionPayment::new(client.clone()).into_rpc())?;

	let signers = Vec::new();

	let no_tx_converter: Option<fp_rpc::NoTransactionConverter> = None;

	io.merge(
		Eth::new(
			client.clone(),
			pool.clone(),
			graph.clone(),
			no_tx_converter,
			sync.clone(),
			signers,
			overrides.clone(),
			backend.clone(),
			// Is authority.
			is_authority,
			block_data_cache.clone(),
			fee_history_cache,
			fee_history_cache_limit,
			10,
			None,
		)
		.into_rpc(),
	)?;

	let max_past_logs: u32 = 10_000;
	let max_stored_filters: usize = 500;
	let tx_pool = TxPool::new(client.clone(), graph);
	if let Some(filter_pool) = filter_pool {
		io.merge(
			EthFilter::new(
				client.clone(),
				backend,
				tx_pool.clone(),
				filter_pool,
				max_stored_filters, // max stored filters
				max_past_logs,
				block_data_cache,
			)
			.into_rpc(),
		)?;
	}

	io.merge(
		EthPubSub::new(
			pool,
			client.clone(),
			sync,
			subscription_task_executor,
			overrides,
			pubsub_notification_sinks,
		)
		.into_rpc(),
	)?;

	io.merge(
		Net::new(
			client.clone(),
			network,
			// Whether to format the `peer_count` response as Hex (default) or not.
			true,
		)
		.into_rpc(),
	)?;

	io.merge(Web3::new(client).into_rpc())?;
	io.merge(tx_pool.into_rpc())?;

	Ok(io)
}
