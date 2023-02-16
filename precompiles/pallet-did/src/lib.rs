
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(assert_matches))]

use fp_evm::{PrecompileHandle, PrecompileOutput};
use frame_support::traits::fungibles::approvals::Inspect as ApprovalInspect;
use frame_support::traits::fungibles::metadata::Inspect as MetadataInspect;
use frame_support::traits::fungibles::Inspect;
use frame_support::traits::OriginTrait;
use frame_support::traits::Currency;
use frame_support::{
    dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
    sp_runtime::traits::StaticLookup,
};
use pallet_evm::{AddressMapping, Precompile};
use precompile_utils::{
    keccak256, succeed, Address, Bytes, EvmData, EvmDataWriter, EvmResult, FunctionModifier,
    LogExt, LogsBuilder, PrecompileHandleExt, RuntimeHelper,
};
use sp_runtime::traits::{Bounded, Zero};

use sp_core::{H160, U256};
use sp_std::{
    convert::{TryFrom, TryInto},
    marker::PhantomData,
};

use pallet_did::verification::DidSignature;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

/// Solidity selector of the Transfer log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_TRANSFER: [u8; 32] = keccak256!("Transfer(address,address,uint256)");

/// Solidity selector of the Approval log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_APPROVAL: [u8; 32] = keccak256!("Approval(address,address,uint256)");

/// Alias for the Balance type for the provided Runtime and Instance.
type BalanceOf<Runtime> = <<Runtime as pallet_did::Config>::Currency as Currency<
    <Runtime as frame_system::Config>::AccountId,
>>::Balance;

/// Alias for the Asset Id type for the provided Runtime and Instance.
pub type AssetIdOf<Runtime, Instance = ()> = <Runtime as pallet_assets::Config<Instance>>::AssetId;

#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
    CreateDid = "createDid(address, uint256)",
    TotalSupply = "totalSupply()",
    BalanceOf = "balanceOf(address)",
    Allowance = "allowance(address,address)",
    Transfer = "transfer(address,uint256)",
    Approve = "approve(address,uint256)",
    TransferFrom = "transferFrom(address,address,uint256)",
    Name = "name()",
    Symbol = "symbol()",
    Decimals = "decimals()",
    MinimumBalance = "minimumBalance()",
    Mint = "mint(address,uint256)",
    Burn = "burn(address,uint256)",
}

pub struct DidPrecompile<Runtime: 'static = ()>(
    PhantomData<(Runtime)>,
);

impl<Runtime> DidPrecompile<Runtime> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
type DidRuntimeCall<Runtime> = <Runtime as pallet_did::Config>::RuntimeCall;
impl<Runtime> Precompile for DidPrecompile<Runtime>
where
    Runtime: pallet_evm::Config + pallet_did::Config,
    BalanceOf<Runtime>: EvmData,
    <DidRuntimeCall<Runtime> as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
    DidRuntimeCall<Runtime>: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
    DidRuntimeCall<Runtime>: From<pallet_did::Call<Runtime>>,
    Runtime::AccountId: From<[u8; 32]>,
{
    fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
        let address = handle.code_address();
        return Self::create_did(handle);
    }

}

impl<Runtime> DidPrecompile<Runtime>
where
    Runtime: pallet_evm::Config + pallet_did::Config,
    BalanceOf<Runtime>: EvmData,
    <DidRuntimeCall<Runtime> as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
    DidRuntimeCall<Runtime>: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
    DidRuntimeCall<Runtime>: From<pallet_did::Call<Runtime>>,
    Runtime::AccountId: From<[u8; 32]>,
{
    fn create_did(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
        // handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;

        let mut input = handle.read_input()?;
        input.expect_arguments(2)?;

        let authenticator: H160 = input.read::<Address>()?.into();
        let signature = input.read::<U256>()?;

        // let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

        // Dispatch call (if enough gas).
        // RuntimeHelper::<Runtime>::try_dispatch(
        //     handle,
        //     Some(origin).into(),
        //     pallet_did::Call::<Runtime>::create_did {
        //         authenticator,
        //         signature,
        //     },
        // )?;
        Ok(succeed(EvmDataWriter::new().write(true).build()))
    }
   
}
