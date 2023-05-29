//! This file is part of substrate in bits. 
//! Use in accordance to MIT licence.
//! 
//! This file contains failing code to demonstrate difference between common hashing algoritms in substrate.

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::inherent::Vec;
    use frame_support::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
    }

    /// Book summary
    #[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct BookSummary<AccountId, BlockNumber> {
        title: Vec<u8>,         // title of book
        author: Vec<u8>,        // author of book
        url: Vec<u8>,           // web url to off-chain storage
        archiver: AccountId,    // account id of archiver
        timestamp: BlockNumber, // time when book was archived
    }

    /// Archive storage
    ///
    /// Maps a book hash to book summary
    /// Book hash is Blake2 hash of book title and author
    #[pallet::storage]
    pub(super) type ArchiveStore<T: Config> = StorageMap<
        _,
        Blake2_128Concat, // NOTE: Twox64Concat is not a correct hasher for the case
        T::Hash,
        BookSummary<T::AccountId, T::BlockNumber>,
        OptionQuery,
    >;
}

