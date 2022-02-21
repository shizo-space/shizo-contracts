use crate::*;
use near_sdk::{ext_contract, Gas};
use near_sdk::env::STORAGE_PRICE_PER_BYTE;

// use ed25519_dalek::Signature;
// use ed25519_dalek::{PublicKey, Verifier};

// 1 Ⓝ in yoctoNEAR
const PRICE: u128 = 10_000_000_000_000_000_000_000_000;
// const OWNER_PUBLIC_KEY: &str = "Fwst6GVTdcdZN6CyZkPdvmpKnffcURxBUhUVppgQ5UWG";
const SHIZO_MEDIA: &str = "https://bafkreidqcnv5kkajvdtkn62r4asevqge3i5xtuno4dbzkbro7kerafqsu4.ipfs.dweb.link";
const ROYALITY_RECEIVER: &str = "shizotest.testnet";
const ROYALITY_PERCENTAGE: u32 = 500; // 5%
const STORAGE_PER_SALE: u128 = 1000 * STORAGE_PRICE_PER_BYTE;
const GAS_FOR_STORAGE_DEPOSIT: Gas = Gas(15_000_000_000_000);
const MARKET_ACCOUNT: &str = "market.shizotest.testnet";


#[ext_contract(ext_storage_deposit)]
trait MarketStorageDeposit {
    //cross contract call to an external contract that is initiated during mintint to deposit storage
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>
    );
}


#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        // signature: String, // signature of the token ID signed by the owner
    ) -> Promise {

        // TODO: check that the token token_id is mintable
        // // decode owner public key using base58
        // let decoded_pb: [u8; 32] = bs58::decode(OWNER_PUBLIC_KEY).into_vec().unwrap().as_slice().try_into().unwrap();
        // let public_key = PublicKey::from_bytes(&decoded_pb).unwrap();

        // // decode signature using base58
        // let encoded_signature: [u8;64] = bs58::decode(signature).into_vec().unwrap().as_slice().try_into().unwrap();
        // let signature = Signature::from_bytes(&encoded_signature).unwrap();
        // assert!(public_key.verify(token_id.as_bytes(), &signature).is_ok(), "Signature and tokenId not matched");

        let deposit = env::attached_deposit();

        assert!(
            deposit == PRICE,
            "Must attach exactly {} yoctoNEAR to cover storage + price",
            PRICE,
        );

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        // add the owner to the royalty map
        let royalty_receiver: AccountId = ROYALITY_RECEIVER.parse().unwrap();
        royalty.insert(royalty_receiver, ROYALITY_PERCENTAGE);

        //specify the token struct that contains the owner ID
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        let metadata: TokenMetadata = TokenMetadata {
            title: Some(format!("ShizoTest #{}", token_id)),
            media: Some(String::from(SHIZO_MEDIA)),
            description: None,
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

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        let market: AccountId = MARKET_ACCOUNT.parse().unwrap();

        ext_storage_deposit::storage_deposit(
            Some(token.owner_id),
            market, //contract account we're calling
            STORAGE_PER_SALE, //NEAR deposit we attach to the call
            GAS_FOR_STORAGE_DEPOSIT, //GAS we're attaching
        )
    }
}