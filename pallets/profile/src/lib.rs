// pallets/profile/src/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        sp_runtime::BoundedVec,
    };
    use user::UserApi;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::string::String;

    /* -------------------------------------------------
     *  Associated-type & pallet declaration
     * ------------------------------------------------- */
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type UserPallet: UserApi<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /* -------------------------------------------------
     *  Helper types & enums
     * ------------------------------------------------- */
    // Adjust lengths to taste once UI limits are known.
    pub type Str64  = BoundedVec<u8, ConstU32<64>>;
    pub type Str128 = BoundedVec<u8, ConstU32<128>>;

    #[derive(
        Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Clone, Copy, Eq, PartialEq, DecodeWithMemTracking
    )]
    pub enum MaritalStatus { Single, Married, Divorced, Widowed }

    #[derive(
        Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Clone, Copy, Eq, PartialEq, DecodeWithMemTracking
    )]
    pub enum Gender { Male, Female, Other }

    #[derive(
        Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Clone, Copy, Eq, PartialEq, DecodeWithMemTracking
    )]
    pub enum BloodType { APos, ANeg, BPos, BNeg, OPos, ONeg, ABPos, ABNeg }

    // Keep Province / District to what you actually need.
    #[derive(
        Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Clone, Copy, Eq, PartialEq, DecodeWithMemTracking
    )]
    pub enum Province { Western, Central, Southern, Northern, Eastern,
                         NorthWestern, NorthCentral, Uva, Sabaragamuwa }

    #[derive(
        Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Clone, Copy, Eq, PartialEq, DecodeWithMemTracking
    )]
    pub enum District { Colombo, Gampaha /* … */ }


    pub type PostalCode = BoundedVec<u8, ConstU32<8>>;

    /* -------------------------------------------------
     *  Core data structures
     * ------------------------------------------------- */
    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Clone, Eq, PartialEq)]
    pub struct UserProfileData {
        pub f_name      : Str64,
        pub m_name      : Str64,
        pub l_name      : Str64,
        pub marital_status: MaritalStatus,
        pub pf_pic_path : Str128,
        pub gender      : Gender,
        pub blood_group : BloodType,
        pub nationality : Str64,
        pub religion    : Str64,
        pub lit_lang    : Str64,
        pub province    : Province,
        pub district    : District,
        pub city        : Str64,
        pub division    : Str64,
        pub postal_code : u32,          // 0 == “unset”
        pub birth_date  : Str64,        // e.g. `YYYY-MM-DD`
    }

    #[derive(Default, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Clone, Eq, PartialEq, DecodeWithMemTracking)]
    pub struct UserProfileDataUpdate {
        pub f_name        : Option<Str64>,
        pub m_name        : Option<Str64>,
        pub l_name        : Option<Str64>,
        pub marital_status: Option<MaritalStatus>,
        pub pf_pic_path   : Option<Str128>,
        pub gender        : Option<Gender>,
        pub blood_group   : Option<BloodType>,
        pub nationality   : Option<Str64>,
        pub religion      : Option<Str64>,
        pub lit_lang      : Option<Str64>,
        pub province      : Option<Province>,
        pub district      : Option<District>,
        pub city          : Option<Str64>,
        pub division      : Option<Str64>,
        pub postal_code   : Option<u32>,
        pub birth_date    : Option<Str64>,
    }

    /* -------------------------------------------------
     *  Storage
     * ------------------------------------------------- */
    #[pallet::storage]
    #[pallet::getter(fn profile_of)]
    pub type Profiles<T: Config> =
        StorageMap<_, Blake2_128, T::AccountId, UserProfileData, OptionQuery>;

    /* -------------------------------------------------
     *  Errors / Events
     * ------------------------------------------------- */
    #[pallet::error]
    pub enum Error<T> {
        DuplicateProfile,
        ProfileNotFound,
        InvalidBirthDate,
        StringTooLong,
        UserNotRegistered
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProfileCreated { account: T::AccountId },
        ProfileUpdated { account: T::AccountId },
        ProfileRemoved { account: T::AccountId },
        ProfileDataFetched {
            account : T::AccountId,
            f_name      : String,
            m_name      : String,
            l_name      : String,
            marital_status: MaritalStatus,
            pf_pic_path : String,
            gender      : Gender,
            blood_group : BloodType,
            nationality : String,
            religion    : String,
            lit_lang    : String,
            province    : Province,
            district    : District,
            city        : String,
            division    : String,
            postal_code : u32,          // 0 == “unset”
            birth_date  : String,
        },
    }

    /* -------------------------------------------------
     *  Internal helpers
     * ------------------------------------------------- */
    impl<T: Config> Pallet<T> {
    /// Very light `YYYY-MM-DD` check
    fn is_valid_date(date: &str) -> bool {
            // Cheap ASCII-only test; works as long as you guarantee the input
            // is plain ASCII (that’s true for the bounded-string you built).
            date.len() == 10
                && &date[4..5] == "-"        // YYYY-**-DD
                && &date[7..8] == "-"        // YYYY-MM-**
                // all other chars must be digits
                && date
                    .chars()
                    .enumerate()
                    .all(|(i, c)| matches!(i, 4 | 7) || c.is_ascii_digit())
        }
    }

    /* -------------------------------------------------
     *  Dispatchables
     * ------------------------------------------------- */
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a fresh profile (fails if one already exists).
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::default())]
        #[allow(clippy::too_many_arguments)]
        pub fn create_profile(
            origin              : OriginFor<T>,
            f_name              : String,
            m_name              : String,
            l_name              : String,
            marital_status      : MaritalStatus,
            pf_pic_path         : String,
            gender              : Gender,
            blood_group         : BloodType,
            nationality         : String,
            religion            : String,
            lit_lang            : String,
            province            : Province,
            district            : District,
            city                : String,
            division            : String,
            postal_code         : u32,
            birth_date          : String,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                <T as Config>::UserPallet::user_exists(&who),
                Error::<T>::UserNotRegistered
            );

            ensure!(!Profiles::<T>::contains_key(&who), Error::<T>::DuplicateProfile);

            let fname: BoundedVec<_, ConstU32<64>> = f_name.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            let mname: BoundedVec<_, ConstU32<64>> = m_name.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            let lname: BoundedVec<_, ConstU32<64>> = l_name.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            let pfpicpath: BoundedVec<_, ConstU32<128>> = pf_pic_path.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            let n_ationality: BoundedVec<_, ConstU32<64>> = nationality.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            let r_eligion: BoundedVec<_, ConstU32<64>> = religion.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            let litlang: BoundedVec<_, ConstU32<64>> = lit_lang.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            let c_ity: BoundedVec<_, ConstU32<64>> = city.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            let d_ivision: BoundedVec<_, ConstU32<64>> = division.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            // let postal_code_bv: BoundedVec<u8, ConstU32<8>> =
            //     postal_code
            //         .map(|n| n.to_string())        // u32 → String
            //         .unwrap_or_default()            // Option::unwrap_or_default
            //         .into_bytes()
            //         .try_into()
            //         .map_err(|_| Error::<T>::StringTooLong)?;

            // let postal_code_bv: PostalCode = postal_code          // Option<u32>
            //     .map(|n| n.to_string())        // "00400", etc.
            //     .unwrap_or_default()           // default ""
            //     .into_bytes()
            //     .try_into()
            //     .map_err(|_| Error::<T>::StringTooLong)?;
            
            ensure!(Self::is_valid_date(&birth_date), Error::<T>::InvalidBirthDate);


            let birthdate: BoundedVec<_, ConstU32<64>> = birth_date.clone().into_bytes()
                .try_into().map_err(|_| Error::<T>::StringTooLong)?;

            let data = UserProfileData {
                f_name        : fname,
                m_name        : mname,
                l_name        : lname,
                marital_status,
                pf_pic_path   : pfpicpath,
                gender,
                blood_group,
                nationality   : n_ationality,
                religion      : r_eligion,
                lit_lang      : litlang,
                province,
                district,
                city          : c_ity,
                division      : d_ivision,
                postal_code,
                birth_date: birthdate,
            };

            Profiles::<T>::insert(&who, data);
            Self::deposit_event(Event::ProfileCreated { account: who });
            Ok(())
        }

        /// Patch any subset of fields that are `Some(..)`.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::default())]
        pub fn update_profile(
            origin  : OriginFor<T>,
            updates : UserProfileDataUpdate,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Profiles::<T>::try_mutate(&who, |maybe_profile| -> DispatchResult {
                let profile = maybe_profile.as_mut().ok_or(Error::<T>::ProfileNotFound)?;

                macro_rules! replace_opt {
                    ($field:ident, $val:expr) => {
                        if let Some(new_val) = $val { profile.$field = new_val; }
                    };
                }
                replace_opt!(f_name,         updates.f_name);
                replace_opt!(m_name,         updates.m_name);
                replace_opt!(l_name,         updates.l_name);
                replace_opt!(marital_status, updates.marital_status);
                replace_opt!(pf_pic_path,    updates.pf_pic_path);
                replace_opt!(gender,         updates.gender);
                replace_opt!(blood_group,    updates.blood_group);
                replace_opt!(nationality,    updates.nationality);
                replace_opt!(religion,       updates.religion);
                replace_opt!(lit_lang,       updates.lit_lang);
                replace_opt!(province,       updates.province);
                replace_opt!(district,       updates.district);
                replace_opt!(city,           updates.city);
                replace_opt!(division,       updates.division);
                replace_opt!(postal_code,    updates.postal_code);

                if let Some(bd) = updates.birth_date {

                    let bd_str = sp_std::str::from_utf8(&bd).map_err(|_| Error::<T>::InvalidBirthDate)?;
                    
                    ensure!(Self::is_valid_date(&bd_str), Error::<T>::InvalidBirthDate);
                    profile.birth_date = bd;
                }

                Ok(())
            })?;

            Self::deposit_event(Event::ProfileUpdated { account: who });
            Ok(())
        }

        /// Delete the caller’s profile.
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::default())]
        pub fn remove_profile(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Profiles::<T>::contains_key(&who), Error::<T>::ProfileNotFound);

            Profiles::<T>::remove(&who);
            Self::deposit_event(Event::ProfileRemoved { account: who });
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(Weight::default())]
        pub fn get_my_profile_details(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            match Profiles::<T>::get(&who) {
                Some(p) => {
                    // Emit the event with human-readable strings
                    Self::deposit_event(Event::ProfileDataFetched {
                        account: who,
                        f_name: String::from_utf8(p.f_name.to_vec()).unwrap_or_default(),
                        m_name: String::from_utf8(p.m_name.to_vec()).unwrap_or_default(),
                        l_name: String::from_utf8(p.l_name.to_vec()).unwrap_or_default(),
                        gender: p.gender,
                        marital_status: p.marital_status, 
                        pf_pic_path:  String::from_utf8(p.pf_pic_path.to_vec()).unwrap_or_default(), 
                        blood_group: p.blood_group, 
                        nationality:  String::from_utf8(p.nationality.to_vec()).unwrap_or_default(), 
                        religion:  String::from_utf8(p.religion.to_vec()).unwrap_or_default(), 
                        lit_lang:  String::from_utf8(p.lit_lang.to_vec()).unwrap_or_default(), 
                        province: p.province, 
                        district: p.district, 
                        city:  String::from_utf8(p.city.to_vec()).unwrap_or_default(), 
                        division:  String::from_utf8(p.division.to_vec()).unwrap_or_default(), 
                        postal_code: p.postal_code, 
                        birth_date:  String::from_utf8(p.birth_date.to_vec()).unwrap_or_default() 
                    });
                    Ok(())
                },
                None => Err(Error::<T>::ProfileNotFound.into()),
            }
        }
    }

    /* -------------------------------------------------
     *  Runtime-API – lightweight off-chain query
     * ------------------------------------------------- */
    // #[cfg(feature = "runtime-api")]
    // pub mod runtime_api {
    //     use super::*;
    //     use sp_api::decl_runtime_apis;

    //     decl_runtime_apis! {
    //         pub trait ProfileApi<AccountId> {
    //             fn get_profile(account: AccountId) -> Option<UserProfileData>;
    //         }
    //     }
    // }
}
