cfg_if::cfg_if! {
    if #[cfg(feature = "legacy")]{
        pub use solana_program::account_info::AccountInfo;
        pub use solana_program::program_error::ProgramError;
        pub use solana_program::pubkey::Pubkey;
    } else {
        // TODO: Redefine types so the crate can be minimal dependency.
    }
}
