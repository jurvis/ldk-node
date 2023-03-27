use crate::error::Error;
use crate::hex_utils;
use crate::io::fs_store::FilesystemStore;
use crate::logger::FilesystemLogger;
use crate::wallet::{Wallet, WalletKeysManager};
use crate::UniffiCustomTypeConverter;

use lightning::chain::chainmonitor;
use lightning::chain::keysinterface::InMemorySigner;
use lightning::chain::transaction::OutPoint as LdkOutpoint;
use lightning::ln::channelmanager::ChannelDetails as LdkChannelDetails;
use lightning::ln::peer_handler::IgnoringMessageHandler;
use lightning::ln::{PaymentHash, PaymentPreimage, PaymentSecret};
use lightning::routing::gossip;
use lightning::routing::gossip::P2PGossipSync;
use lightning::routing::router::DefaultRouter;
use lightning::routing::scoring::ProbabilisticScorer;
use lightning::routing::utxo::UtxoLookup;
use lightning::util::ser::{Readable, Writeable, Writer};
use lightning_invoice::{Invoice, SignedRawInvoice};
use lightning_net_tokio::SocketDescriptor;
use lightning_transaction_sync::EsploraSyncClient;

use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::PublicKey;
use bitcoin::Address;

use std::convert::TryInto;
use std::default::Default;
use std::fmt;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub(crate) type ChainMonitor = chainmonitor::ChainMonitor<
	InMemorySigner,
	Arc<EsploraSyncClient<Arc<FilesystemLogger>>>,
	Arc<Wallet<bdk::database::SqliteDatabase>>,
	Arc<Wallet<bdk::database::SqliteDatabase>>,
	Arc<FilesystemLogger>,
	Arc<FilesystemStore>,
>;

pub(crate) type PeerManager = lightning::ln::peer_handler::PeerManager<
	SocketDescriptor,
	Arc<ChannelManager>,
	Arc<GossipSync>,
	Arc<OnionMessenger>,
	Arc<FilesystemLogger>,
	IgnoringMessageHandler,
	Arc<WalletKeysManager<bdk::database::SqliteDatabase>>,
>;

pub(crate) type ChannelManager = lightning::ln::channelmanager::ChannelManager<
	Arc<ChainMonitor>,
	Arc<Wallet<bdk::database::SqliteDatabase>>,
	Arc<WalletKeysManager<bdk::database::SqliteDatabase>>,
	Arc<WalletKeysManager<bdk::database::SqliteDatabase>>,
	Arc<WalletKeysManager<bdk::database::SqliteDatabase>>,
	Arc<Wallet<bdk::database::SqliteDatabase>>,
	Arc<Router>,
	Arc<FilesystemLogger>,
>;

pub(crate) type KeysManager = WalletKeysManager<bdk::database::SqliteDatabase>;

pub(crate) type Router =
	DefaultRouter<Arc<NetworkGraph>, Arc<FilesystemLogger>, Arc<Mutex<Scorer>>>;
pub(crate) type Scorer = ProbabilisticScorer<Arc<NetworkGraph>, Arc<FilesystemLogger>>;

pub(crate) type GossipSync =
	P2PGossipSync<Arc<NetworkGraph>, Arc<dyn UtxoLookup + Send + Sync>, Arc<FilesystemLogger>>;

pub(crate) type NetworkGraph = gossip::NetworkGraph<Arc<FilesystemLogger>>;

pub(crate) type OnionMessenger = lightning::onion_message::OnionMessenger<
	Arc<WalletKeysManager<bdk::database::SqliteDatabase>>,
	Arc<WalletKeysManager<bdk::database::SqliteDatabase>>,
	Arc<FilesystemLogger>,
	IgnoringMessageHandler,
>;

impl UniffiCustomTypeConverter for SocketAddr {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Ok(addr) = SocketAddr::from_str(&val) {
			return Ok(addr);
		}

		Err(Error::PublicKeyInvalid.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.to_string()
	}
}

impl UniffiCustomTypeConverter for PublicKey {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Ok(key) = PublicKey::from_str(&val) {
			return Ok(key);
		}

		Err(Error::PublicKeyInvalid.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.to_string()
	}
}

impl UniffiCustomTypeConverter for Address {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Ok(addr) = Address::from_str(&val) {
			return Ok(addr);
		}

		Err(Error::AddressInvalid.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.to_string()
	}
}

impl UniffiCustomTypeConverter for Invoice {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Ok(signed) = val.parse::<SignedRawInvoice>() {
			if let Ok(invoice) = Invoice::from_signed(signed) {
				return Ok(invoice);
			}
		}

