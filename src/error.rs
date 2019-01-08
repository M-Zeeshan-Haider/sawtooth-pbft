/*
 * Copyright 2018 Bitwise IO, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

//! PBFT-specific error messages

use std::error::Error;
use std::fmt;

use hex;
use protobuf::error::ProtobufError;

use crate::protos::pbft_message::PbftBlock;

/// Errors that might occur in a PbftNode
#[derive(Debug)]
pub enum PbftError {
    /// An error occured while serializing or deserializing a Protobuf message
    SerializationError(ProtobufError),

    /// The blocks don't match but should
    MismatchedBlocks(Vec<PbftBlock>),

    /// The message is in a different view than this node is
    ViewMismatch(u64, u64),

    /// Internal PBFT error (description)
    InternalError(String),

    /// Timed out waiting for a message
    Timeout,

    /// There is no working block; no operations can be performed
    NoWorkingBlock,

    /// The message should only come from the primary, but was sent by a secondary node
    NotFromPrimary,
}

impl Error for PbftError {
    fn description(&self) -> &str {
        use self::PbftError::*;
        match self {
            SerializationError(_) => "SerializationError",
            MismatchedBlocks(_) => "MismatchedBlocks",
            ViewMismatch(_, _) => "ViewMismatch",
            InternalError(_) => "InternalError",
            Timeout => "Timeout",
            NoWorkingBlock => "NoWorkingBlock",
            NotFromPrimary => "NotFromPrimary",
        }
    }
}

impl fmt::Display for PbftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.description())?;
        match self {
            PbftError::SerializationError(pb_err) => pb_err.fmt(f),
            PbftError::ViewMismatch(exp, got) => write!(f, "View mismatch: {} != {}", exp, got),
            PbftError::MismatchedBlocks(blocks) => write!(
                f,
                "Mismatched blocks: {:?}",
                blocks.iter().map(|block| hex::encode(block.get_block_id()))
            ),
            PbftError::Timeout => write!(f, "Timed out"),
            PbftError::InternalError(description) => write!(f, "{}", description),
            PbftError::NoWorkingBlock => write!(f, "There is no working block"),
            PbftError::NotFromPrimary => write!(
                f,
                "Message should be from primary, but was sent by secondary"
            ),
        }
    }
}
