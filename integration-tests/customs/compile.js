"use strict";
// 1. Import packages
var fs = require('fs');
var solc = require('solc');
// 2. Get path and load contract
var source = fs.readFileSync('./integration-tests/customs/ERC20.sol', 'utf8');
// 3. Create input object
var input = {
    language: 'Solidity',
    sources: {
        'ERC20.sol': { content: source },
    },
    settings: {
        outputSelection: {
            '*': { '*': ['*'] },
        },
    }
};
// 4. Compile the contract
var tempFile = JSON.parse(solc.compile(JSON.stringify(input)));
var contractFile = tempFile.contracts['ERC20.sol']['IERC20'];
// 5. Export contract data
module.exports = contractFile;
//# sourceMappingURL=compile.js.map