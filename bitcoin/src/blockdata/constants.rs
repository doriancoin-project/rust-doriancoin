// Written in 2014 by Andrew Poelstra <apoelstra@wpsoftware.net>
// SPDX-License-Identifier: CC0-1.0

//! Blockdata constants.
//!
//! This module provides various constants relating to the blockchain and
//! consensus code. In particular, it defines the genesis block and its
//! single transaction.
//!

use core::convert::TryFrom;
use core::default::Default;

use bitcoin_internals::impl_array_newtype;
use hex_lit::hex;

use crate::hashes::{Hash, sha256d};
use crate::blockdata::script;
use crate::blockdata::opcodes::all::*;
use crate::blockdata::locktime::absolute;
use crate::blockdata::transaction::{OutPoint, Transaction, TxOut, TxIn, Sequence};
use crate::blockdata::block::{self, Block};
use crate::blockdata::witness::Witness;
use crate::network::constants::Network;
use crate::pow::CompactTarget;
use crate::internal_macros::impl_bytes_newtype;

/// How many satoshis are in "one bitcoin".
pub const COIN_VALUE: u64 = 100_000_000;
/// How many seconds between blocks we expect on average
pub const TARGET_BLOCK_SPACING: u32 = 150;
/// How many blocks between diffchanges
pub const DIFFCHANGE_INTERVAL: u32 = 2016;
/// How much time on average should occur between diffchanges
pub const DIFFCHANGE_TIMESPAN: u32 = 150 * 2016;
/// The maximum allowed weight for a block, see BIP 141 (network rule)
pub const MAX_BLOCK_WEIGHT: u32 = 4_000_000;
/// The minimum transaction weight for a valid serialized transaction.
pub const MIN_TRANSACTION_WEIGHT: u32 = 4 * 60;
/// The factor that non-witness serialization data is multiplied by during weight calculation.
pub const WITNESS_SCALE_FACTOR: usize = 4;
/// The maximum allowed number of signature check operations in a block.
pub const MAX_BLOCK_SIGOPS_COST: i64 = 80_000;
/// Mainnet (doriancoin) pubkey address prefix.
pub const PUBKEY_ADDRESS_PREFIX_MAIN: u8 = 30; // 0x1e
/// Mainnet (doriancoin) script address prefix.
pub const SCRIPT_ADDRESS_PREFIX_MAIN: u8 = 28; // 0x1c
/// Test (testnet, signet, regtest) pubkey address prefix.
pub const PUBKEY_ADDRESS_PREFIX_TEST: u8 = 111; // 0x6f
/// Test (testnet, signet, regtest) script address prefix.
pub const SCRIPT_ADDRESS_PREFIX_TEST: u8 = 58; // 0x3a
/// The maximum allowed script size.
pub const MAX_SCRIPT_ELEMENT_SIZE: usize = 520;
/// How may blocks between halvings.
pub const SUBSIDY_HALVING_INTERVAL: u32 = 840_000;
/// Maximum allowed value for an integer in Script.
pub const MAX_SCRIPTNUM_VALUE: u32 = 0x80000000; // 2^31
/// Number of blocks needed for an output from a coinbase transaction to be spendable.
pub const COINBASE_MATURITY: u32 = 100;

/// The maximum value allowed in an output (useful for sanity checking,
/// since keeping everything below this value should prevent overflows
/// if you are doing anything remotely sane with monetary values).
pub const MAX_MONEY: u64 = 84_000_000 * COIN_VALUE;

/// Constructs and returns the coinbase (and only) transaction of the Doriancoin genesis block.
fn bitcoin_genesis_tx() -> Transaction {
    // Base
    let mut ret = Transaction {
        version: 1,
        lock_time: absolute::LockTime::ZERO,
        input: vec![],
        output: vec![],
    };

    // Inputs
    let msg: &[u8] = b"LA Times 08/Mar/2014 For Dorian Nakamoto, bitcoin article brings denials, intrigue";
    let push_msg = <&script::PushBytes>::try_from(msg).expect("genesis coinbase message fits");
    let in_script = script::Builder::new().push_int(486604799)
                                          .push_int_non_minimal(4)
                                          .push_slice(push_msg)
                                          .into_script();
    ret.input.push(TxIn {
        previous_output: OutPoint::null(),
        script_sig: in_script,
        sequence: Sequence::MAX,
        witness: Witness::default(),
    });

    // Outputs
    let script_bytes = hex!("040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9");
    let out_script = script::Builder::new()
        .push_slice(script_bytes)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    ret.output.push(TxOut {
        value: 50 * COIN_VALUE,
        script_pubkey: out_script
    });

    // end
    ret
}

