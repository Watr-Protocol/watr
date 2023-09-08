// 1. Import packages
const fs = require('fs');
const solc = require('solc');

const loadErc20Contract = () => {
	// 2. Get path and load contract
	const source = fs.readFileSync('./integration-tests/customs/xcm/ERC20.sol', 'utf8');

	// 3. Create input object
	const input = {
		language: 'Solidity',
		sources: {
			'ERC20.sol': {content: source},
		},
		settings: {
			outputSelection: {
				'*': {'*': ['*']},
			},
		}
	};
	// 4. Compile the contract
	const tempFile = JSON.parse(solc.compile(JSON.stringify(input)));
	return tempFile.contracts['ERC20.sol']['IERC20'];
}

// 5. Export contract data
module.exports = loadErc20Contract();
