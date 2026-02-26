# Treasury Allocation Contract

The Treasury Allocation Contract acts as an access control and budgeting layer in front of the core `stellarcade-treasury` contract. 

## Features

- **Bucket Budgets**: Admins can define spending limits (budgets) for specific `bucket_id` tokens over a period.
- **Allocation Requests**: Any authorized user/contract can request an allocation of tokens out of a specific bucket.
- **Admin Approval Workflow**: An admin must explicitly approve requests.
- **Seamless Treasury Integration**: Upon approval, the allocation contract natively invokes the `treasury.allocate` method.

## Methods

- `init(admin, treasury_contract)`: Initialize the contract bindings limits.
- `create_budget(bucket_id, limit, period)`: Setup bucket constraints.
- `request_allocation(requester, bucket_id, amount, reason) -> u32`: Request tokens securely.
- `approve_allocation(request_id)`: Approves and disburses tokens against a valid request.
- `reject_allocation(request_id)`: Pre-emptively rejects a request.
- `budget_state(bucket_id)`: Fetches limits vs. allocations for visibility.
- `request_state(request_id)`: Fetches lifecycle state.

## Integration Dependencies

Relies entirely on `stellarcade-treasury` as the ultimate custodian; it assumes that `stellarcade-treasury` is already initialized, and that the `Treasury Allocation Contract` address has been set as the `admin` of the `stellarcade-treasury` to allow proxy token disbursement.
