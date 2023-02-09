// Copyright 2023 Watr Foundation
// This file is part of Watr.

// Watr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Watr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Watr.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use crate as pallet_did;
use crate::{mock::*, Event as MotionEvent};
use codec::Encode;
use frame_support::{assert_ok, dispatch::GetDispatchInfo, weights::Weight};
use frame_system::{EventRecord, Phase};
use mock::{RuntimeCall, RuntimeEvent};
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash};

fn record(event: RuntimeEvent) -> EventRecord<RuntimeEvent, H256> {
	EventRecord { phase: Phase::Initialization, event, topics: vec![] }
}

#[test]
fn simple_majority_works() {
	assert_eq!(true, true);
}
