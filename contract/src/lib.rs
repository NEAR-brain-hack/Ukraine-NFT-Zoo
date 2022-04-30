use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use serde::{Serialize, Deserialize};
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, Balance
};
use near_sdk::json_types::{U128};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    total_supply: u64
}

const ONE_NEAR_ES_YOCTO: Balance = 1_000_000_000_000_000_000_000_000;

const MEAT: &str = "https://bafybeifj7myffnjjpkid2275saz6q53oys4wy5w3rc5vxcgethtkmi6wz4.ipfs.dweb.link/meat.png";
const VEGETABLE: &str = "https://bafybeifguxtp5gjpaknbi5kglrx3lzz55vc2zpqtugzoygmaxslqelyqkq.ipfs.dweb.link/vegetable.png";
const ZOOKEEPER: &str = "https://bafybeigo6k3qazqewnrnquf3kxvmrr6tkficz6mdhicv4letocpcsy23ju.ipfs.dweb.link/zookeeper.png";
const TICKET: &str = "https://bafybeigvf7oewzdg3owvscp2s22fpyw7uzelzwcaxjqa5qaiosjmeldpom.ipfs.dweb.link/ticket.png";
const RICE: &str = "https://bafybeigw73ya5j2n3ujy4qfg3lwsowz4mitcfn2xqr63a4v3obbre22lvi.ipfs.dweb.link/rice.png";


#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[derive(Serialize ,Deserialize, BorshSerialize, BorshStorageKey)]
pub enum Donation {
    Meat,
    Vegetable,
    Ticket,
    ZooKeeper,
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            total_supply: 0
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let this: Self = env::state_read().expect("Cannot deserialize");

        assert_eq!(
            env::predecessor_account_id(),
            this.tokens.owner_id,
            "Only owner"
        );

        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                this.tokens.owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: this.metadata,
            total_supply: this.total_supply
        }
    }

    // One person could create only one nft which token's id is user's account_id
    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: AccountId,
        donation_type: Donation
    ) -> Token {
        let deposit_value = env::attached_deposit();
        let token_metadata_base = match donation_type {
            Donation::Meat => {
                (
                    "MEAT FOR LION".to_string(),
                    MEAT.to_string(),
                    "Thanks for your donation".to_string(),
                    4u128
                )
            },
            Donation::Vegetable => {
                (
                    "VEGETABLE FOR RABBIT".to_string(),
                    VEGETABLE.to_string(),
                    "Thanks for your donation".to_string(),
                    2u128
                )
            },
            Donation::Ticket => {
                (
                    "TICKET".to_string(),
                    TICKET.to_string(),
                    "Thanks for your donation".to_string(),
                    3u128
                )
            },
            Donation::ZooKeeper => {
                (
                    "DONATE FOR ZOOKEEPER".to_string(),
                    ZOOKEEPER.to_string(),
                    "Thanks for your donation".to_string(),
                    2u128
                )
            }
        };
        assert!(deposit_value == token_metadata_base.3 * ONE_NEAR_ES_YOCTO);
        let metadata = TokenMetadata {
            title: Some(token_metadata_base.0),
            media: Some(token_metadata_base.1),
            description: Some(token_metadata_base.2),
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };
        self.total_supply += 1;
        self.tokens.internal_mint(self.total_supply.to_string(), receiver_id, Some(metadata))
    }

    pub fn withdraw(&mut self, receiver: AccountId, amount: U128) -> Promise {
        assert_eq!(
            receiver,
            self.tokens.owner_id
        );

        Promise::new(receiver).transfer(amount.into())
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
