use crate::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    system_contract_errors::auction::Error,
    CLTyped, Key, URef, U512,
};

/// Provides functionality of a contract storage.
pub trait StorageProvider {
    /// Compatible error type.
    type Error: From<Error>;

    /// Obtain named [`Key`].
    fn get_key(&mut self, name: &str) -> Option<Key>;

    /// Read data from [`URef`].
    fn read<T: FromBytes + CLTyped>(&mut self, uref: URef) -> Result<Option<T>, Self::Error>;

    /// Write data to [`URef].
    fn write<T: ToBytes + CLTyped>(&mut self, uref: URef, value: T) -> Result<(), Self::Error>;
}

/// Provides functionality of a mint contract.
pub trait MintProvider {
    /// Error representation for Mint errors.
    type Error: From<Error>;

    /// Bond specified amount from a purse.
    fn bond(&mut self, amount: U512, purse: URef) -> Result<(URef, U512), Self::Error>;

    /// Unbond specified amount.
    fn unbond(&mut self, amount: U512) -> Result<(URef, U512), Self::Error>;

    /// Sets the Bool field in the tuple representing a founding validator's stake to True, enabling this validator to unbond.
    fn release_founder_stake(&mut self, account_hash: AccountHash) -> Result<bool, Self::Error>;
}

/// Provides functionality of a system.
pub trait SystemProvider {
    /// Error represntation for system errors.
    type Error: From<Error>;

    /// Creates new purse.
    fn create_purse(&mut self) -> URef;

    /// Gets purse balance.
    fn get_balance(&mut self, purse: URef) -> Result<Option<U512>, Self::Error>;

    /// Transfers specified `amount` of tokens from `source` purse into a `target` purse.
    fn transfer_from_purse_to_purse(
        &mut self,
        source: URef,
        target: URef,
        amount: U512,
    ) -> Result<(), Self::Error>;
}