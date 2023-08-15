const ethers = require('ethers');

const contractFile = require('./compile.js');
const contractABI = contractFile.abi;

const removeDid = async (context, ...args) => {
	const {networkName, rpcPort, chainId, senderPrivKey, didPrecompileAddress} = args[0][0];
	const {did} = args[0][1];

	const providerRPC = {
		development: {
			name: networkName,
			rpc: `http://127.0.0.1:${rpcPort}`,
			chainId,
		},
	};

	const provider = new ethers.providers.StaticJsonRpcProvider(providerRPC.development.rpc, {
		chainId: providerRPC.development.chainId,
		name: providerRPC.development.name,
	});

	// Create a new instance of the Wallet class
	const wallet = new ethers.Wallet(senderPrivKey, provider);

	// Create a new instance of the Contract class
	const contract = new ethers.Contract(didPrecompileAddress, contractABI, wallet);

	try {
		const tx = await contract.removeDid(did, {gasLimit: 250000});
		await tx.wait();
	} catch (e) {
		console.error(`\n⚠️  WARNING: Eth tx failed\n`, e);
	}
}

module.exports = removeDid;
