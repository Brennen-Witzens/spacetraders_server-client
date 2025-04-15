use super::payment::Payment;
use async_graphql::{Enum, Object};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Enum, Copy, Clone, Eq, PartialEq, Deserialize, Debug)]
pub enum ContractType {
    PROCUREMENT,
    TRANSPORT,
    SHUTTLE,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeliver {
    trade_symbol: String,
    destination_symbol: String,
    units_required: i32,
    units_fulfilled: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerms {
    deadline: String,
    payment: Payment,
    deliver: Option<Vec<ContractDeliver>>,
}

#[Object]
impl ContractTerms {
    async fn deadline(&self) -> &str {
        &self.deadline
    }
    async fn payment(&self) -> &Payment {
        &self.payment
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    id: String,
    faction_symbol: String,
    #[serde(rename = "type")]
    contract_type: ContractType,
    terms: ContractTerms,
    accepted: bool,
    fulfilled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline_to_accept: Option<String>,
}

#[Object]
impl Contract {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn faction_symbol(&self) -> &str {
        &self.faction_symbol
    }
    async fn contract_type(&self) -> &ContractType {
        &self.contract_type
    }
    async fn accepted(&self) -> bool {
        self.accepted
    }
    async fn fulfilled(&self) -> bool {
        self.fulfilled
    }
    async fn deadline_to_accept(&self) -> Option<&str> {
        self.deadline_to_accept.as_deref()
    }
}
