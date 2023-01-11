const ethers = require('ethers');

const contractFile = require('./compile');
const contractABI = contractFile.abi;

const readContract = async (context, ...args) => {
    const {networkName, rpcPort, chainId, senderPrivKey, xc20PrecompileAddress} = args[0][0]
    const {balanceOfAddr} = args[0][1]
    const providerRPC = {
      development: {
        name: networkName,
        rpc: 'http://127.0.0.1:' + rpcPort,
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
    const contract = new ethers.Contract(xc20PrecompileAddress, contractABI, wallet);

    const name = await contract.name();
    const symbol = await contract.symbol();
    const totalSupply = await contract.totalSupply();
    const balanceOf = await contract.balanceOf(balanceOfAddr);

    const asset_metadata = {
        name,
        symbol,
        totalSupply: totalSupply.toNumber(),
        balanceOf: balanceOf.toNumber()
    }
    context.variables['$asset_metadata'] = asset_metadata;
}

export default readContract 


