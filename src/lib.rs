use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector, UnorderedSet};
use near_sdk::{
    env,
    require,
    assert_self,
    near_bindgen,
    AccountId,
    BorshStorageKey
};
use serde::{Serialize, Deserialize};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKeys {
    TPInfosKey,
    SourcesKey,
    ActionsKey,
    SourceActionsKey,
    SourceActionsSetKey,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct CoretoTPActionRequestData {
	trust: f32,
	performance: f32,
	action_type: String,
	action_date: String,
    account_did: String,
    identifier: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct CoretoTPAction {
	trust: f32,
	performance: f32,
	action_type: String,
	action_date: String,
	block_date: String,
	source_label: String,
    source: AccountId,
    identifier: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CoretoTPStat {
    actions: Vector<CoretoTPAction>
}

impl CoretoTPStat {
    fn default() -> Self {
        Self {
            actions: Vector::new(StorageKeys::ActionsKey),
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CoretoTPLedger {
    tp_infos: LookupMap<String, CoretoTPStat>,
    sources: LookupMap<AccountId, String>,
    source_actions: LookupMap<AccountId, UnorderedSet<String>>,
}

impl Default for CoretoTPLedger {
    fn default() -> Self {
        Self {
            tp_infos: LookupMap::new(StorageKeys::TPInfosKey),
            sources: LookupMap::new(StorageKeys::SourcesKey),
            source_actions: LookupMap::new(StorageKeys::SourceActionsKey),
        }
    }
}


#[near_bindgen]
impl CoretoTPLedger {
	pub fn save_actions_batch(
        &mut self,
        batch: Vec<CoretoTPActionRequestData>,
    ) {
		require!(
			self.sources.contains_key(&env::signer_account_id()),
			"Invalid signer wallet."
		);

        let mut source_action = if self.source_actions.contains_key(&env::signer_account_id()) {
            self.source_actions
                .get(&env::signer_account_id())
                .unwrap()
        } else {
            UnorderedSet::new(StorageKeys::SourceActionsSetKey)
        };

        for data in batch.iter() {
            let action = CoretoTPAction {
                trust: data.trust,
                performance: data.performance,
                action_type: data.action_type.clone(),
                action_date: data.action_date.clone(),
                identifier: data.identifier.clone(),
                block_date: env::block_timestamp().to_string(),
                source_label: self.sources.get(&env::signer_account_id()).unwrap(),
                source: env::signer_account_id(),
            };

            let mut tp_info = self.tp_infos.get(&data.account_did)
                .unwrap_or(CoretoTPStat::default());

            tp_info.actions.push(&action);
            source_action.insert(&action.action_type);

            self.tp_infos.insert(&data.account_did, &tp_info);
        }

        self.source_actions.insert(&env::signer_account_id(), &source_action);
	}

	pub fn save_action(
        &mut self,
        account_did: String,
        trust: f32,
        performance: f32,
        action_type: String,
        action_date: String,
        identifier: String,
    ) {
		require!(
			self.sources.contains_key(&env::signer_account_id()),
			"Invalid signer wallet."
		);

        let action = CoretoTPAction {
            trust,
            performance,
            action_type,
            action_date,
            identifier,
            block_date: env::block_timestamp().to_string(),
            source_label: self.sources.get(&env::signer_account_id()).unwrap(),
            source: env::signer_account_id(),
        };

        let mut tp_info = if self.tp_infos.contains_key(&account_did) {
            self.tp_infos.get(&account_did).unwrap()
        } else {
            CoretoTPStat::default()
        };

        tp_info.actions.push(&action);
        self.tp_infos.insert(&account_did, &tp_info);

        let mut source_action = if self.source_actions.contains_key(&env::signer_account_id()) {
            self.source_actions
                .get(&env::signer_account_id())
                .unwrap()
        } else {
            UnorderedSet::new(
                StorageKeys::SourceActionsSetKey
            )
        };

        source_action.insert(&action.action_type);
        self.source_actions.insert(&env::signer_account_id(), &source_action);
	}

	pub fn get_user_actions(&self, source_label: String, account_did: String) -> Vec<CoretoTPAction> {
		require!(self.tp_infos.contains_key(&account_did), "AccountDID not found.");

		let infos : CoretoTPStat = self.tp_infos.get(&account_did).unwrap();

        return infos.actions
            .to_vec()
            .into_iter()
            .filter(|action| action.source_label == source_label)
            .collect();
	}

    pub fn get_user_trust_actions(&self, source_label: String, account_did: String) -> Vec<CoretoTPAction> {
		require!(self.tp_infos.contains_key(&account_did), "AccountDID not found.");

		let infos : CoretoTPStat = self.tp_infos.get(&account_did).unwrap();

        return infos.actions
            .to_vec()
            .into_iter()
            .filter(|action| action.source_label == source_label && action.trust > 0.0)
            .collect();
	}

    pub fn get_user_performance_actions(&self, source_label: String, account_did: String) -> Vec<CoretoTPAction> {
		require!(self.tp_infos.contains_key(&account_did), "AccountDID not found.");

		let infos : CoretoTPStat = self.tp_infos.get(&account_did).unwrap();

        return infos.actions
            .to_vec()
            .into_iter()
            .filter(|action| action.source_label == source_label && action.performance > 0.0)
            .collect();
	}

    pub fn get_source_action_types(&self, source: AccountId) -> Vec<String> {
		require!(
			self.source_actions.contains_key(&source),
			"Source not found."
		);

        return self.source_actions.get(&source).unwrap().to_vec();
	}

    pub fn get_user_trust(&self, source_label: String, account_did: String) -> f32 {
        env::log_str(&format!("{} {}", source_label, account_did.to_string()).to_string());
		return 0.0;
	}

    pub fn get_user_performance(&self, source_label: String, account_did: String) -> f32 {
        env::log_str(&format!("{} {}", source_label, account_did.to_string()).to_string());
		return 0.0;
	}

	pub fn add_source(&mut self, source: AccountId, source_label: String) {
		assert_self();
		require!(
			!self.sources.contains_key(&source),
			"Source already exists."
		);

		self.sources.insert(&source, &source_label);
	}

	pub fn remove_source(&mut self, source: AccountId) {
		assert_self();
		require!(
			self.sources.contains_key(&source),
			"Source not found."
		);

		self.sources.remove(&source);
	}
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    use super::*;

    // Allows for modifying the environment of the mocked blockchain
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn get_contact_with_mocked_source_and_action(source_label: String, action: String) -> CoretoTPLedger {
        let mut context = get_context(accounts(0));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();

        contract.add_source(
            accounts(1),
            source_label.to_string(),
        );

        testing_env!(
            context
                .signer_account_id(accounts(1))
                .build()
        );

        contract.save_action(
            "did:mock:accounts(2)".to_string(),
            10.0,
            10.0,
            action.to_string(),
            "1640995200".to_string(),
            "123".to_string(),
        );

        return contract;
    }

    #[test]
    fn get_user_trust() {
        let context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());


        let contract = CoretoTPLedger::default();
        let trust = contract.get_user_trust(
            "coreto".to_string(),
            "did:mock:accounts(1)".to_string(),
        );
        assert_eq!(trust, 0.0);
    }

    #[test]
    fn get_user_performance() {
        let context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let contract = CoretoTPLedger::default();
        let trust = contract.get_user_performance(
            "coreto".to_string(),
            "did:mock:accounts(1)".to_string(),
        );
        assert_eq!(trust, 0.0);
    }

    #[test]
    #[should_panic(expected = r#"Invalid signer wallet."#)]
    fn save_action_no_sources() {
        let context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();
        contract.save_action(
            "did:mock:accounts(1)".to_string(),
            10.0,
            10.0,
            "reaction".to_string(),
            "1640995200".to_string(),
            "123".to_string(),
        );
    }

    #[test]
    fn save_action() {
        let mut context = get_context(accounts(0));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();

        contract.add_source(
            accounts(1),
            "coreto_website".to_string(),
        );

        testing_env!(
            context
                .signer_account_id(accounts(1))
                .build()
        );

        contract.save_action(
            "did:mock:accounts(1)".to_string(),
            10.0,
            10.0,
            "reaction".to_string(),
            "1640995200".to_string(),
            "123".to_string(),
        );
    }

    #[test]
    fn save_actions_batch() {
        let mut context = get_context(accounts(0));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();

        contract.add_source(
            accounts(1),
            "coreto_website".to_string(),
        );

        testing_env!(
            context
                .signer_account_id(accounts(1))
                .build()
        );

        let actions_batch: Vec<CoretoTPActionRequestData> = vec![
            CoretoTPActionRequestData {
                action_type: "reaction".to_string(),
                performance: 10.0,
                trust: 10.0,
                action_date: "1640995200".to_string(),
                account_did: "did:mock:accounts(3)".to_string(),
                identifier: "123".to_string(),
            },
            CoretoTPActionRequestData {
                action_type: "reaction".to_string(),
                performance: 11.0,
                trust: 10.0,
                action_date: "1640995200".to_string(),
                account_did: "did:mock:accounts(3)".to_string(),
                identifier: "124".to_string(),
            },
            CoretoTPActionRequestData {
                action_type: "reaction".to_string(),
                performance: 11.0,
                trust: 11.0,
                action_date: "1640995200".to_string(),
                account_did: "did:mock:accounts(4)".to_string(),
                identifier: "125".to_string(),
            },
        ];

        contract.save_actions_batch(
            actions_batch,
        );

        let mut account_3_actions: Vec<CoretoTPAction> = contract.get_user_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(3)".to_string(),
        );

        let mut account_4_actions: Vec<CoretoTPAction> = contract.get_user_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(4)".to_string(),
        );

        assert_eq!(account_3_actions.len(), 2);
        assert_eq!(account_4_actions.len(), 1);



        let actions_batch2: Vec<CoretoTPActionRequestData> = vec![
            CoretoTPActionRequestData {
                action_type: "reaction2".to_string(),
                performance: 10.0,
                trust: 10.0,
                action_date: "1640995200".to_string(),
                account_did: "did:mock:accounts(3)".to_string(),
                identifier: "123".to_string(),
            },
            CoretoTPActionRequestData {
                action_type: "reaction2".to_string(),
                performance: 11.0,
                trust: 10.0,
                action_date: "1640995200".to_string(),
                account_did: "did:mock:accounts(3)".to_string(),
                identifier: "124".to_string(),
            },
            CoretoTPActionRequestData {
                action_type: "reaction2".to_string(),
                performance: 11.0,
                trust: 11.0,
                action_date: "1640995200".to_string(),
                account_did: "did:mock:accounts(4)".to_string(),
                identifier: "125".to_string(),
            },
        ];

        contract.save_actions_batch(
            actions_batch2,
        );

        account_3_actions = contract.get_user_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(3)".to_string(),
        );

        account_4_actions = contract.get_user_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(4)".to_string(),
        );

        assert_eq!(account_3_actions.len(), 4);
        assert_eq!(account_4_actions.len(), 2);
    }

    #[test]
    #[should_panic(expected = r#"Source not found."#)]
    fn get_source_action_types_no_source() {
        let contract = get_contact_with_mocked_source_and_action(
            "coreto_website".to_string(),
            "reaction".to_string(),
        );

        contract.get_source_action_types(accounts(3));
    }

    #[test]
    fn get_source_action_types() {
        let contract = get_contact_with_mocked_source_and_action(
            "coreto_website".to_string(),
            "reaction".to_string(),
        );

        let action_types = contract.get_source_action_types(accounts(1));
        assert_eq!(action_types.len(), 1);
        assert_eq!(action_types[0], "reaction");
    }

    #[test]
    fn get_source_action_types_multiple_types() {
        let mut contract = get_contact_with_mocked_source_and_action(
            "coreto_website".to_string(),
            "reaction".to_string(),
        );

        contract.save_action(
            "did:mock:accounts(1)".to_string(),
            10.0,
            10.0,
            "article".to_string(),
            "1640995200".to_string(),
            "123".to_string(),
        );

        let action_types = contract.get_source_action_types(accounts(1));
        assert_eq!(action_types.len(), 2);
        assert_eq!(action_types[0], "reaction");
        assert_eq!(action_types[1], "article");
    }

    #[test]
    fn get_user_trust_actions() {
        let contract = get_contact_with_mocked_source_and_action(
            "coreto_website".to_string(),
            "reaction-trust".to_string(),
        );

        let trust_actions = contract.get_user_trust_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(2)".to_string(),
        );

        assert_eq!(trust_actions.len(), 1);
        assert_eq!(trust_actions[0].action_type, "reaction-trust");
        assert_eq!(trust_actions[0].trust, 10.0);
    }

    #[test]
    #[should_panic(expected = r#"AccountDID not found."#)]
    fn get_user_trust_actions_no_actions() {
        let contract = get_contact_with_mocked_source_and_action(
            "coreto_website".to_string(),
            "reaction-trust".to_string(),
        );

        contract.get_user_trust_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(3)".to_string(),
        );
    }

    #[test]
    fn get_user_performance_actions() {
        let contract = get_contact_with_mocked_source_and_action(
            "coreto_website".to_string(),
            "reaction-performance".to_string(),
        );

        let performance_actions = contract.get_user_performance_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(2)".to_string(),
        );
        assert_eq!(performance_actions.len(), 1);
        assert_eq!(performance_actions[0].action_type, "reaction-performance");
        assert_eq!(performance_actions[0].performance, 10.0);
    }

    #[test]
    #[should_panic(expected = r#"AccountDID not found."#)]
    fn get_user_performance_actions_no_actions() {
        let contract = get_contact_with_mocked_source_and_action(
            "coreto_website".to_string(),
            "reaction-performance".to_string(),
        );

        contract.get_user_performance_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(3)".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = r#"AccountDID not found."#)]
    fn get_user_actions_no_actions() {
        let context = get_context(accounts(0));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let contract = CoretoTPLedger::default();

        contract.get_user_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(1)".to_string(),
        );
    }

    #[test]
    fn get_user_actions() {
        let contract = get_contact_with_mocked_source_and_action(
            "coreto_website".to_string(),
            "reaction".to_string(),
        );

        let actions: Vec<CoretoTPAction> = contract.get_user_actions(
            "coreto_website".to_string(),
            "did:mock:accounts(2)".to_string(),
        );

        assert_eq!(actions.len(), 1);
    }

    #[test]
    #[should_panic(expected = r#"Method is private"#)]
    fn add_source_not_owner() {
        let context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();
        contract.add_source(
            accounts(1),
            "coreto_website".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = r#"Source already exists."#)]
    fn add_source_already_exists() {
        let context = get_context(accounts(0));

        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();
        contract.add_source(
            accounts(1),
            "coreto_website".to_string(),
        );

        contract.add_source(
            accounts(1),
            "coreto_website".to_string(),
        );
    }

    #[test]
    fn add_source() {
        let context = get_context(accounts(0));

        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();
        contract.add_source(
            accounts(1),
            "coreto_website".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = r#"Method is private"#)]
    fn remove_source_not_owner() {
        let context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();
        contract.remove_source(accounts(1));
    }

    #[test]
    #[should_panic(expected = r#"Source not found."#)]
    fn remove_source_not_found() {
        let context = get_context(accounts(0));

        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();
        contract.remove_source(accounts(1));
    }

    #[test]
    fn remove_source() {
        let context = get_context(accounts(0));

        testing_env!(context.build());

        let mut contract = CoretoTPLedger::default();
        contract.add_source(
            accounts(1),
            "coreto_website".to_string(),
        );

        contract.remove_source(accounts(1));
    }
}
