# NFT Reward System Contract

The `nft-reward` contract manages the lifecycle of NFT reward campaigns on the StellarCade platform. It allows administrators to define reward campaigns with metadata commitments and supply limits, and provides a secure mechanism for users to claim their earned NFTs.

## Public Interface

### `init(admin: Address, nft_contract: Address, reward_contract: Address)`
Initializes the contract with the primary administrator and the addresses of the NFT collection and general reward contracts.

### `define_nft_reward(campaign_id: u32, metadata_uri: String, supply: u32)`
Defines a new NFT reward campaign.
- **`campaign_id`**: A unique identifier for the campaign.
- **`metadata_uri`**: The URI (e.g., IPFS link) for the NFT metadata.
- **`supply`**: The maximum number of NFTs that can be awarded in this campaign.

### `mint_reward(user: Address, campaign_id: u32)`
Awards an NFT reward to a specific user for a given campaign.
- This decrements the campaign's remaining supply.
- Sets a `PendingReward` flag for the user.

### `claim_nft(user: Address, campaign_id: u32)`
Allows a user to claim their pending NFT.
- Verifies the `PendingReward` flag.
- Sets the `Claimed` status to prevent double-claiming.
- In production, this interacts with an external NFT contract to execute the minting logic.

### `nft_reward_state(campaign_id: u32) -> Option<CampaignData>`
Returns the current configuration and status of a campaign.

## Storage Strategy

- **Instance Storage**: Stores global configuration (`Admin`, `NftContract`, `RewardContract`).
- **Persistent Storage**:
    - **`Campaign`**: Full campaign data including metadata URI and remaining supply.
    - **`PendingReward`**: Tracks users who have earned but not yet claimed their rewards.
    - **`Claimed`**: Tracks completed claims to ensure invariants hold.

## Security & Invariants

- **Authorization**: All administrative functions (`define_nft_reward`, `mint_reward`) require admin authentication.
- **Supply Integrity**: The contract ensures that the total number of minted/pending rewards never exceeds the defined supply.
- **Reentrancy & Double-Claiming**: The `Claimed` flag is set before external contract calls to prevent race conditions.
