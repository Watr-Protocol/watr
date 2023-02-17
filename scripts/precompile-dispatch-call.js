const { ApiPromise, WsProvider } = require('@polkadot/api');
const Web3 = require('web3');

const wsProvider = new WsProvider('ws://127.0.0.1::9933');
const web3 = new Web3('http://127.0.0.1:8833');

async function eth_call() {
    const api = await ApiPromise.create({ provider: wsProvider });
    
    let randomAddress = "2yAoYcWnpcnUZjtQnGpbYBPh5FLZEcws2xDYVoCu4R1gChYK";
    // 100 followed by 18 zeroes 
    let amountToTransfer = 100n * (10n ** 18n);

    // Define the address to send the dispatch call to
    const dispatchAddress = "0x0000000000000000000000000000000000000401";

    // Define the sender account address and private key
    const senderAddress = '0xe31B11A052aFC923259949352B2f573a21301Ba4'; 
    const privateKey = '0x05306c74d0514acf0d2a02049c31284c17c9200270df88eda11f4421cd04742a';

    // Define the gas limit and gas price for the transaction
    const gasLimit = 256000; 
    const gasPrice = 0x10000000000; 

    let balanceBefore = await getAccountBalance(api, randomAddress);
    console.log(Number(balanceBefore));

    let eth_balance = await getAccountBalance(web3, senderAddress, true);
    console.log(Number(eth_balance));

    //get balance transfer call hash
    let balancesCall = api.tx.balances.transfer(randomAddress, amountToTransfer);
    let callBytes = balancesCall?.method.toHex() || "";

    let didCreateCallBytes = "0x3c00e31b11a052afc923259949352b2f573a21301ba4022633913d75df1af1d6356aaf0e1d8658c5739a16c53bf79e7ed38c5beb89e2094dcc4ab110635762d0ea795410bfa14dcab09f696995cde1db5d23458ab0121c1b";

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

    let balanceAfter = await getAccountBalance(api, randomAddress);
    console.log(Number(balanceAfter));

    if (balanceAfter - balanceBefore == amountToTransfer) console.log("success!!!");
}

async function getAccountBalance(provider, address, isEth=false) {
  let balance = 0; 

  if (isEth) {
    balance = await provider.eth.getBalance(address);
  }else {
    let res = await provider.query.system.account(address);
    balance = res.data.free;
  }

  return balance 
}


eth_call();