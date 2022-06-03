// Copyright 2021-2022 Green Aureus GmbH

// Permission is hereby granted, free of charge, to any person obtaining a copy 
// of this software and associated documentation files (the "Software"), to read 
// the Software only. Permission is hereby NOT GRANTED to use, copy, modify, 
// merge, publish, distribute, sublicense, and/or sell copies of the Software.

//! Benchmarking setup for pallet-supply-chain

use super::*;

#[allow(unused)]
use crate::Pallet as SupplyChain;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    impl_benchmark_test_suite!(SupplyChain, crate::mock::new_test_ext(), crate::mock::Test);
}
