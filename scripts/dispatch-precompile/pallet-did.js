const { ApiPromise, WsProvider } = require('@polkadot/api');
const polkadotCryptoUtils = require("@polkadot/util-crypto");
const Web3 = require('web3');

const wsProvider = new WsProvider('ws://127.0.0.1:9933');
const web3 = new Web3('http://127.0.0.1:9933');

async function eth_call() {
    const api = await ApiPromise.create({ provider: wsProvider });

    let ss58Prefix = 19;

    // Define the address to send the dispatch call to
    const dispatchAddress = "0x0000000000000000000000000000000000000401";

    // Define the sender account address and private key
    const senderAddress = '0xe31B11A052aFC923259949352B2f573a21301Ba4'; 
    const privateKey = '0x05306c74d0514acf0d2a02049c31284c17c9200270df88eda11f4421cd04742a';

    // Convert the sender address to SS58 format
    const senderAddressSS58 = polkadotCryptoUtils.evmToAddress(senderAddress, ss58Prefix);

    // Define the gas limit and gas price for the transaction
    const gasLimit = 256000; 
    const gasPrice = 0x10000000000; 

    //get create_did call hash
    let didCall = api.tx.did.createDid(senderAddressSS58, senderAddress, null, null);
    let callBytes = didCall?.method.toHex() || "";

    // Define the nonce for the transaction
    const nonce = await web3.eth.getTransactionCount(senderAddress);

    const ethCallObj = {
      from: senderAddress,
      to: dispatchAddress, // Address of the dispatch precompile
      nonce: nonce,
      gas: gasLimit,
      gasPrice: gasPrice,
      data: callBytes
  };

    //check if transaction *will* pass (is not included in block)
    let result = await web3.eth.call(ethCallObj);

    if (result.error) {
      console.log(result);
    }

    // Create a new transaction object
    const txObject = {
        from: senderAddress,
        to: dispatchAddress, // Address of the dispatch precompile
        nonce: nonce,
        gasLimit: gasLimit,
        gasPrice: gasPrice,
        data: callBytes
    };

    // Sign the transaction using the sender account private key
    const signedTx = await web3.eth.accounts.signTransaction(txObject, privateKey);

    // Send the signed transaction to the network
    const txReceipt = await web3.eth.sendSignedTransaction(signedTx.rawTransaction);

    console.log('Transaction receipt:', txReceipt);
}

eth_call();