//! Benchmarking setup for pallet-supply-chain

use super::*;

#[allow(unused)]
use crate::Pallet as SupplyChain;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    impl_benchmark_test_suite!(SupplyChain, crate::mock::new_test_ext(), crate::mock::Test);
}
