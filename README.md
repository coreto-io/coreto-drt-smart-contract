Audited with [Rustle](https://github.com/blocksecteam/rustle), the first automatic auditor for NEAR Community

# Smart Contract Functions

```rust
pub struct CoretoTPActionRequestData {
	trust: f32, // The trust value resulted from the action
	performance: f32, // The performance value resulted from the action
	action_type: String, // The type of the action
	action_date: String, // The date of the action (UNIX timestamp in ms)
    account_did: String, // The DID of the person that did the action
    identifier: String // Internal identifier used by each source to check if the action was synced
}

pub fn save_actions_batch(
    &mut self,
    batch: Vec<CoretoTPActionRequestData>,
)
```

```rust
pub fn save_action(
    &mut self,
    account_did: String,
    trust: f32,
    performance: f32,
    action_type: String,
    action_date: String,
    identifier: String,
)
```

```rust
pub fn get_user_actions(
    &self,
    source_label: String,
    account_id: AccountId
) -> Vec<CoretoTPAction>
```

```rust
pub fn get_user_trust_actions(
    &self,
    source_label: String,
    account_did: String
) -> Vec<CoretoTPAction>
```

```rust
pub fn get_user_performance_actions(
    &self,
    source_label: String,
    account_did: String
) -> Vec<CoretoTPAction>
```

```rust
pub fn get_source_action_types(
    &self,
    source: AccountId
) -> Vec<String>
```

```rust
pub fn get_user_trust(
    &self,
    source_label: String,
    account_did: String
) -> f32
```

```rust
pub fn get_user_performance(
    &self,
    source_label: String,
    account_did: String
) -> f32
```

```rust
pub fn add_source(
    &mut self,
    source: AccountId,
    source_label: String
)
```

```rust
pub fn remove_source(
    &mut self,
    source: AccountId
)
```

# Run tests

`cargo test -- --nocapture`

# Compile

`cargo build --target wasm32-unknown-unknown --release`

# Deploy (regular)

```
near login
near deploy --wasmFile target/wasm32-unknown-unknown/release/coreto_trust_performance_ledger.wasm --accountId YOUR_ACCOUNT_HERE
```

# Call

```
near call YOUR_ACCOUNT_HERE METHOD_NAME METHOD_ARGUMENTS --accountId YOUR_ACCOUNT_HERE
near view YOUR_ACCOUNT_HERE GETTER_METHOD_NAME METHOD_ARGUMENTS --accountId YOUR_ACCOUNT_HERE
```

# Deploy (dev account)
```
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/coreto_trust_performance_ledger.wasm --helperUrl https://near-contract-helper.onrender.com
source neardev/dev-account.env
echo $CONTRACT_NAME
```

# Call
```
near call $CONTRACT_NAME METHOD_NAME METHOD_ARGUMENTS --accountId $CONTRACT_NAME
near view $CONTRACT_NAME GETTER_METHOD_NAME METHOD_ARGUMENTS --accountId $CONTRACT_NAME
```
