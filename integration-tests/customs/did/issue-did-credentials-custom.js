const setup = require('./setup');

const addDidServices = async (context, ...args) => {
	const contract = setup(args[0][0]);
	const {issuerDid, subjectDid, credentials, verifiableCredentialHash} = args[0][1];

	try {
		const tx = await contract.issueCredentials(issuerDid, subjectDid, credentials, verifiableCredentialHash, {gasLimit: 250000});
		await tx.wait();
	} catch (e) {
		console.error(`\n⚠️  WARNING: Eth tx failed\n`, e);
	}
}

module.exports = addDidServices;
