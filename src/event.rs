use near_sdk::AccountId;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Debug)]
#[serde(tag = "standard")]
#[serde(rename_all = "snake_case")]
pub enum NearEvent<'a> {
    #[serde(borrow)]
    Nep171(Nep171Event<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nep171Event<'a> {
    pub version: &'static str,
    #[serde(flatten)]
    #[serde(borrow)]
    pub event_kind: Nep171EventKind<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum Nep171EventKind<'a> {
    #[serde(borrow)]
    NftMint(Vec<NftMintData<'a>>),
    #[serde(borrow)]
    NftTransfer(Vec<NftTransferData<'a>>),
    #[serde(borrow)]
    NftBurn(Vec<NftBurnData<'a>>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct NftMintData<'a> {
    #[serde(borrow)]
    pub owner_id: &'a str,
    #[serde(borrow)]
    pub token_ids: Vec<&'a str>,
    #[serde(borrow)]
    pub memo: Option<&'a str>,
}

impl<'a> NftMintData<'a> {
    pub fn new(
        owner_id: &'a AccountId,
        token_ids: Vec<&'a str>,
        memo: Option<&'a str>,
    ) -> NftMintData<'a> {
        Self {
            owner_id: owner_id.as_str(),
            token_ids,
            memo,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct NftTransferData<'a> {
    #[serde(borrow)]
    pub old_owner_id: &'a str,
    #[serde(borrow)]
    pub new_owner_id: &'a str,
    #[serde(borrow)]
    pub token_ids: Vec<&'a str>,
    #[serde(borrow)]
    pub authorized_id: Option<&'a str>,
    #[serde(borrow)]
    pub memo: Option<&'a str>,
}

impl<'a> NftTransferData<'a> {
    pub fn new(
        old_owner_id: &'a AccountId,
        new_owner_id: &'a AccountId,
        token_ids: Vec<&'a str>,
        authorized_id: Option<&'a AccountId>,
        memo: Option<&'a str>,
    ) -> NftTransferData<'a> {
        Self {
            authorized_id: authorized_id.map(|id| id.as_str()),
            old_owner_id: old_owner_id.as_str(),
            new_owner_id: new_owner_id.as_str(),
            token_ids,
            memo,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct NftBurnData<'a> {
    #[serde(borrow)]
    pub owner_id: &'a str,
    #[serde(borrow)]
    pub token_ids: Vec<&'a str>,
    #[serde(borrow)]
    pub authorized_id: Option<&'a str>,
    #[serde(borrow)]
    pub memo: Option<&'a str>,
}

impl<'a> NftBurnData<'a> {
    pub fn new(
        owner_id: &'a AccountId,
        token_ids: Vec<&'a str>,
        authorized_id: Option<&'a AccountId>,
        memo: Option<&'a str>,
    ) -> NftBurnData<'a> {
        Self {
            owner_id: owner_id.as_str(),
            token_ids,
            authorized_id: authorized_id.map(|id| id.as_str()),
            memo,
        }
    }
}

impl<'a> NearEvent<'a> {
    pub fn new_171(version: &'static str, event_kind: Nep171EventKind<'a>) -> Self {
        NearEvent::Nep171(Nep171Event {
            version,
            event_kind,
        })
    }

    pub fn new_171_v1(event_kind: Nep171EventKind<'a>) -> Self {
        NearEvent::new_171("1.0.0", event_kind)
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn nft_burn(data: Vec<NftBurnData<'a>>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftBurn(data))
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn nft_transfer(data: Vec<NftTransferData<'a>>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftTransfer(data))
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn nft_mint(data: Vec<NftMintData<'a>>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftMint(data))
    }

    pub(crate) fn to_json_string(&self) -> String {
        near_sdk::serde_json::to_string(self).unwrap()
    }

    pub fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }

    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        near_sdk::env::log_str(&self.to_json_event_string());
    }
}