		Err(Error::InvalidInvoice.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.to_string()
	}
}

impl UniffiCustomTypeConverter for PaymentHash {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Ok(hash) = Sha256::from_str(&val) {
			Ok(PaymentHash(hash.into_inner()))
		} else {
			Err(Error::PaymentHashInvalid.into())
		}
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		Sha256::from_slice(&obj.0).unwrap().to_string()
	}
}

impl UniffiCustomTypeConverter for PaymentPreimage {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Some(bytes_vec) = hex_utils::to_vec(&val) {
			let bytes_res = bytes_vec.try_into();
			if let Ok(bytes) = bytes_res {
				return Ok(PaymentPreimage(bytes));
			}
		}
		Err(Error::PaymentPreimageInvalid.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		hex_utils::to_string(&obj.0)
	}
}

impl UniffiCustomTypeConverter for PaymentSecret {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Some(bytes_vec) = hex_utils::to_vec(&val) {
			let bytes_res = bytes_vec.try_into();
			if let Ok(bytes) = bytes_res {
				return Ok(PaymentSecret(bytes));
			}
		}
		Err(Error::PaymentSecretInvalid.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		hex_utils::to_string(&obj.0)
	}
}

/// The global identifier of a channel.
///
/// Note that this will start out to be a temporary ID until channel funding negotiation is
/// finalized, at which point it will change to be a permanent global ID tied to the on-chain
/// funding transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelId(pub [u8; 32]);

impl UniffiCustomTypeConverter for ChannelId {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Some(hex_vec) = hex_utils::to_vec(&val) {
			if hex_vec.len() == 32 {
				let mut channel_id = [0u8; 32];
				channel_id.copy_from_slice(&hex_vec[..]);
				return Ok(Self(channel_id));
			}
		}
		Err(Error::ChannelIdInvalid.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		hex_utils::to_string(&obj.0)
	}
}

impl Writeable for ChannelId {
	fn write<W: Writer>(&self, writer: &mut W) -> Result<(), lightning::io::Error> {
		Ok(self.0.write(writer)?)
	}
}

impl Readable for ChannelId {
	fn read<R: lightning::io::Read>(
		reader: &mut R,
	) -> Result<Self, lightning::ln::msgs::DecodeError> {
		Ok(Self(Readable::read(reader)?))
	}
}

/// A local, potentially user-provided, identifier of a channel.
///
/// By default, this will be randomly generated for the user to ensure local uniqueness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserChannelId(pub u128);

impl UniffiCustomTypeConverter for UserChannelId {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		Ok(UserChannelId(u128::from_str(&val).map_err(|_| Error::ChannelIdInvalid)?))
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.0.to_string()
	}
}

impl Writeable for UserChannelId {
	fn write<W: Writer>(&self, writer: &mut W) -> Result<(), lightning::io::Error> {
		Ok(self.0.write(writer)?)
	}
}

impl Readable for UserChannelId {
	fn read<R: lightning::io::Read>(
		reader: &mut R,
	) -> Result<Self, lightning::ln::msgs::DecodeError> {
		Ok(Self(Readable::read(reader)?))
	}
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Network(pub bitcoin::Network);

impl Default for Network {
	fn default() -> Self {
		Self(bitcoin::Network::Regtest)
	}
}

impl FromStr for Network {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"mainnet" => Ok(Self(bitcoin::Network::Bitcoin)),
			"bitcoin" => Ok(Self(bitcoin::Network::Bitcoin)),
			"testnet" => Ok(Self(bitcoin::Network::Testnet)),
			"regtest" => Ok(Self(bitcoin::Network::Regtest)),
			"signet" => Ok(Self(bitcoin::Network::Signet)),
			_ => Err(()),
		}
	}
}

impl fmt::Display for Network {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Writeable for Network {
	fn write<W: Writer>(&self, writer: &mut W) -> Result<(), lightning::io::Error> {
		let val = match self.0 {
			bitcoin::Network::Bitcoin => 0u8,
			bitcoin::Network::Testnet => 1u8,
			bitcoin::Network::Regtest => 2u8,
			bitcoin::Network::Signet => 3u8,
		};
		Ok(val.write(writer)?)
	}
}

impl Readable for Network {
	fn read<R: lightning::io::Read>(
		reader: &mut R,
	) -> Result<Self, lightning::ln::msgs::DecodeError> {
		let network = match Readable::read(reader)? {
			0u8 => bitcoin::Network::Bitcoin,
			1u8 => bitcoin::Network::Testnet,
			2u8 => bitcoin::Network::Regtest,
			3u8 => bitcoin::Network::Signet,
			_ => return Err(lightning::ln::msgs::DecodeError::InvalidValue),
		};

		Ok(Self(network))
	}
}

impl UniffiCustomTypeConverter for Network {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		Ok(Network::from_str(&val).map_err(|_| Error::NetworkInvalid)?)
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.0.to_string()
	}
}

