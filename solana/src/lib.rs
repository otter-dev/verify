//! > **Formal Verification for SOLANA Programs**
//!
//! ## Invariants
//!
//! Invariants are properties that should always be true.  For example, the balance of a token account should never be negative. There are two types of invariants in the Solana programs: `account invariants` and `instruction invariants`.
//!
//! ### Instruction Invariants
//!   An instruction invariant specifies sufficient conditions for an instruction to succeed (or fail). These are specified as `succeeds_if` or `errors_if` macro annotations on the instruction handler.
//!   - `succeeds_if` - The instruction should succeed if and only if the given condition is true.
//!   ```
//!  #[succeeds_if(
//!    ctx.user.balance >= amount
//!     )]
//! pub fn withdraw(ctx: Context<Withdraw>, amount: u64) {
//!    ...
//! }
//! ```
//!
//!   - `errors_if` - The instruction should fail if and only if the given condition is true.
//!   ```rust
//! #[errors_if(
//!     ctx.user.balance < amount
//!    )]
//!     pub fn withdraw(ctx: Context<Withdraw>, amount: u64) {
//!         ...
//!     }
//!    ```
//!
//! ### Account Invariants
//! The other type of invariant is an Account Invariant.
//! This invariant describes some property of an account that should always hold.
//! We use the `invariant` macro to specify these invariants.
//!
//! - `invariant` - The account invariant should hold if and only if the given condition is true.
//! For example, the balance of a token account should never be negative.
//!
//! ```rust
//!     #[account]
//!     #[invariant(
//!         self.balance >= 0
//!     )]
//!     struct User {
//!         pub balance: i64,
//!     }
//! ```

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

/// The instruction should succeed if and only if the given condition is true.
#[proc_macro_attribute]
pub fn succeeds_if(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// The instruction should fail if and only if the given condition is true.
#[proc_macro_attribute]
pub fn errors_if(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// The account invariant should hold if and only if the given condition is true.
#[proc_macro_attribute]
pub fn invariant(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Ignore the following block of code for verification
#[proc_macro_attribute]
pub fn verify_ignore(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// The account has a constraint defined
#[proc_macro_attribute]
pub fn has_constraint(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Arbitrary)]
pub fn derive_arbitrary(_item: TokenStream) -> TokenStream {
    (quote! {}).into()
}
