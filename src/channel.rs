use bitcoin::secp256k1::PublicKey;
use lightning::chain::transaction::OutPoint as LdkOutpoint;
use lightning::ln::channelmanager::ChannelDetails as LdkChannelDetails;
use crate::ChannelId;

/// Data structure that references and transaction output.
pub struct OutPoint {
    /// The referenced transaction's txid.
    pub txid: String,
    /// The index of the referenced output in its transaction's vout.
    pub index: u16,
}

impl From<LdkOutpoint> for OutPoint {
    fn from(value: LdkOutpoint) -> Self {
       OutPoint {
           txid: value.txid.to_string(),
           index: value.index,
       }
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