/// Details about the user's channel as returned by [`Node::list_channels`].
///
/// [`Node::list_channels`]: [`crate::Node::list_channels`]
pub struct ChannelDetails {
	/// The channel's ID.
	pub channel_id: ChannelId,
	/// The `node_id` of our channel's counterparty.
	pub counterparty: PublicKey,
	/// Information about the channel's funding transaction output. `None `unless a funding
	/// transaction has been successfully negotiated with the channel's counterparty.
	pub funding_txo: Option<OutPoint>,
	/// Position of the funding transaction on-chain. `None` unless the funding transaction has been
	/// confirmed and fully opened.
	pub short_channel_id: Option<u64>,
	/// The value, in satoshis, of this channel as appears in the funding output.
	pub channel_value_satoshis: u64,
	/// Total balance of the channel. It is the amount that will be returned to the user if the
	/// channel is closed. The value is not exact, due to potential in-flight and fee-rate changes.
	/// Therefore, exactly this amount is likely irrecoverable on close.
	pub balance_msat: u64,
	/// Available outbound capacity for sending HTLCs to the remote peer. The amount does not
	/// include any pending HTLCs which are not yet resolved (and, thus, whose balance is not
	/// available for inclusion in new outbound HTLCs). This further does not include any
	/// pending outgoing HTLCs which are awaiting some other resolution to be sent.
	pub outbound_capacity_msat: u64,
	/// Available outbound capacity for sending HTLCs to the remote peer. The amount does not
	/// include any pending HTLCs which are not yet resolved (and, thus, whose balance is not
	/// available for inclusion in new inbound HTLCs). This further does not include any
	/// pending outgoing HTLCs which are awaiting some other resolution to be sent.
	pub inbound_capacity_msat: u64,
	/// The number of required confirmations on the funding transactions before the funding is
	/// considered "locked". The amount is selected by the channel fundee.
	///
	/// The value will be `None` for outbound channels until the counterparty accepts the channel.
	pub confirmations_required: Option<u32>,
	/// The current number of confirmations on the funding transaction.
	pub confirmations: Option<u32>,
	/// Returns `True` if the channel was initiated (and therefore funded) by us.
	pub is_outbound: bool,
	/// Returns `True` if the channel is confirmed, both parties have exchanged `channel_ready`
	/// messages, and the channel is not currently being shut down. Both parties exchange
	/// `channel_ready` messages upon independently verifying that the required confirmations count
	/// provided by `confirmations_required` has been reached.
	pub is_channel_ready: bool,
	/// Returns `True` if the channel is (a) confirmed and `channel_ready` has been exchanged,
	/// (b) the peer is connected, and (c) the channel is not currently negotiating shutdown.
	pub is_usable: bool,
	/// Returns `True` if this channel is (or will be) publicly-announced
	pub is_public: bool,
	/// The difference in the CLTV value between incoming HTLCs and an outbound HTLC forwarded over
	/// the channel.
	pub cltv_expiry_delta: Option<u16>,
}

impl From<LdkChannelDetails> for ChannelDetails {
	fn from(value: LdkChannelDetails) -> Self {
		ChannelDetails {
			channel_id: ChannelId(value.channel_id),
			counterparty: value.counterparty.node_id,
			funding_txo: value.funding_txo.and_then(|o| Some(o.into())),
			short_channel_id: value.short_channel_id,
			channel_value_satoshis: value.channel_value_satoshis,
			balance_msat: value.balance_msat,
			outbound_capacity_msat: value.outbound_capacity_msat,
			inbound_capacity_msat: value.inbound_capacity_msat,
			confirmations_required: value.confirmations_required,
			confirmations: value.confirmations,
			is_outbound: value.is_outbound,
			is_channel_ready: value.is_channel_ready,
			is_usable: value.is_usable,
			is_public: value.is_public,
			cltv_expiry_delta: value.config.and_then(|c| Some(c.cltv_expiry_delta)),
		}
	}
}

/// Data structure that references and transaction output.
pub struct OutPoint {
	/// The referenced transaction's txid.
	pub txid: String,
	/// The index of the referenced output in its transaction's vout.
	pub index: u16,
}

impl From<LdkOutpoint> for OutPoint {
	fn from(value: LdkOutpoint) -> Self {
		OutPoint { txid: value.txid.to_string(), index: value.index }
	}
}
