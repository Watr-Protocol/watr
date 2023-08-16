const setup = require('./setup');

const revokeDidCredentials = async (context, ...args) => {
	const contract = setup(args[0][0]);
	const {issuerDid, subjectDid, credentials} = args[0][1];

	try {
		const tx = await contract.revokeCredentials(issuerDid, subjectDid, credentials, {gasLimit: 250000});
		await tx.wait();
	} catch (e) {
		console.error(`\n⚠️  WARNING: Eth tx failed\n`, e);
	}
}

module.exports = revokeDidCredentials;
