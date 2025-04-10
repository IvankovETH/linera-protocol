// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! An ABI for applications that implement an EVM runtime.
use linera_base::abi::{ContractAbi, ServiceAbi};

/// An ABI for applications that implement an EVM runtime.
pub struct EvmAbi;

impl ContractAbi for EvmAbi {
    type Operation = Vec<u8>;
    type Response = Vec<u8>;
}

impl ServiceAbi for EvmAbi {
    type Query = Vec<u8>;
    type QueryResponse = Vec<u8>;
}
