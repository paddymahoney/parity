// Copyright 2015, 2016 Ethcore (UK) Ltd.
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

//! Collects votes on hashes at each height and round.

use util::*;
use super::message::ConsensusMessage;
use super::{Height, Round, Step};
use ethkey::recover;

#[derive(Debug)]
pub struct VoteCollector {
	/// Storing all Proposals, Prevotes and Precommits.
	votes: RwLock<BTreeMap<ConsensusMessage, Address>>
}

impl VoteCollector {
	pub fn new() -> VoteCollector {
		VoteCollector { votes: RwLock::new(BTreeMap::new()) }
	}

	pub fn vote(&self, message: ConsensusMessage, voter: Address) {
		if let Some(mut guard) = self.votes.write() {
			*guard.insert(message, voter);
		}
	}

	pub fn seal_signatures(&self, height: Height, round: Round, block_hash: Option<H256>) -> Vec<H520> {
		self.votes
			.read()
			.keys()
			// Get only Propose and Precommits.
			.filter(|m| m.is_aligned(height, round, block_hash) && m.step != Step::Prevote)
			.map(|m| m.signature)
			.collect()
	}

	pub fn aligned_signatures(&self, message: &ConsensusMessage) -> Vec<H520> {
		self.seal_signatures(message.height, message.round, message.block_hash)
	}

	pub fn count_signatures(&self, height: Height, round: Round) -> usize {
		self.votes
			.read()
			.keys()
			// Get only Propose and Precommits.
			.filter(|m| m.is_round(height, round) && m.step != Step::Prevote)
			.map(|m| m.signature)
			.collect()	
	}
}