#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Configuration trait for the pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Defines the event type for the pallet.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	/// Storage item to keep a boolean value.
	#[pallet::storage]
	pub type StoredValue<T> = StorageValue<_, bool>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new value was stored.
		ValueStored {
			/// The stored value.
			value: bool,
			/// The account who stored it.
			who: T::AccountId,
		},
		/// The stored value was flipped.
		ValueFlipped {
			/// The new flipped value.
			new_value: bool,
			/// The account who flipped it.
			who: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// No value is currently stored.
		NoneValue,
		/// A value has already been set.
		AlreadySet,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Store a boolean value if nothing has been stored yet.
		///
		/// Emits `ValueStored` event on success.
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn set_value(origin: OriginFor<T>, value: bool) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(StoredValue::<T>::get().is_none(), Error::<T>::AlreadySet);

			StoredValue::<T>::put(value);

			Self::deposit_event(Event::ValueStored { value, who });

			Ok(())
		}

		/// Flip the current boolean value.
		///
		/// Emits `ValueFlipped` event on success.
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn flip_value(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let old_value = StoredValue::<T>::get().ok_or(Error::<T>::NoneValue)?;

			let new_value = !old_value;

			StoredValue::<T>::put(new_value);

			Self::deposit_event(Event::ValueFlipped {
				new_value,
				who,
			});

			Ok(())
		}
	}
}
