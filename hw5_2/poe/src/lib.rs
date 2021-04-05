#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{  
	decl_module, decl_storage, decl_event, 
	ensure, decl_error, dispatch};
use frame_system::ensure_signed;
use sp_std::vec::Vec;


pub trait Config: frame_system::Config {
  type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {

	trait Store for Module<T: Config> as TemplateModule {
		Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}


decl_event! {
  pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
    ClaimCreated(AccountId, Vec<u8>),
    ClaimRevoked(AccountId, Vec<u8>),
	ClaimTransferred(AccountId, Vec<u8>),
  }
 }

decl_error! {
  pub enum Error for Module<T: Config> {
    ProofAlreadyExist,
    ClaimNotExist,
    NotClaimOwner,
	ClaimAlreadyOwned,
  }
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;


		#[weight = 10]
		pub fn create_claim(origin, claim: Vec<u8>) {

			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			let block_num = <frame_system::Module<T>>::block_number();

			Proofs::<T>::insert(&claim, (&sender, block_num));

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			//Ok(())
		}


		#[weight = 10]
		pub fn revoke_claim(origin, claim: Vec<u8>)-> dispatch::DispatchResult  {

			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(sender == owner, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

			Ok(())
		}


		#[weight = 10]
		pub fn transfer_claim(origin, claim: Vec<u8>)-> dispatch::DispatchResult  {

			let receiv = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(receiv != owner, Error::<T>::ClaimAlreadyOwned);

			let blockno = <frame_system::Module<T>>::block_number();

			Proofs::<T>::insert(&claim, (&receiv,blockno));

			Self::deposit_event(RawEvent::ClaimTransferred(receiv, claim));

			Ok(())
		}
	}
}