/// Constructs and returns the genesis block.
pub fn genesis_block(network: Network) -> Block {
    let txdata = vec![bitcoin_genesis_tx()];
    let hash: sha256d::Hash = txdata[0].txid().into();
    let merkle_root = hash.into();
    match network {
        Network::Bitcoin => {
            Block {
                header: block::Header {
                    version: block::Version::ONE,
                    prev_blockhash: Hash::all_zeros(),
                    merkle_root,
                    time: 1394325760,
                    bits: CompactTarget::from_consensus(0x1e0ffff0),
                    nonce: 385834689
                },
                txdata,
            }
        }
        Network::Testnet => {
            Block {
                header: block::Header {
                    version: block::Version::ONE,
                    prev_blockhash: Hash::all_zeros(),
                    merkle_root,
                    time: 1394325759,
                    bits: CompactTarget::from_consensus(0x1e0ffff0),
                    nonce: 149343
                },
                txdata,
            }
        }
        Network::Signet => {
            Block {
                header: block::Header {
                    version: block::Version::ONE,
                    prev_blockhash: Hash::all_zeros(),
                    merkle_root,
                    time: 1394325759,
                    bits: CompactTarget::from_consensus(0x1e0ffff0),
                    nonce: 149343
                },
                txdata,
            }
        }
        Network::Regtest => {
            Block {
                header: block::Header {
                    version: block::Version::ONE,
                    prev_blockhash: Hash::all_zeros(),
                    merkle_root,
                    time: 1394325759,
                    bits: CompactTarget::from_consensus(0x1e0ffff0),
                    nonce: 149343
                },
                txdata,
            }
        }
    }
}

/// The uniquely identifying hash of the target blockchain.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChainHash([u8; 32]);
impl_array_newtype!(ChainHash, u8, 32);
impl_bytes_newtype!(ChainHash, 32);

impl ChainHash {
    /// `ChainHash` for mainnet Doriancoin.
    pub const BITCOIN: Self = Self([185, 209, 231, 209, 199, 35, 6, 203, 181, 113, 2, 65, 27, 9, 198, 235, 226, 254, 197, 105, 125, 8, 86, 116, 11, 210, 123, 39, 94, 162, 29, 210]);
    /// `ChainHash` for testnet Doriancoin.
    pub const TESTNET: Self = Self([81, 191, 47, 89, 253, 225, 142, 91, 150, 92, 50, 82, 24, 40, 69, 99, 34, 114, 14, 95, 188, 205, 117, 123, 221, 159, 181, 78, 70, 105, 119, 112]);
    /// `ChainHash` for signet Doriancoin.
    pub const SIGNET: Self = Self([81, 191, 47, 89, 253, 225, 142, 91, 150, 92, 50, 82, 24, 40, 69, 99, 34, 114, 14, 95, 188, 205, 117, 123, 221, 159, 181, 78, 70, 105, 119, 112]);
    /// `ChainHash` for regtest Doriancoin.
    pub const REGTEST: Self = Self([81, 191, 47, 89, 253, 225, 142, 91, 150, 92, 50, 82, 24, 40, 69, 99, 34, 114, 14, 95, 188, 205, 117, 123, 221, 159, 181, 78, 70, 105, 119, 112]);

