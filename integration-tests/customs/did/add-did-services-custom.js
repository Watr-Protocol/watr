const setup = require('./setup');

const addDidServices = async (context, ...args) => {
	const contract = setup(args[0][0]);
	const {did, services} = args[0][1];

	try {
		const tx = await contract.addDidServices(did, services, {gasLimit: 250000});
		await tx.wait();
	} catch (e) {
		console.error(`\n⚠️  WARNING: Eth tx failed\n`, e);
	}
}

module.exports = addDidServices;
