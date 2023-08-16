const ethers = require('ethers');
const contractFile = require('./compile.js');
const contractABI = contractFile.abi;

const setup = (params) => {
	const {networkName, rpcPort, chainId, senderPrivKey, didPrecompileAddress} = params;

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
	return new ethers.Contract(didPrecompileAddress, contractABI, wallet);
};

module.exports = setup;
