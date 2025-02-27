// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use chrono::{DateTime, Utc};
use tari_common_types::tari_address::TariAddress;
use tari_comms::peer_manager::NodeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    pub alias: String,
    pub address: TariAddress,
    pub node_id: NodeId,
    pub last_seen: Option<DateTime<Utc>>,
    pub latency: Option<u32>,
    pub favourite: bool,
}

impl Contact {
    pub fn new(
        alias: String,
        address: TariAddress,
        last_seen: Option<DateTime<Utc>>,
        latency: Option<u32>,
        favourite: bool,
    ) -> Self {
        Self {
            alias,
            node_id: NodeId::from_key(address.comms_public_key()),
            address,
            last_seen,
            latency,
            favourite,
        }
    }
}

impl From<&TariAddress> for Contact {
    fn from(address: &TariAddress) -> Self {
        Self {
            alias: address.to_emoji_string(),
            address: address.clone(),
            node_id: NodeId::from_key(address.public_spend_key()),
            last_seen: None,
            latency: None,
            favourite: false,
        }
    }
}
