// 1. Import packages
const fs = require('fs');
const solc = require('solc');

const loadDidContract = () => {
	// 2. Get path and load contract
	const source = fs.readFileSync('./precompiles/did/WatrDID.sol', 'utf8');

	// 3. Create input object
	const input = {
		language: 'Solidity',
		sources: {
			'WatrDID.sol': {content: source},
		},
		settings: {
			outputSelection: {
				'*': {'*': ['*']},
			},
		}
	};
	// 4. Compile the contract
	const tempFile = JSON.parse(solc.compile(JSON.stringify(input)));
	return tempFile.contracts['WatrDID.sol']['WatrDID'];
}

// 5. Export contract data
module.exports = loadDidContract();

