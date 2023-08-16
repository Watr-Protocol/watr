const setup = require('./setup');

const removeDid = async (context, ...args) => {
	const contract = setup(args[0][0]);
	const {did} = args[0][1];

	try {
		const tx = await contract.removeDid(did, {gasLimit: 250000});
		await tx.wait();
	} catch (e) {
		console.error(`\n⚠️  WARNING: Eth tx failed\n`, e);
	}
}

module.exports = removeDid;
