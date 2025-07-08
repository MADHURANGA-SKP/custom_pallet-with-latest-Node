// pallets/foo/src/weights.rs   ← will be RE-GENERATED later
#![allow(dead_code)]
use frame_support::weights::Weight;
use frame_support::{traits::Get, weights::{constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Declare only the calls you want benchmarked
pub trait WeightInfo {
    fn create_user()  -> Weight;
    fn get_my_user_details()  -> Weight;
    fn update_user() -> Weight;
    fn remove_user() -> Weight;
}

/// Weights for pallet_template using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn create_user()  -> Weight {
        todo!()
    }

    fn get_my_user_details()  -> Weight {
        todo!()
    }

    fn update_user() -> Weight {
        todo!()
    }

    fn remove_user() -> Weight {
        todo!()
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn create_user()  -> Weight {
        todo!()
    }

    fn get_my_user_details()  -> Weight {
        todo!()
    }

    fn update_user() -> Weight {
        todo!()
    }

    fn remove_user() -> Weight {
        todo!()
    }
}
