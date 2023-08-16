const setup = require('./setup');

const updateDid = async (context, ...args) => {
	const contract = setup(args[0][0]);
	const {did, controller, authentication, assertion, services} = args[0][1];

	try {
		const tx = await contract.updateDid(did, controller, authentication, assertion, services, {gasLimit: 250000});
		await tx.wait();
	} catch (e) {
		console.error(`\n⚠️  WARNING: Eth tx failed\n`, e);
	}
}

module.exports = updateDid;