    /// Returns the hash of the `network` genesis block for use as a chain hash.
    ///
    /// See [BOLT 0](https://github.com/lightning/bolts/blob/ffeece3dab1c52efdb9b53ae476539320fa44938/00-introduction.md#chain_hash)
    /// for specification.
    pub const fn using_genesis_block(network: Network) -> Self {
        let hashes = [Self::BITCOIN, Self::TESTNET, Self::SIGNET, Self::REGTEST];
        hashes[network as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::network::constants::Network;
    use crate::consensus::encode::serialize;
    use crate::blockdata::locktime::absolute;
    use crate::internal_macros::hex;

    #[test]
    fn bitcoin_genesis_first_transaction() {
        let gen = bitcoin_genesis_tx();

        assert_eq!(gen.version, 1);
        assert_eq!(gen.input.len(), 1);
        assert_eq!(gen.input[0].previous_output.txid, Hash::all_zeros());
        assert_eq!(gen.input[0].previous_output.vout, 0xFFFFFFFF);
        assert_eq!(serialize(&gen.input[0].script_sig),
                   hex!("5b04ffff001d01044c524c412054696d65732030382f4d61722f3230313420466f7220446f7269616e204e616b616d6f746f2c20626974636f696e2061727469636c65206272696e67732064656e69616c732c20696e747269677565"));

        assert_eq!(gen.input[0].sequence, Sequence::MAX);
        assert_eq!(gen.output.len(), 1);
        assert_eq!(serialize(&gen.output[0].script_pubkey),
                   hex!("4341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac"));
        assert_eq!(gen.output[0].value, 50 * COIN_VALUE);
        assert_eq!(gen.lock_time, absolute::LockTime::ZERO);

        assert_eq!(gen.wtxid().to_string(), "a27b7d0a286e46fae3cb7e5b1eae6001fc1b15afee2f6a147291e7eb19746d5d");
    }

    #[test]
    fn bitcoin_genesis_full_block() {
        let gen = genesis_block(Network::Bitcoin);

        assert_eq!(gen.header.version, block::Version::ONE);
        assert_eq!(gen.header.prev_blockhash, Hash::all_zeros());
        assert_eq!(gen.header.merkle_root.to_string(), "a27b7d0a286e46fae3cb7e5b1eae6001fc1b15afee2f6a147291e7eb19746d5d");

        assert_eq!(gen.header.time, 1394325760);
        assert_eq!(gen.header.bits, CompactTarget::from_consensus(0x1e0ffff0));
        assert_eq!(gen.header.nonce, 385834689);
        assert_eq!(gen.header.block_hash().to_string(), "d21da25e277bd20b7456087d69c5fee2ebc6091b410271b5cb0623c7d1e7d1b9");
    }

    #[test]
    fn testnet_genesis_full_block() {
        let gen = genesis_block(Network::Testnet);
        assert_eq!(gen.header.version, block::Version::ONE);
        assert_eq!(gen.header.prev_blockhash, Hash::all_zeros());
        assert_eq!(gen.header.merkle_root.to_string(), "a27b7d0a286e46fae3cb7e5b1eae6001fc1b15afee2f6a147291e7eb19746d5d");
        assert_eq!(gen.header.time, 1394325759);
        assert_eq!(gen.header.bits, CompactTarget::from_consensus(0x1e0ffff0));
        assert_eq!(gen.header.nonce, 149343);
        assert_eq!(gen.header.block_hash().to_string(), "707769464eb59fdd7b75cdbc5f0e72226345281852325c965b8ee1fd592fbf51");
    }

    #[test]
    fn signet_genesis_full_block() {
        let gen = genesis_block(Network::Signet);
        assert_eq!(gen.header.version, block::Version::ONE);
        assert_eq!(gen.header.prev_blockhash, Hash::all_zeros());
        assert_eq!(gen.header.merkle_root.to_string(), "a27b7d0a286e46fae3cb7e5b1eae6001fc1b15afee2f6a147291e7eb19746d5d");
        assert_eq!(gen.header.time, 1394325759);
        assert_eq!(gen.header.bits, CompactTarget::from_consensus(0x1e0ffff0));
        assert_eq!(gen.header.nonce, 149343);
        assert_eq!(gen.header.block_hash().to_string(), "707769464eb59fdd7b75cdbc5f0e72226345281852325c965b8ee1fd592fbf51");
    }

    // The *_chain_hash tests are sanity/regression tests, they verify that the const byte array
    // representing the genesis block is the same as that created by hashing the genesis block.
    fn chain_hash_and_genesis_block(network: Network) {
        use crate::hashes::sha256;

        // The genesis block hash is a double-sha256 and it is displayed backwards.
        let genesis_hash = genesis_block(network).block_hash();
        // We abuse the sha256 hash here so we get a LowerHex impl that does not print the hex backwards.
        let hash = sha256::Hash::from_slice(genesis_hash.as_byte_array()).unwrap();
        let want = format!("{:02x}", hash);

        let chain_hash = ChainHash::using_genesis_block(network);
        let got = format!("{:02x}", chain_hash);

        // Compare strings because the spec specifically states how the chain hash must encode to hex.
        assert_eq!(got, want);

        match network {
            Network::Bitcoin => {},
            Network::Testnet => {},
            Network::Signet => {},
            Network::Regtest => {},
            // Update ChainHash::using_genesis_block and chain_hash_genesis_block with new variants.
        }
    }

    macro_rules! chain_hash_genesis_block {
        ($($test_name:ident, $network:expr);* $(;)*) => {
            $(
                #[test]
                fn $test_name() {
                    chain_hash_and_genesis_block($network);
                }
            )*
        }
    }

    chain_hash_genesis_block! {
        mainnet_chain_hash_genesis_block, Network::Bitcoin;
        testnet_chain_hash_genesis_block, Network::Testnet;
        signet_chain_hash_genesis_block, Network::Signet;
        regtest_chain_hash_genesis_block, Network::Regtest;
    }

    // Test vector taken from: https://github.com/lightning/bolts/blob/master/00-introduction.md
    #[test]
    fn mainnet_chain_hash_test_vector() {
        let got = ChainHash::using_genesis_block(Network::Bitcoin).to_string();
        let want = "b9d1e7d1c72306cbb57102411b09c6ebe2fec5697d0856740bd27b275ea21dd2";
        assert_eq!(got, want);
    }
}
