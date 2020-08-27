use crate::prelude::*;
use alloc::vec::Vec;
use parity_scale_codec::{Decode, Encode};
pub fn permission_asset_definition_id() -> AssetDefinitionId {
    AssetDefinitionId::new("permissions", "global")
}

#[derive(Clone, Debug, Default, Encode, Decode)]
pub struct Permissions {
    pub origin: Vec<Permission>,
}

#[derive(Clone, Debug, Encode, Decode, PartialEq)]
pub enum Permission {
    /// Allows to do anything.
    Anything,
    /// Allows to add domains.
    AddDomain,
    /// Allows to add triggers.
    AddTrigger,
    /// Allows to register asset definition.
    RegisterAssetDefinition(Option<<Domain as Identifiable>::Id>),
    /// Allows to register account.
    RegisterAccount(Option<<Domain as Identifiable>::Id>),
    /// Allows to register asset.
    MintAsset(
        Option<<Domain as Identifiable>::Id>,
        Option<<AssetDefinition as Identifiable>::Id>,
    ),
    /// Allows to demint asset.
    DemintAsset(
        Option<<Domain as Identifiable>::Id>,
        Option<<AssetDefinition as Identifiable>::Id>,
    ),
    /// Allows to transfer asset.
    TransferAsset(
        Option<<Domain as Identifiable>::Id>,
        Option<<AssetDefinition as Identifiable>::Id>,
    ),
    /// Allows to add signatory.
    AddSignatory(
        Option<<Domain as Identifiable>::Id>,
        Option<<Account as Identifiable>::Id>,
    ),
    /// Allows to remove signatory.
    RemoveSignatory(
        Option<<Domain as Identifiable>::Id>,
        Option<<Account as Identifiable>::Id>,
    ),
}

impl Permissions {
    pub fn new() -> Self {
        Permissions::default()
    }

    pub fn single(permission: Permission) -> Self {
        Permissions {
            origin: vec![permission],
        }
    }
}

pub mod isi {
    use super::*;
    // use iroha_derive::Io;
    use parity_scale_codec::{Decode, Encode};

    /// Iroha special instructions related to `Permission`.
    #[derive(Clone, Debug, Encode, Decode)]
    pub enum PermissionInstruction {
        /// Should be able to do anything.
        CanAnything(<Account as Identifiable>::Id),
        /// Should be able to add triggers.
        CanAddTrigger(<Account as Identifiable>::Id),
        /// Should be able to add domain.
        CanAddDomain(<Account as Identifiable>::Id),
        /// Should be able to register account.
        CanRegisterAccount(
            <Account as Identifiable>::Id,
            Option<<Domain as Identifiable>::Id>,
        ),
        /// Should be able to register asset definition.
        CanRegisterAssetDefinition(
            <Account as Identifiable>::Id,
            Option<<Domain as Identifiable>::Id>,
        ),
        /// Should be able to transfer asset.
        CanTransferAsset(
            <Account as Identifiable>::Id,
            <AssetDefinition as Identifiable>::Id,
            Option<<Domain as Identifiable>::Id>,
        ),
        /// Should be able to add signatory.
        CanAddSignatory(
            <Account as Identifiable>::Id,
            <Account as Identifiable>::Id,
            Option<<Domain as Identifiable>::Id>,
        ),
        /// Should be able to remove signatory.
        CanRemoveSignatory(
            <Account as Identifiable>::Id,
            <Account as Identifiable>::Id,
            Option<<Domain as Identifiable>::Id>,
        ),
        /// Should be able to mint asset.
        CanMintAsset(
            <Account as Identifiable>::Id,
            <AssetDefinition as Identifiable>::Id,
            Option<<Domain as Identifiable>::Id>,
        ),
        /// Should be able to demint asset.
        CanDemintAsset(
            <Account as Identifiable>::Id,
            <AssetDefinition as Identifiable>::Id,
            Option<<Domain as Identifiable>::Id>,
        ),
    }

    impl From<&PermissionInstruction> for Permission {
        fn from(instruction: &PermissionInstruction) -> Self {
            match instruction {
                PermissionInstruction::CanAnything(_) => Permission::Anything,
                PermissionInstruction::CanAddDomain(_) => Permission::AddDomain,
                PermissionInstruction::CanAddTrigger(_) => Permission::AddTrigger,
                PermissionInstruction::CanRegisterAccount(_, option_domain_id) => {
                    Permission::RegisterAccount(option_domain_id.clone())
                }
                PermissionInstruction::CanRegisterAssetDefinition(_, option_domain_id) => {
                    Permission::RegisterAssetDefinition(option_domain_id.clone())
                }
                PermissionInstruction::CanTransferAsset(
                    _,
                    asset_definition_id,
                    option_domain_id,
                ) => Permission::TransferAsset(
                    option_domain_id.clone(),
                    Some(asset_definition_id.clone()),
                ),
                PermissionInstruction::CanAddSignatory(_, account_id, option_domain_id) => {
                    Permission::AddSignatory(option_domain_id.clone(), Some(account_id.clone()))
                }
                PermissionInstruction::CanRemoveSignatory(_, account_id, option_domain_id) => {
                    Permission::RemoveSignatory(option_domain_id.clone(), Some(account_id.clone()))
                }
                PermissionInstruction::CanMintAsset(_, asset_definition_id, option_domain_id) => {
                    Permission::MintAsset(
                        option_domain_id.clone(),
                        Some(asset_definition_id.clone()),
                    )
                }
                PermissionInstruction::CanDemintAsset(_, asset_definition_id, option_domain_id) => {
                    Permission::DemintAsset(
                        option_domain_id.clone(),
                        Some(asset_definition_id.clone()),
                    )
                }
            }
        }
    }
}
