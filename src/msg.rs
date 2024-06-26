use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cosmwasm_std::Binary;
use cosmwasm_std::Coin;

use crate::state::GameState;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub authkey: String,
    pub owner: Addr
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReceiveNft {
        sender: String,
        token_id: String,
        msg: Binary,
    },
    StartRaffle {
        ticket_price: u64,
        total_ticket_count: u64,
        nft_contract_addr: Addr,
        nft_token_id: String,
        collection_wallet: Addr, // Collection wallet address to send tokens after the game finished
        end_time: u64,
    },
    EnterRaffle {
        game_id: u64
    },
    TransferTokensToCollectionWallet {
        amount: u128,
        denom: String,
        collection_wallet_address: String,
    },
    SelectWinnerAndTransferNFTtoWinner { game_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetGlobalInfo {},
    GetGameInfo { game_id: u64 },
    GetTicketsForWallet { game_id: u64, wallet_addr: Addr },
    GetAllGames {},
    GetBalance {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalResponse {
    pub raffle_count: u64,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameResponse {
    pub ticket_price: u64,
    pub sold_ticket_count: u64,
    pub total_ticket_count: u64,
    pub raffle_status: u8,
    pub nft_contract_addr: Addr,
    pub nft_token_id: String,
    pub owner: Addr,
    pub collection_wallet: Addr,
    pub end_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WalletTicketResponse {
    pub tickets: Vec<u64>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllGamesResponse {
    pub games: Vec<GameState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Coin,
}