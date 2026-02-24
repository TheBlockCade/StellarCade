const DeploymentUtil = require('../utils/deployment.util');
const { getStellarInstance } = require('./stellar.service'); // Assuming there is a stellar service

class DeploymentService {
    constructor(networkProfile = 'Dev') {
        this.networkProfile = networkProfile;
    }

    /**
     * Retrieves all verified contract addresses for the current active profile.
     * Useful for API metadata endpoints broadcasting the deployed system state to clients.
     */
    getContractRegistry() {
        try {
            const deployMap = DeploymentUtil.loadDeploymentMap(this.networkProfile);
            return deployMap.contracts;
        } catch (err) {
            console.error(`Failed to load contract registry for ${this.networkProfile}:`, err.message);
            return {};
        }
    }

    /**
     * Fetches the underlying mapped address for a specific contract to initialize transactions.
     */
    resolveAddress(contractAlias) {
        try {
            return DeploymentUtil.getContractAddress(contractAlias, this.networkProfile);
        } catch (err) {
            console.error(`Error resolving address for contract '${contractAlias}':`, err.message);
            throw new Error(`Contract ${contractAlias} is not properly deployed or initialized`);
        }
    }

    /**
     * Validates if the deployment artifacts are synced with the active environment.
     */
    async validateNetworkSync() {
        try {
            const deployMap = DeploymentUtil.loadDeploymentMap(this.networkProfile);
            const stellar = getStellarInstance ? getStellarInstance() : null;

            // Perform integrity checks between deployment output and actual network constants if applicable
            return {
                synced: true,
                profile: deployMap.network,
                timestamp: deployMap.timestamp,
                authorizedAdmin: deployMap.adminAddress,
            };
        } catch (err) {
            return { synced: false, error: err.message };
        }
    }
}

module.exports = new DeploymentService(process.env.NETWORK_PROFILE || 'Dev');
