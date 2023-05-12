const signData = async (data) => {

    // Check if MetaMask is installed
    if (typeof window.ethereum === 'undefined') {
      alert('Please install MetaMask to use this feature.');
      return;
    }
    
    // Request access to user's Ethereum account
    await window.ethereum.request({ method: 'eth_requestAccounts' });
    
    // Get the user's Ethereum address
    const account = (await window.ethereum.request({ method: 'eth_accounts' }))[0];    

    // Sign the data
    const signature = await window.ethereum.request({
      method: 'personal_sign',
      // stringify with 2 spaces (to pretty print)
      params: [JSON.stringify(data, null, 2), account],
    });
    
    // Update the contents of the "accountAddress" and "signedData" elements
    document.getElementById("accountAddress").innerHTML = account;
    document.getElementById("signedData").innerHTML = signature;

    // Log the signed data and signature
    console.log(`Data: ${data}`);
    console.log(`Signature: ${signature}`);
  };
  
  