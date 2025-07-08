#![cfg_attr(not(feature = "std"), no_std)]

pub type Balance = u128;

//#[cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

// #[cfg(test)]
// mod mock;

pub use pallet::*;
// // pub mod weights;
// use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use log::info;
    use scale_info::prelude::string::String;
    use frame_system::ensure_signed;
    use frame_support::sp_runtime::BoundedVec;
    use frame_support::{pallet_prelude::{
        OptionQuery, 
        StorageValue, 
        StorageMap, 
        ValueQuery,
        Weight,
        DispatchResult,
        Encode,
        Decode,
        TypeInfo,
        MaxEncodedLen,
        RuntimeDebug,
        ConstU32,
        IsType,
        ensure
    }, Blake2_128};
    use frame_system::pallet_prelude::OriginFor;
    use frame_system::Origin;

	#[pallet::config]
	pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        //type WeightInfo: WeightInfo;
    }

    // custom struct 
    #[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen, PartialEq, Eq, RuntimeDebug)]
    pub struct UserDetails {
        pub fname: BoundedVec<u8, ConstU32<64>>,
        pub lname: BoundedVec<u8, ConstU32<64>>,
        pub address: BoundedVec<u8, ConstU32<128>>,
        pub age: u32,
    }

    // storage
    #[pallet::storage]
    pub type TotalIssuance<T: Config> = StorageValue<_, Balance, ValueQuery>;

    #[pallet::storage]
    pub type Balances<T: Config> = StorageMap<_,  Blake2_128, T::AccountId, Balance, ValueQuery>;

    #[pallet::storage]
    pub type UserDetailsStorage<T: Config> = StorageMap<
    _, 
        Blake2_128, 
        T::AccountId, 
        UserDetails, 
        OptionQuery
    >;

    // error
    #[pallet::error]
    pub enum Error<T> {
        FirstNameTooLong,
        LastNameTooLong,
        AddressTooLong,
        NoUserDataFound
    } 

    // event
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UserDataCreated {
            account: T::AccountId,
        },
        UserDataRemoved {
            account: T::AccountId,
        },
        UserDataUpdated {
            account: T::AccountId,
        },
        UserDataFetched {
            account: T::AccountId,
            fname: String,
            lname: String,
            address: String,
            age: u32,
        },
    }

    // Self::deposit_event(Event::UserDataCreated {
    //     account: who.clone(),
    // });

	#[pallet::pallet]
	pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An unsafe mint that can be called by anyone. Not a great idea.
        
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::default())]
        pub fn create_user(
            origin: OriginFor<T>,
            fname: String,
            lname: String,
            address: String,
            age: u32,
        ) -> DispatchResult {
            
            let who = ensure_signed(origin)?;

            let fname_bounded: BoundedVec<_, ConstU32<64>> = fname.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::FirstNameTooLong)?;

            let lname_bounded: BoundedVec<_, ConstU32<64>> = lname.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::LastNameTooLong)?;

            let address_bounded: BoundedVec<_, ConstU32<128>> = address.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::AddressTooLong)?;

            let details = UserDetails {
                fname: fname_bounded,
                lname: lname_bounded,
                address: address_bounded,
                age,
            };

            UserDetailsStorage::<T>::insert(&who, details);

            Self::deposit_event(Event::UserDataCreated {
                account: who,
            });

            Ok(())
        }

        /// Transfer `amount` from `origin` to `dest`.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::default())]
        pub fn get_my_user_details(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            match UserDetailsStorage::<T>::get(&who) {

                Some(details) => {
                    Self::deposit_event(Event::UserDataFetched {
                        account: who,
                        fname: String::from_utf8(details.fname.to_vec()).unwrap_or_default(),
                        lname: String::from_utf8(details.lname.to_vec()).unwrap_or_default(),
                        address: String::from_utf8(details.address.to_vec()).unwrap_or_default(),
                        age: details.age,
                    });
                    Ok(())
                },
                None => Err(Error::<T>::NoUserDataFound.into()),
            }
        }

        #[pallet::call_index(3)]
        #[pallet::weight(Weight::default())]
        pub fn update_user(
            origin: OriginFor<T>,
            fname: String,
            lname: String,
            address: String,
            age: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Convert to bounded vectors with validation
            let fname_bounded: BoundedVec<_, ConstU32<64>> = fname.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::FirstNameTooLong)?;

            let lname_bounded: BoundedVec<_, ConstU32<64>> = lname.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::LastNameTooLong)?;

            let address_bounded: BoundedVec<_, ConstU32<128>> = address.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::AddressTooLong)?;

            // Check if user data exists
            ensure!(UserDetailsStorage::<T>::contains_key(&who), Error::<T>::NoUserDataFound);

            // Overwrite data
            let updated = UserDetails {
                fname: fname_bounded,
                lname: lname_bounded,
                address: address_bounded,
                age,
            };

            UserDetailsStorage::<T>::insert(&who, updated);

            // Emit updated event if needed (reuse or create new)
            Self::deposit_event(Event::UserDataUpdated { account: who });

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(Weight::default())]
        pub fn remove_user(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check if data exists before attempting to remove
            ensure!(UserDetailsStorage::<T>::contains_key(&who), Error::<T>::NoUserDataFound);

            // Remove the entry
            UserDetailsStorage::<T>::remove(&who);

            // Optional: emit a removal event
            Self::deposit_event(Event::UserDataRemoved { account: who.clone() }); // Or define a `UserDataRemoved` event

            Ok(())
        }
    }

}


#[cfg(feature = "runtime-api")]
pub mod runtime_api {
    use super::*;
    use sp_std::prelude::*;
    use sp_runtime::traits::Convert;
    use sp_api::decl_runtime_apis;
    use scale_info::prelude::string::String;

    use frame_support::sp_runtime::AccountId32;

    decl_runtime_apis! {
        pub trait UserDetailsApi {
            fn get_user_details(account: AccountId32) -> Option<(String, String, String, u32)>;
        }
    }
}