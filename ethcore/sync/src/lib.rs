// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

#![warn(missing_docs)]

//! Blockchain sync module
//! Implements ethereum protocol version 63 as specified here:
//! https://github.com/ethereum/wiki/wiki/Ethereum-Wire-Protocol
//!

extern crate common_types as types;
extern crate ethcore_network as network;
extern crate ethcore_network_devp2p as devp2p;
extern crate parity_bytes as bytes;
extern crate ethcore_io as io;
extern crate ethcore_transaction as transaction;
extern crate ethcore;
extern crate ethereum_types;
extern crate env_logger;
extern crate hashdb;
extern crate fastmap;
extern crate rand;
extern crate semver;
extern crate parking_lot;
extern crate smallvec;
extern crate rlp;
extern crate ipnetwork;
extern crate keccak_hash as hash;
extern crate keccak_hasher;
extern crate triehash_ethereum;
extern crate kvdb;

extern crate ethcore_light as light;

#[cfg(test)] extern crate ethkey;
#[cfg(test)] extern crate kvdb_memorydb;
#[cfg(test)] extern crate rustc_hex;
#[cfg(test)] extern crate ethcore_private_tx;

#[macro_use]
extern crate macros;
#[macro_use]
extern crate log;
#[macro_use]
extern crate heapsize;
#[macro_use]
extern crate trace_time;

pub mod common_types;
#[cfg(feature = "light")]
pub mod light_sync;
#[cfg(not(feature = "light"))]
pub mod full_sync;

pub use devp2p::{NetworkService, validate_node_url};
pub use network::{NonReservedPeerMode, Error, ErrorKind, ConnectionFilter, ConnectionDirection, PeerId};
pub use common_types::*;

#[cfg(feature = "light")]
pub use light_sync::*;
#[cfg(not(feature = "light"))]
pub use full_sync::chain::{SyncStatus, SyncState};
#[cfg(not(feature = "light"))]
pub use full_sync::private_tx::{PrivateTxHandler, NoopPrivateTxHandler, SimplePrivateTxHandler};
#[cfg(not(feature = "light"))]
pub use full_sync::*;

use std::ops::Range;
use network::{NetworkContext, ProtocolId};

/// Trait for managing network
pub trait ManageNetwork : Send + Sync {
	/// Set to allow unreserved peers to connect
	fn accept_unreserved_peers(&self);
	/// Set to deny unreserved peers to connect
	fn deny_unreserved_peers(&self);
	/// Remove reservation for the peer
	fn remove_reserved_peer(&self, peer: String) -> Result<(), String>;
	/// Add reserved peer
	fn add_reserved_peer(&self, peer: String) -> Result<(), String>;
	/// Start network
	fn start_network(&self);
	/// Stop network
	fn stop_network(&self);
	/// Returns the minimum and maximum peers.
	/// Note that `range.end` is *exclusive*.
	// TODO: Range should be changed to RangeInclusive once stable (https://github.com/rust-lang/rust/pull/50758)
	fn num_peers_range(&self) -> Range<u32>;
	/// Get network context for protocol.
	fn with_proto_context(&self, proto: ProtocolId, f: &mut FnMut(&NetworkContext));
}
