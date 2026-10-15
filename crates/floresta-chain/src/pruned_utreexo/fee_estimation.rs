// SPDX-License-Identifier: MIT OR Apache-2.0

//! Rolling window of per-block fee rates used for fee estimation.
//!
//! [`FeeRateStore`] is the outbound port used to persist per-block fee rates and
//! [`FeeEstimator`] is the read-side port used by protocol adapters (Electrum,
//! RPC). The rolling-window domain and its application service are built on top
//! of these ports.

use bitcoin::FeeRate;

use crate::DatabaseError;

/// A per-block fee rate, keyed by the height of the block it was observed in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeeRateEntry {
    pub height: u32,
    pub fee_rate: FeeRate,
}

/// Read-side port used by protocol adapters (Electrum, RPC) to answer fee queries.
pub trait FeeEstimator: Send + Sync + 'static {
    /// Returns the estimated fee rate to be included within `target` blocks.
    fn estimate_fee(&self, target: usize) -> FeeRate;
}

/// Narrow outbound port for persisting per-block fee rates.
pub trait FeeRateStore: Send + Sync + 'static {
    type Error: DatabaseError;
    fn save(&self, entry: FeeRateEntry) -> Result<(), Self::Error>;
    fn get(&self, height: u32) -> Result<Option<FeeRateEntry>, Self::Error>;
}
