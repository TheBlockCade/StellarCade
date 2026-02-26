# vip-subscription

Definition of a VIP subscription plan.  `benefits_hash` is a 32-byte SHA-256 hash of the off-chain benefits document, providing a tamper-evident commitment without on-chain verbosity. `price` is the token amount charged per subscription period. `duration` is the subscription length in seconds.

## Public Methods

### `init`
Initialize the contract. May only be called once.  `admin` is the only address authorized to define plans. `treasury_contract` is the address that receives subscription payments.

```rust
pub fn init(env: Env, admin: Address, treasury_contract: Address) -> Result<(), Error>
```

### `define_plan`
Define a new VIP subscription plan. Admin only.  `plan_id` must be unique; re-defining an existing plan returns `PlanAlreadyExists`. `price` must be positive. `duration` must be positive (in seconds). `benefits_hash` is the 32-byte SHA-256 hash of the off-chain benefits document.

```rust
pub fn define_plan(
```

### `subscribe`
Subscribe `user` to `plan_id`. The user pays the plan price via the treasury contract.  Rejected if the user already has an active (non-expired) subscription. Use `renew` to extend an active subscription. A user whose subscription has already expired may call `subscribe` again to start fresh.

```rust
pub fn subscribe(env: Env, user: Address, plan_id: u32) -> Result<(), Error>
```

### `renew`
Renew `user`'s subscription to `plan_id`. The user pays the plan price.  If the subscription is still active the duration is added to the current `expires_at` (so renewals always stack). If already expired, the new expiry is `now + duration`. The plan_id in the record is updated to match the renewed plan (allowing cross-plan renewal).  Rejected if no subscription record exists for the user.

```rust
pub fn renew(env: Env, user: Address, plan_id: u32) -> Result<(), Error>
```

### `status_of`
Return the subscription status for `user`.  Returns a `SubscriptionStatus` with `has_subscription = false` if the user has never subscribed. If a record exists, `is_active` reflects whether the current ledger timestamp is before `expires_at`.

```rust
pub fn status_of(env: Env, user: Address) -> SubscriptionStatus
```

