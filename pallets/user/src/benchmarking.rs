// pallets/foo/src/benchmarking.rs
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_support::{BoundedVec, traits::Get};
use frame_support::pallet_prelude::ConstU32;
use scale_info::prelude::string::String;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_std::vec;

/* -------------------------------------------------------------------- *
 *  Benchmarks                                                          *
 * -------------------------------------------------------------------- */
#[benchmarks]
mod benchmarks {
    use super::*;

    /* ===== create_user ============================================= */
    #[benchmark]
    fn create_user() {
        // — setup — ----------------------------------------------------------
        let caller: T::AccountId = whitelisted_caller();

        // worst-case input (64-byte names, max u32 age, etc.)
        let fname   = String::from_utf8(vec![b'F'; 64]).unwrap();      // "FFFFFFFF…"
        let lname   = String::from_utf8(vec![b'L'; 64]).unwrap();
        let address = String::from_utf8(vec![b'A'; 128]).unwrap();
        let age: u32 = u32::MAX;

        // — extrinsic under test — ------------------------------------------
        #[extrinsic_call]      // <-- attribute sits *above* the placeholder _
        _(
            RawOrigin::Signed(caller.clone()),
            fname.clone(),
            lname.clone(),
            address.clone(),
            age
        );

        // — post-condition — -------------------------------------------------
        assert!(UserDetailsStorage::<T>::contains_key(&caller));
    }

    /* ===== get_my_user_details ===================================== */
    // #[benchmark]
    // fn get_my_user_details() {
        
    // }

    /* ===== update_user ============================================= */
    // #[benchmark]
    // fn update_user() {
        
    // }

    /* ===== remove_user ============================================= */
    // #[benchmark]
    // fn remove_user() {
        
    // }

    /* ===== wiring for `cargo test --features runtime-benchmarks` ==== */
    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::test_ext(),
        crate::mock::Test
    );
}
