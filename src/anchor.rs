//! A module for implementing Centrifuge document anchoring (merklized document commitments) on substrate for
//! Centrifuge chain.
//!
//! For a more formally detailed explanation refer section 3.4 of
//! [Centrifuge Protocol Paper](https://staticw.centrifuge.io/assets/centrifuge_os_protocol_paper.pdf)

use codec::{Decode, Encode};
use sp_core::H256;
use support::{decl_module, decl_storage};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// The data structure for storing committed anchors.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct AnchorData<Hash, BlockNumber> {
    pub id: Hash,
    pub doc_root: Hash,
    pub anchored_block: BlockNumber,
}

impl<Hash, BlockNumber> AnchorData<Hash, BlockNumber> {
    pub fn get_doc_root(self) -> Hash {
        self.doc_root
    }
}

/// The module's configuration trait.
pub trait Trait: system::Trait + pallet_timestamp::Trait + balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Anchor {
        Version: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

impl<T: Trait> Module<T> {
    /// Get an anchor by its id in the child storage
    pub fn get_anchor_by_id(_anchor_id: T::Hash) -> Option<AnchorData<T::Hash, T::BlockNumber>> {
        let result: AnchorData<T::Hash, T::BlockNumber> = Default::default();
        Some(result)
    }
}
