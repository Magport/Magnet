#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod rococo_runtime {
			use super::runtime_types;
			pub mod validator_manager {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						ValidatorsRegistered(::sp_std::vec::Vec<::sp_core::crypto::AccountId32>),
						#[codec(index = 1)]
						ValidatorsDeregistered(::sp_std::vec::Vec<::sp_core::crypto::AccountId32>),
					}
				}
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub enum ProxyType {
				#[codec(index = 0)]
				Any,
				#[codec(index = 1)]
				NonTransfer,
				#[codec(index = 2)]
				Governance,
				#[codec(index = 3)]
				IdentityJudgement,
				#[codec(index = 4)]
				CancelProxy,
				#[codec(index = 5)]
				Auction,
				#[codec(index = 6)]
				Society,
				#[codec(index = 7)]
				OnDemandOrdering,
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 3)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec(index = 4)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 33)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 7)]
				Offences(runtime_types::pallet_offences::pallet::Event),
				#[codec(index = 8)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 10)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Event),
				#[codec(index = 11)]
				ImOnline(runtime_types::pallet_im_online::pallet::Event),
				#[codec(index = 13)]
				Democracy(runtime_types::pallet_democracy::pallet::Event),
				#[codec(index = 14)]
				Council(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 15)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Event2),
				#[codec(index = 16)]
				PhragmenElection(runtime_types::pallet_elections_phragmen::pallet::Event),
				#[codec(index = 17)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 18)]
				Treasury(runtime_types::pallet_treasury::pallet::Event),
				#[codec(index = 19)]
				Claims(runtime_types::polkadot_runtime_common::claims::pallet::Event),
				#[codec(index = 24)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec(index = 25)]
				Identity(runtime_types::pallet_identity::pallet::Event),
				#[codec(index = 26)]
				Society(runtime_types::pallet_society::pallet::Event),
				#[codec(index = 27)]
				Recovery(runtime_types::pallet_recovery::pallet::Event),
				#[codec(index = 28)]
				Vesting(runtime_types::pallet_vesting::pallet::Event),
				#[codec(index = 29)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Event),
				#[codec(index = 30)]
				Proxy(runtime_types::pallet_proxy::pallet::Event),
				#[codec(index = 31)]
				Multisig(runtime_types::pallet_multisig::pallet::Event),
				#[codec(index = 32)]
				Preimage(runtime_types::pallet_preimage::pallet::Event),
				#[codec(index = 35)]
				Bounties(runtime_types::pallet_bounties::pallet::Event),
				#[codec(index = 40)]
				ChildBounties(runtime_types::pallet_child_bounties::pallet::Event),
				#[codec(index = 36)]
				Tips(runtime_types::pallet_tips::pallet::Event),
				#[codec(index = 38)]
				Nis(runtime_types::pallet_nis::pallet::Event),
				#[codec(index = 45)]
				NisCounterpartBalances(runtime_types::pallet_balances::pallet::Event2),
				#[codec(index = 53)]
				ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Event),
				#[codec(index = 56)]
				Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Event),
				#[codec(index = 60)]
				Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Event),
				#[codec(index = 62)]
				ParasDisputes(runtime_types::polkadot_runtime_parachains::disputes::pallet::Event),
				#[codec(index = 64)]
				MessageQueue(runtime_types::pallet_message_queue::pallet::Event),
				#[codec(index = 66)]
				OnDemandAssignmentProvider(
					runtime_types::polkadot_runtime_parachains::assigner_on_demand::pallet::Event,
				),
				#[codec(index = 70)]
				Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Event),
				#[codec(index = 71)]
				Slots(runtime_types::polkadot_runtime_common::slots::pallet::Event),
				#[codec(index = 72)]
				Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Event),
				#[codec(index = 73)]
				Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Event),
				#[codec(index = 99)]
				XcmPallet(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 251)]
				AssignedSlots(
					runtime_types::polkadot_runtime_common::assigned_slots::pallet::Event,
				),
				#[codec(index = 252)]
				ValidatorManager(runtime_types::rococo_runtime::validator_manager::pallet::Event),
				#[codec(index = 254)]
				StateTrieMigration(runtime_types::pallet_state_trie_migration::pallet::Event),
				#[codec(index = 255)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
			}
		}
		pub mod pallet_indices {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					IndexAssigned {
						who: ::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					IndexFreed { index: ::core::primitive::u32 },
					#[codec(index = 2)]
					IndexFrozen {
						index: ::core::primitive::u32,
						who: ::sp_core::crypto::AccountId32,
					},
				}
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					TransactionFeePaid {
						who: ::sp_core::crypto::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
		}
		pub mod pallet_offences {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Offence {
						kind: [::core::primitive::u8; 16usize],
						timeslot: ::sp_std::vec::Vec<::core::primitive::u8>,
					},
				}
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					NewSession { session_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod sp_consensus_grandpa {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Public(pub runtime_types::sp_core::ed25519::Public);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
			}
		}
		pub mod pallet_grandpa {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					NewAuthorities {
						authority_set: ::sp_std::vec::Vec<(
							runtime_types::sp_consensus_grandpa::app::Public,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 1)]
					Paused,
					#[codec(index = 2)]
					Resumed,
				}
			}
		}
		pub mod pallet_im_online {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					HeartbeatReceived {
						authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
					},
					#[codec(index = 1)]
					AllGood,
					#[codec(index = 2)]
					SomeOffline {
						offline: ::sp_std::vec::Vec<(::sp_core::crypto::AccountId32, ())>,
					},
				}
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
			}
		}
		pub mod pallet_democracy {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Proposed {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					Tabled {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					ExternalTabled,
					#[codec(index = 3)]
					Started {
						ref_index: ::core::primitive::u32,
						threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					},
					#[codec(index = 4)]
					Passed { ref_index: ::core::primitive::u32 },
					#[codec(index = 5)]
					NotPassed { ref_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					Cancelled { ref_index: ::core::primitive::u32 },
					#[codec(index = 7)]
					Delegated {
						who: ::sp_core::crypto::AccountId32,
						target: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 8)]
					Undelegated { account: ::sp_core::crypto::AccountId32 },
					#[codec(index = 9)]
					Vetoed {
						who: ::sp_core::crypto::AccountId32,
						proposal_hash: runtime_types::primitive_types::H256,
						until: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					Blacklisted { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec(index = 11)]
					Voted {
						voter: ::sp_core::crypto::AccountId32,
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 12)]
					Seconded {
						seconder: ::sp_core::crypto::AccountId32,
						prop_index: ::core::primitive::u32,
					},
					#[codec(index = 13)]
					ProposalCanceled { prop_index: ::core::primitive::u32 },
					#[codec(index = 14)]
					MetadataSet {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: runtime_types::primitive_types::H256,
					},
					#[codec(index = 15)]
					MetadataCleared {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: runtime_types::primitive_types::H256,
					},
					#[codec(index = 16)]
					MetadataTransferred {
						prev_owner: runtime_types::pallet_democracy::types::MetadataOwner,
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: runtime_types::primitive_types::H256,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum MetadataOwner {
					#[codec(index = 0)]
					External,
					#[codec(index = 1)]
					Proposal(::core::primitive::u32),
					#[codec(index = 2)]
					Referendum(::core::primitive::u32),
				}
			}
			pub mod vote {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum AccountVote<_0> {
					#[codec(index = 0)]
					Standard { vote: runtime_types::pallet_democracy::vote::Vote, balance: _0 },
					#[codec(index = 1)]
					Split { aye: _0, nay: _0 },
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					// :: subxt :: ext :: codec :: CompactAs,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Vote(pub ::core::primitive::u8);
			}
			pub mod vote_threshold {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum VoteThreshold {
					#[codec(index = 0)]
					SuperMajorityApprove,
					#[codec(index = 1)]
					SuperMajorityAgainst,
					#[codec(index = 2)]
					SimpleMajority,
				}
			}
		}
		pub mod pallet_collective {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Proposed {
						account: ::sp_core::crypto::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: runtime_types::primitive_types::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					Voted {
						account: ::sp_core::crypto::AccountId32,
						proposal_hash: runtime_types::primitive_types::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Approved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec(index = 3)]
					Disapproved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec(index = 4)]
					Executed {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					MemberExecuted {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					Closed {
						proposal_hash: runtime_types::primitive_types::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event2 {
					#[codec(index = 0)]
					Proposed {
						account: ::sp_core::crypto::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: runtime_types::primitive_types::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					Voted {
						account: ::sp_core::crypto::AccountId32,
						proposal_hash: runtime_types::primitive_types::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Approved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec(index = 3)]
					Disapproved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec(index = 4)]
					Executed {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					MemberExecuted {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					Closed {
						proposal_hash: runtime_types::primitive_types::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
			}
		}
		pub mod pallet_elections_phragmen {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					NewTerm {
						new_members: ::sp_std::vec::Vec<(
							::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						)>,
					},
					#[codec(index = 1)]
					EmptyTerm,
					#[codec(index = 2)]
					ElectionError,
					#[codec(index = 3)]
					MemberKicked { member: ::sp_core::crypto::AccountId32 },
					#[codec(index = 4)]
					Renounced { candidate: ::sp_core::crypto::AccountId32 },
					#[codec(index = 5)]
					CandidateSlashed {
						candidate: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					SeatHolderSlashed {
						seat_holder: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					MemberAdded,
					#[codec(index = 1)]
					MemberRemoved,
					#[codec(index = 2)]
					MembersSwapped,
					#[codec(index = 3)]
					MembersReset,
					#[codec(index = 4)]
					KeyChanged,
					#[codec(index = 5)]
					Dummy,
				}
			}
		}
		pub mod pallet_treasury {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Proposed { proposal_index: ::core::primitive::u32 },
					#[codec(index = 1)]
					Spending { budget_remaining: ::core::primitive::u128 },
					#[codec(index = 2)]
					Awarded {
						proposal_index: ::core::primitive::u32,
						award: ::core::primitive::u128,
						account: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 3)]
					Rejected {
						proposal_index: ::core::primitive::u32,
						slashed: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Burnt { burnt_funds: ::core::primitive::u128 },
					#[codec(index = 5)]
					Rollover { rollover_balance: ::core::primitive::u128 },
					#[codec(index = 6)]
					Deposit { value: ::core::primitive::u128 },
					#[codec(index = 7)]
					SpendApproved {
						proposal_index: ::core::primitive::u32,
						amount: ::core::primitive::u128,
						beneficiary: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 8)]
					UpdatedInactive {
						reactivated: ::core::primitive::u128,
						deactivated: ::core::primitive::u128,
					},
				}
			}
		}
		pub mod pallet_identity {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					IdentitySet { who: ::sp_core::crypto::AccountId32 },
					#[codec(index = 1)]
					IdentityCleared {
						who: ::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					IdentityKilled {
						who: ::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					JudgementRequested {
						who: ::sp_core::crypto::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					JudgementUnrequested {
						who: ::sp_core::crypto::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					JudgementGiven {
						target: ::sp_core::crypto::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					RegistrarAdded { registrar_index: ::core::primitive::u32 },
					#[codec(index = 7)]
					SubIdentityAdded {
						sub: ::sp_core::crypto::AccountId32,
						main: ::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					SubIdentityRemoved {
						sub: ::sp_core::crypto::AccountId32,
						main: ::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					SubIdentityRevoked {
						sub: ::sp_core::crypto::AccountId32,
						main: ::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
				}
			}
		}
		pub mod pallet_society {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Founded { founder: ::sp_core::crypto::AccountId32 },
					#[codec(index = 1)]
					Bid {
						candidate_id: ::sp_core::crypto::AccountId32,
						offer: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Vouch {
						candidate_id: ::sp_core::crypto::AccountId32,
						offer: ::core::primitive::u128,
						vouching: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 3)]
					AutoUnbid { candidate: ::sp_core::crypto::AccountId32 },
					#[codec(index = 4)]
					Unbid { candidate: ::sp_core::crypto::AccountId32 },
					#[codec(index = 5)]
					Unvouch { candidate: ::sp_core::crypto::AccountId32 },
					#[codec(index = 6)]
					Inducted {
						primary: ::sp_core::crypto::AccountId32,
						candidates: ::sp_std::vec::Vec<::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 7)]
					SuspendedMemberJudgement {
						who: ::sp_core::crypto::AccountId32,
						judged: ::core::primitive::bool,
					},
					#[codec(index = 8)]
					CandidateSuspended { candidate: ::sp_core::crypto::AccountId32 },
					#[codec(index = 9)]
					MemberSuspended { member: ::sp_core::crypto::AccountId32 },
					#[codec(index = 10)]
					Challenged { member: ::sp_core::crypto::AccountId32 },
					#[codec(index = 11)]
					Vote {
						candidate: ::sp_core::crypto::AccountId32,
						voter: ::sp_core::crypto::AccountId32,
						vote: ::core::primitive::bool,
					},
					#[codec(index = 12)]
					DefenderVote {
						voter: ::sp_core::crypto::AccountId32,
						vote: ::core::primitive::bool,
					},
					#[codec(index = 13)]
					NewParams {
						params: runtime_types::pallet_society::GroupParams<::core::primitive::u128>,
					},
					#[codec(index = 14)]
					Unfounded { founder: ::sp_core::crypto::AccountId32 },
					#[codec(index = 15)]
					Deposit { value: ::core::primitive::u128 },
					#[codec(index = 16)]
					Elevated {
						member: ::sp_core::crypto::AccountId32,
						rank: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub struct GroupParams<_0> {
				pub max_members: ::core::primitive::u32,
				pub max_intake: ::core::primitive::u32,
				pub max_strikes: ::core::primitive::u32,
				pub candidate_deposit: _0,
			}
		}
		pub mod pallet_recovery {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					RecoveryCreated { account: ::sp_core::crypto::AccountId32 },
					#[codec(index = 1)]
					RecoveryInitiated {
						lost_account: ::sp_core::crypto::AccountId32,
						rescuer_account: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 2)]
					RecoveryVouched {
						lost_account: ::sp_core::crypto::AccountId32,
						rescuer_account: ::sp_core::crypto::AccountId32,
						sender: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 3)]
					RecoveryClosed {
						lost_account: ::sp_core::crypto::AccountId32,
						rescuer_account: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 4)]
					AccountRecovered {
						lost_account: ::sp_core::crypto::AccountId32,
						rescuer_account: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 5)]
					RecoveryRemoved { lost_account: ::sp_core::crypto::AccountId32 },
				}
			}
		}
		pub mod pallet_utility {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					BatchInterrupted {
						index: ::core::primitive::u32,
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec(index = 1)]
					BatchCompleted,
					#[codec(index = 2)]
					BatchCompletedWithErrors,
					#[codec(index = 3)]
					ItemCompleted,
					#[codec(index = 4)]
					ItemFailed { error: runtime_types::sp_runtime::DispatchError },
					#[codec(index = 5)]
					DispatchedAs {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_vesting {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					VestingUpdated {
						account: ::sp_core::crypto::AccountId32,
						unvested: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					VestingCompleted { account: ::sp_core::crypto::AccountId32 },
				}
			}
		}
		pub mod pallet_scheduler {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Scheduled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 1)]
					Canceled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 2)]
					Dispatched {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					CallUnavailable {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 4)]
					PeriodicFailed {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 5)]
					PermanentlyOverweight {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
		}
		pub mod pallet_proxy {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					ProxyExecuted {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					PureCreated {
						pure: ::sp_core::crypto::AccountId32,
						who: ::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						disambiguation_index: ::core::primitive::u16,
					},
					#[codec(index = 2)]
					Announced {
						real: ::sp_core::crypto::AccountId32,
						proxy: ::sp_core::crypto::AccountId32,
						call_hash: runtime_types::primitive_types::H256,
					},
					#[codec(index = 3)]
					ProxyAdded {
						delegator: ::sp_core::crypto::AccountId32,
						delegatee: ::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					ProxyRemoved {
						delegator: ::sp_core::crypto::AccountId32,
						delegatee: ::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
				}
			}
		}
		pub mod pallet_multisig {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					NewMultisig {
						approving: ::sp_core::crypto::AccountId32,
						multisig: ::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 1)]
					MultisigApproval {
						approving: ::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 2)]
					MultisigExecuted {
						approving: ::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					MultisigCancelled {
						cancelling: ::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub struct Timepoint<_0> {
				pub height: _0,
				pub index: ::core::primitive::u32,
			}
		}
		pub mod pallet_preimage {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Noted { hash: runtime_types::primitive_types::H256 },
					#[codec(index = 1)]
					Requested { hash: runtime_types::primitive_types::H256 },
					#[codec(index = 2)]
					Cleared { hash: runtime_types::primitive_types::H256 },
				}
			}
		}
		pub mod pallet_bounties {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					BountyProposed { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					BountyRejected { index: ::core::primitive::u32, bond: ::core::primitive::u128 },
					#[codec(index = 2)]
					BountyBecameActive { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					BountyAwarded {
						index: ::core::primitive::u32,
						beneficiary: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 4)]
					BountyClaimed {
						index: ::core::primitive::u32,
						payout: ::core::primitive::u128,
						beneficiary: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 5)]
					BountyCanceled { index: ::core::primitive::u32 },
					#[codec(index = 6)]
					BountyExtended { index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_child_bounties {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Added { index: ::core::primitive::u32, child_index: ::core::primitive::u32 },
					#[codec(index = 1)]
					Awarded {
						index: ::core::primitive::u32,
						child_index: ::core::primitive::u32,
						beneficiary: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 2)]
					Claimed {
						index: ::core::primitive::u32,
						child_index: ::core::primitive::u32,
						payout: ::core::primitive::u128,
						beneficiary: ::sp_core::crypto::AccountId32,
					},
					#[codec(index = 3)]
					Canceled { index: ::core::primitive::u32, child_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_tips {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					NewTip { tip_hash: runtime_types::primitive_types::H256 },
					#[codec(index = 1)]
					TipClosing { tip_hash: runtime_types::primitive_types::H256 },
					#[codec(index = 2)]
					TipClosed {
						tip_hash: runtime_types::primitive_types::H256,
						who: ::sp_core::crypto::AccountId32,
						payout: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					TipRetracted { tip_hash: runtime_types::primitive_types::H256 },
					#[codec(index = 4)]
					TipSlashed {
						tip_hash: runtime_types::primitive_types::H256,
						finder: ::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
				}
			}
		}
		pub mod pallet_nis {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					BidPlaced {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						duration: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					BidRetracted {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						duration: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					BidDropped {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						duration: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					Issued {
						index: ::core::primitive::u32,
						expiry: ::core::primitive::u32,
						who: ::sp_core::crypto::AccountId32,
						proportion: runtime_types::sp_arithmetic::per_things::Perquintill,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Thawed {
						index: ::core::primitive::u32,
						who: ::sp_core::crypto::AccountId32,
						proportion: runtime_types::sp_arithmetic::per_things::Perquintill,
						amount: ::core::primitive::u128,
						dropped: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					Funded { deficit: ::core::primitive::u128 },
					#[codec(index = 6)]
					Transferred {
						from: ::sp_core::crypto::AccountId32,
						to: ::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
				}
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub struct CandidateHash(pub runtime_types::primitive_types::H256);
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::sp_std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::sp_std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::sp_std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v5 {
				use super::runtime_types;
				// 	pub mod assignment_app {
				// 		use super::runtime_types;
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				// 	}
				pub mod collator_app {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
				// 	pub mod executor_params {
				// 		use super::runtime_types;
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub enum ExecutorParam {
				// 			#[codec(index = 1)]
				// 			MaxMemoryPages(::core::primitive::u32),
				// 			#[codec(index = 2)]
				// 			StackLogicalMax(::core::primitive::u32),
				// 			#[codec(index = 3)]
				// 			StackNativeMax(::core::primitive::u32),
				// 			#[codec(index = 4)]
				// 			PrecheckingMaxMemory(::core::primitive::u64),
				// 			#[codec(index = 5)]
				// 			PvfPrepTimeout(
				// 				runtime_types::polkadot_primitives::v5::PvfPrepTimeoutKind,
				// 				::core::primitive::u64,
				// 			),
				// 			#[codec(index = 6)]
				// 			PvfExecTimeout(
				// 				runtime_types::polkadot_primitives::v5::PvfExecTimeoutKind,
				// 				::core::primitive::u64,
				// 			),
				// 			#[codec(index = 7)]
				// 			WasmExtBulkMemory,
				// 		}
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub struct ExecutorParams(
				// 			pub  ::sp_std::vec::Vec<
				// 				runtime_types::polkadot_primitives::v5::executor_params::ExecutorParam,
				// 			>,
				// 		);
				// 	}
				// 	pub mod signed {
				// 		use super::runtime_types;
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub struct UncheckedSigned<_0, _1> {
				// 			pub payload: _0,
				// 			pub validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex,
				// 			pub signature:
				// 				runtime_types::polkadot_primitives::v5::validator_app::Signature,
				// 			#[codec(skip)]
				// 			pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
				// 		}
				// 	}
				// 	pub mod slashing {
				// 		use super::runtime_types;
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub struct DisputeProof {
				// 			pub time_slot:
				// 				runtime_types::polkadot_primitives::v5::slashing::DisputesTimeSlot,
				// 			pub kind:
				// 				runtime_types::polkadot_primitives::v5::slashing::SlashingOffenceKind,
				// 			pub validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex,
				// 			pub validator_id:
				// 				runtime_types::polkadot_primitives::v5::validator_app::Public,
				// 		}
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub struct DisputesTimeSlot {
				// 			pub session_index: ::core::primitive::u32,
				// 			pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
				// 		}
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub struct OpaqueKeyOwnershipProof(pub ::sp_std::vec::Vec<::core::primitive::u8>);
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub struct PendingSlashes {
				// 			pub keys: ::subxt::utils::KeyedVec<
				// 				runtime_types::polkadot_primitives::v5::ValidatorIndex,
				// 				runtime_types::polkadot_primitives::v5::validator_app::Public,
				// 			>,
				// 			pub kind:
				// 				runtime_types::polkadot_primitives::v5::slashing::SlashingOffenceKind,
				// 		}
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub enum SlashingOffenceKind {
				// 			#[codec(index = 0)]
				// 			ForInvalid,
				// 			#[codec(index = 1)]
				// 			AgainstValid,
				// 		}
				// 	}
				// 	pub mod validator_app {
				// 		use super::runtime_types;
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				// 		#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 		pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct Assignment {
				// 		pub para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct AvailabilityBitfield(
				// 		pub  ::subxt::utils::bits::DecodedBits<
				// 			::core::primitive::u8,
				// 			::subxt::utils::bits::Lsb0,
				// 		>,
				// 	);
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct BackedCandidate<_0> {
				// 		pub candidate:
				// 			runtime_types::polkadot_primitives::v5::CommittedCandidateReceipt<_0>,
				// 		pub validity_votes: ::sp_std::vec::Vec<
				// 			runtime_types::polkadot_primitives::v5::ValidityAttestation,
				// 		>,
				// 		pub validator_indices: ::subxt::utils::bits::DecodedBits<
				// 			::core::primitive::u8,
				// 			::subxt::utils::bits::Lsb0,
				// 		>,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct CandidateCommitments<_0> {
				// 		pub upward_messages:
				// 			runtime_types::bounded_collections::bounded_vec::BoundedVec<
				// 				::sp_std::vec::Vec<::core::primitive::u8>,
				// 			>,
				// 		pub horizontal_messages:
				// 			runtime_types::bounded_collections::bounded_vec::BoundedVec<
				// 				runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
				// 					runtime_types::polkadot_parachain_primitives::primitives::Id,
				// 				>,
				// 			>,
				// 		pub new_validation_code: ::core::option::Option<
				// 			runtime_types::polkadot_parachain_primitives::primitives::ValidationCode,
				// 		>,
				// 		pub head_data:
				// 			runtime_types::polkadot_parachain_primitives::primitives::HeadData,
				// 		pub processed_downward_messages: ::core::primitive::u32,
				// 		pub hrmp_watermark: _0,
				// 	}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct CandidateDescriptor < _0 > { pub para_id : runtime_types :: polkadot_parachain_primitives :: primitives :: Id , pub relay_parent : _0 , pub collator : runtime_types :: polkadot_primitives :: v5 :: collator_app :: Public , pub persisted_validation_data_hash : runtime_types::primitive_types::H256 , pub pov_hash : runtime_types::primitive_types::H256 , pub erasure_root : runtime_types::primitive_types::H256 , pub signature : runtime_types :: polkadot_primitives :: v5 :: collator_app :: Signature , pub para_head : runtime_types::primitive_types::H256 , pub validation_code_hash : runtime_types :: polkadot_parachain_primitives :: primitives :: ValidationCodeHash , }
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum CandidateEvent<_0> {
				// 		#[codec(index = 0)]
				// 		CandidateBacked(
				// 			runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
				// 			runtime_types::polkadot_parachain_primitives::primitives::HeadData,
				// 			runtime_types::polkadot_primitives::v5::CoreIndex,
				// 			runtime_types::polkadot_primitives::v5::GroupIndex,
				// 		),
				// 		#[codec(index = 1)]
				// 		CandidateIncluded(
				// 			runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
				// 			runtime_types::polkadot_parachain_primitives::primitives::HeadData,
				// 			runtime_types::polkadot_primitives::v5::CoreIndex,
				// 			runtime_types::polkadot_primitives::v5::GroupIndex,
				// 		),
				// 		#[codec(index = 2)]
				// 		CandidateTimedOut(
				// 			runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
				// 			runtime_types::polkadot_parachain_primitives::primitives::HeadData,
				// 			runtime_types::polkadot_primitives::v5::CoreIndex,
				// 		),
				// 	}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct CandidateReceipt<_0> {
					pub descriptor: runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
					pub commitments_hash: runtime_types::primitive_types::H256,
				}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct CommittedCandidateReceipt<_0> {
				// 		pub descriptor: runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
				// 		pub commitments: runtime_types::polkadot_primitives::v5::CandidateCommitments<
				// 			::core::primitive::u32,
				// 		>,
				// 	}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					// :: subxt :: ext :: codec :: CompactAs,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct CoreIndex(pub ::core::primitive::u32);
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum CoreOccupied<_0> {
				// 		#[codec(index = 0)]
				// 		Free,
				// 		#[codec(index = 1)]
				// 		Paras(runtime_types::polkadot_primitives::v5::ParasEntry<_0>),
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum CoreState<_0, _1> {
				// 		#[codec(index = 0)]
				// 		Occupied(runtime_types::polkadot_primitives::v5::OccupiedCore<_0, _1>),
				// 		#[codec(index = 1)]
				// 		Scheduled(runtime_types::polkadot_primitives::v5::ScheduledCore),
				// 		#[codec(index = 2)]
				// 		Free,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct DisputeState<_0> {
				// 		pub validators_for: ::subxt::utils::bits::DecodedBits<
				// 			::core::primitive::u8,
				// 			::subxt::utils::bits::Lsb0,
				// 		>,
				// 		pub validators_against: ::subxt::utils::bits::DecodedBits<
				// 			::core::primitive::u8,
				// 			::subxt::utils::bits::Lsb0,
				// 		>,
				// 		pub start: _0,
				// 		pub concluded_at: ::core::option::Option<_0>,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum DisputeStatement {
				// 		#[codec(index = 0)]
				// 		Valid(runtime_types::polkadot_primitives::v5::ValidDisputeStatementKind),
				// 		#[codec(index = 1)]
				// 		Invalid(runtime_types::polkadot_primitives::v5::InvalidDisputeStatementKind),
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct DisputeStatementSet {
				// 		pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
				// 		pub session: ::core::primitive::u32,
				// 		pub statements: ::sp_std::vec::Vec<(
				// 			runtime_types::polkadot_primitives::v5::DisputeStatement,
				// 			runtime_types::polkadot_primitives::v5::ValidatorIndex,
				// 			runtime_types::polkadot_primitives::v5::validator_app::Signature,
				// 		)>,
				// 	}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					// :: subxt :: ext :: codec :: CompactAs,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct GroupIndex(pub ::core::primitive::u32);
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct GroupRotationInfo<_0> {
				// 		pub session_start_block: _0,
				// 		pub group_rotation_frequency: _0,
				// 		pub now: _0,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct IndexedVec<_0, _1>(
				// 		pub ::sp_std::vec::Vec<_1>,
				// 		#[codec(skip)] pub ::core::marker::PhantomData<_0>,
				// 	);
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct InherentData<_0> {
				// 		pub bitfields: ::sp_std::vec::Vec<
				// 			runtime_types::polkadot_primitives::v5::signed::UncheckedSigned<
				// 				runtime_types::polkadot_primitives::v5::AvailabilityBitfield,
				// 				runtime_types::polkadot_primitives::v5::AvailabilityBitfield,
				// 			>,
				// 		>,
				// 		pub backed_candidates: ::sp_std::vec::Vec<
				// 			runtime_types::polkadot_primitives::v5::BackedCandidate<
				// 				::subxt::utils::H256,
				// 			>,
				// 		>,
				// 		pub disputes: ::sp_std::vec::Vec<
				// 			runtime_types::polkadot_primitives::v5::DisputeStatementSet,
				// 		>,
				// 		pub parent_header: _0,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum InvalidDisputeStatementKind {
				// 		#[codec(index = 0)]
				// 		Explicit,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct OccupiedCore<_0, _1> {
				// 		pub next_up_on_available: ::core::option::Option<
				// 			runtime_types::polkadot_primitives::v5::ScheduledCore,
				// 		>,
				// 		pub occupied_since: _1,
				// 		pub time_out_at: _1,
				// 		pub next_up_on_time_out: ::core::option::Option<
				// 			runtime_types::polkadot_primitives::v5::ScheduledCore,
				// 		>,
				// 		pub availability: ::subxt::utils::bits::DecodedBits<
				// 			::core::primitive::u8,
				// 			::subxt::utils::bits::Lsb0,
				// 		>,
				// 		pub group_responsible: runtime_types::polkadot_primitives::v5::GroupIndex,
				// 		pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
				// 		pub candidate_descriptor:
				// 			runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum OccupiedCoreAssumption {
				// 		#[codec(index = 0)]
				// 		Included,
				// 		#[codec(index = 1)]
				// 		TimedOut,
				// 		#[codec(index = 2)]
				// 		Free,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct ParasEntry<_0> {
				// 		pub assignment: runtime_types::polkadot_primitives::v5::Assignment,
				// 		pub availability_timeouts: ::core::primitive::u32,
				// 		pub ttl: _0,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct PersistedValidationData<_0, _1> {
				// 		pub parent_head:
				// 			runtime_types::polkadot_parachain_primitives::primitives::HeadData,
				// 		pub relay_parent_number: _1,
				// 		pub relay_parent_storage_root: _0,
				// 		pub max_pov_size: ::core::primitive::u32,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct PvfCheckStatement { pub accept : :: core :: primitive :: bool , pub subject : runtime_types :: polkadot_parachain_primitives :: primitives :: ValidationCodeHash , pub session_index : :: core :: primitive :: u32 , pub validator_index : runtime_types :: polkadot_primitives :: v5 :: ValidatorIndex , }
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum PvfExecTimeoutKind {
				// 		#[codec(index = 0)]
				// 		Backing,
				// 		#[codec(index = 1)]
				// 		Approval,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum PvfPrepTimeoutKind {
				// 		#[codec(index = 0)]
				// 		Precheck,
				// 		#[codec(index = 1)]
				// 		Lenient,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct ScheduledCore {
				// 		pub para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
				// 		pub collator: ::core::option::Option<
				// 			runtime_types::polkadot_primitives::v5::collator_app::Public,
				// 		>,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct ScrapedOnChainVotes<_0> {
				// 		pub session: ::core::primitive::u32,
				// 		pub backing_validators_per_candidate: ::sp_std::vec::Vec<(
				// 			runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
				// 			::sp_std::vec::Vec<(
				// 				runtime_types::polkadot_primitives::v5::ValidatorIndex,
				// 				runtime_types::polkadot_primitives::v5::ValidityAttestation,
				// 			)>,
				// 		)>,
				// 		pub disputes: ::sp_std::vec::Vec<
				// 			runtime_types::polkadot_primitives::v5::DisputeStatementSet,
				// 		>,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct SessionInfo {
				// 		pub active_validator_indices:
				// 			::sp_std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
				// 		pub random_seed: [::core::primitive::u8; 32usize],
				// 		pub dispute_period: ::core::primitive::u32,
				// 		pub validators: runtime_types::polkadot_primitives::v5::IndexedVec<
				// 			runtime_types::polkadot_primitives::v5::ValidatorIndex,
				// 			runtime_types::polkadot_primitives::v5::validator_app::Public,
				// 		>,
				// 		pub discovery_keys:
				// 			::sp_std::vec::Vec<runtime_types::sp_authority_discovery::app::Public>,
				// 		pub assignment_keys: ::sp_std::vec::Vec<
				// 			runtime_types::polkadot_primitives::v5::assignment_app::Public,
				// 		>,
				// 		pub validator_groups: runtime_types::polkadot_primitives::v5::IndexedVec<
				// 			runtime_types::polkadot_primitives::v5::GroupIndex,
				// 			::sp_std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
				// 		>,
				// 		pub n_cores: ::core::primitive::u32,
				// 		pub zeroth_delay_tranche_width: ::core::primitive::u32,
				// 		pub relay_vrf_modulo_samples: ::core::primitive::u32,
				// 		pub n_delay_tranches: ::core::primitive::u32,
				// 		pub no_show_slots: ::core::primitive::u32,
				// 		pub needed_approvals: ::core::primitive::u32,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum UpgradeGoAhead {
				// 		#[codec(index = 0)]
				// 		Abort,
				// 		#[codec(index = 1)]
				// 		GoAhead,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum UpgradeRestriction {
				// 		#[codec(index = 0)]
				// 		Present,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum ValidDisputeStatementKind {
				// 		#[codec(index = 0)]
				// 		Explicit,
				// 		#[codec(index = 1)]
				// 		BackingSeconded(::subxt::utils::H256),
				// 		#[codec(index = 2)]
				// 		BackingValid(::subxt::utils::H256),
				// 		#[codec(index = 3)]
				// 		ApprovalChecking,
				// 	}
				// 	#[derive(
				// 		:: codec :: Decode,
				// 		:: codec :: Encode,
				// 		:: subxt :: ext :: codec :: CompactAs,
				// 		Clone,
				// 		Debug,
				// 		PartialEq,
				// 	)]
				// 	pub struct ValidatorIndex(pub ::core::primitive::u32);
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub enum ValidityAttestation {
				// 		#[codec(index = 1)]
				// 		Implicit(runtime_types::polkadot_primitives::v5::validator_app::Signature),
				// 		#[codec(index = 2)]
				// 		Explicit(runtime_types::polkadot_primitives::v5::validator_app::Signature),
				// 	}
				// }
				// pub mod vstaging {
				// 	use super::runtime_types;
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct AsyncBackingParams {
				// 		pub max_candidate_depth: ::core::primitive::u32,
				// 		pub allowed_ancestry_len: ::core::primitive::u32,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct BackingState<_0, _1> {
				// 		pub constraints: runtime_types::polkadot_primitives::vstaging::Constraints<_1>,
				// 		pub pending_availability: ::sp_std::vec::Vec<
				// 			runtime_types::polkadot_primitives::vstaging::CandidatePendingAvailability<
				// 				_0,
				// 				_1,
				// 			>,
				// 		>,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct CandidatePendingAvailability<_0, _1> {
				// 		pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
				// 		pub descriptor: runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
				// 		pub commitments:
				// 			runtime_types::polkadot_primitives::v5::CandidateCommitments<_1>,
				// 		pub relay_parent_number: _1,
				// 		pub max_pov_size: ::core::primitive::u32,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct Constraints < _0 > { pub min_relay_parent_number : _0 , pub max_pov_size : :: core :: primitive :: u32 , pub max_code_size : :: core :: primitive :: u32 , pub ump_remaining : :: core :: primitive :: u32 , pub ump_remaining_bytes : :: core :: primitive :: u32 , pub max_ump_num_per_candidate : :: core :: primitive :: u32 , pub dmp_remaining_messages : :: std :: vec :: Vec < _0 > , pub hrmp_inbound : runtime_types :: polkadot_primitives :: vstaging :: InboundHrmpLimitations < _0 > , pub hrmp_channels_out : :: std :: vec :: Vec < (runtime_types :: polkadot_parachain_primitives :: primitives :: Id , runtime_types :: polkadot_primitives :: vstaging :: OutboundHrmpChannelLimitations ,) > , pub max_hrmp_num_per_candidate : :: core :: primitive :: u32 , pub required_parent : runtime_types :: polkadot_parachain_primitives :: primitives :: HeadData , pub validation_code_hash : runtime_types :: polkadot_parachain_primitives :: primitives :: ValidationCodeHash , pub upgrade_restriction : :: core :: option :: Option < runtime_types :: polkadot_primitives :: v5 :: UpgradeRestriction > , pub future_validation_code : :: core :: option :: Option < (_0 , runtime_types :: polkadot_parachain_primitives :: primitives :: ValidationCodeHash ,) > , }
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct InboundHrmpLimitations<_0> {
				// 		pub valid_watermarks: ::sp_std::vec::Vec<_0>,
				// 	}
				// 	#[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// 	pub struct OutboundHrmpChannelLimitations {
				// 		pub bytes_remaining: ::core::primitive::u32,
				// 		pub messages_remaining: ::core::primitive::u32,
				// 	}
			}
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Public(pub [::core::primitive::u8; 33usize]);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub mod vrf {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct VrfSignature {
						pub output: [::core::primitive::u8; 32usize],
						pub proof: [::core::primitive::u8; 64usize],
					}
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub struct OpaqueMetadata(pub ::sp_std::vec::Vec<::core::primitive::u8>);
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub enum Void {}
		}
		pub mod pallet_message_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					# [codec (index = 0)] ProcessingFailed { id : [:: core :: primitive :: u8 ; 32usize] , origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , error : runtime_types :: frame_support :: traits :: messages :: ProcessMessageError , } , # [codec (index = 1)] Processed { id : [:: core :: primitive :: u8 ; 32usize] , origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , weight_used : :: sp_weights :: Weight , success : :: core :: primitive :: bool , } , # [codec (index = 2)] OverweightEnqueued { id : [:: core :: primitive :: u8 ; 32usize] , origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , page_index : :: core :: primitive :: u32 , message_index : :: core :: primitive :: u32 , } , # [codec (index = 3)] PageReaped { origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , index : :: core :: primitive :: u32 , } , }
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Attempted { outcome: runtime_types::staging_xcm::v3::traits::Outcome },
					#[codec(index = 1)]
					Sent {
						origin: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						destination: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						message: runtime_types::staging_xcm::v3::Xcm,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 2)]
					UnexpectedResponse {
						origin: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 3)]
					ResponseReady {
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v3::Response,
					},
					#[codec(index = 4)]
					Notified {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
					},
					#[codec(index = 5)]
					NotifyOverweight {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
						actual_weight: ::sp_weights::Weight,
						max_budgeted_weight: ::sp_weights::Weight,
					},
					#[codec(index = 6)]
					NotifyDispatchError {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
					},
					#[codec(index = 7)]
					NotifyDecodeFailed {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
					},
					#[codec(index = 8)]
					InvalidResponder {
						origin: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						query_id: ::core::primitive::u64,
						expected_location: ::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 9)]
					InvalidResponderVersion {
						origin: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 10)]
					ResponseTaken { query_id: ::core::primitive::u64 },
					#[codec(index = 11)]
					AssetsTrapped {
						hash: runtime_types::primitive_types::H256,
						origin: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						assets: runtime_types::staging_xcm::VersionedMultiAssets,
					},
					#[codec(index = 12)]
					VersionChangeNotified {
						destination: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						result: ::core::primitive::u32,
						cost: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 13)]
					SupportedVersionChanged {
						location: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						version: ::core::primitive::u32,
					},
					#[codec(index = 14)]
					NotifyTargetSendFail {
						location: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						query_id: ::core::primitive::u64,
						error: runtime_types::staging_xcm::v3::traits::Error,
					},
					#[codec(index = 15)]
					NotifyTargetMigrationFail {
						location: runtime_types::staging_xcm::VersionedMultiLocation,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 16)]
					InvalidQuerierVersion {
						origin: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 17)]
					InvalidQuerier {
						origin: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						query_id: ::core::primitive::u64,
						expected_querier:
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						maybe_actual_querier: ::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 18)]
					VersionNotifyStarted {
						destination: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						cost: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 19)]
					VersionNotifyRequested {
						destination: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						cost: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 20)]
					VersionNotifyUnrequested {
						destination: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						cost: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 21)]
					FeesPaid {
						paying: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						fees: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
					},
					#[codec(index = 22)]
					AssetsClaimed {
						hash: runtime_types::primitive_types::H256,
						origin: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						assets: runtime_types::staging_xcm::VersionedMultiAssets,
					},
				}
			}
		}
		pub mod staging_xcm {
			use super::runtime_types;
			pub mod double_encoded {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct DoubleEncoded {
					pub encoded: ::sp_std::vec::Vec<::core::primitive::u8>,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct DoubleEncoded2 {
					pub encoded: ::sp_std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v2 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: runtime_types::staging_xcm::v2::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: runtime_types::staging_xcm::v2::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: runtime_types::staging_xcm::v2::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey(
							runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::staging_xcm::v2::BodyId,
							part: runtime_types::staging_xcm::v2::BodyPart,
						},
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::staging_xcm::v2::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::sp_std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
						#[codec(index = 6)]
						Blob(::sp_std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::staging_xcm::v2::multiasset::AssetInstance),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct MultiAsset {
						pub id: runtime_types::staging_xcm::v2::multiasset::AssetId,
						pub fun: runtime_types::staging_xcm::v2::multiasset::Fungibility,
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::staging_xcm::v2::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::staging_xcm::v2::multiasset::WildMultiAsset),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct MultiAssets(
						pub  ::sp_std::vec::Vec<
							runtime_types::staging_xcm::v2::multiasset::MultiAsset,
						>,
					);
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::staging_xcm::v2::multiasset::AssetId,
							fun: runtime_types::staging_xcm::v2::multiasset::WildFungibility,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::staging_xcm::v2::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
							runtime_types::staging_xcm::v2::junction::Junction,
						),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::staging_xcm::v2::multilocation::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						MultiLocationFull,
						#[codec(index = 5)]
						MultiLocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						UnhandledXcmVersion,
						#[codec(index = 23)]
						WeightLimitReached(::core::primitive::u64),
						#[codec(index = 24)]
						Barrier,
						#[codec(index = 25)]
						WeightNotComputable,
					}
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum BodyId {
					#[codec(index = 0)]
					Unit,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Index(#[codec(compact)] ::core::primitive::u32),
					#[codec(index = 3)]
					Executive,
					#[codec(index = 4)]
					Technical,
					#[codec(index = 5)]
					Legislative,
					#[codec(index = 6)]
					Judicial,
					#[codec(index = 7)]
					Defense,
					#[codec(index = 8)]
					Administration,
					#[codec(index = 9)]
					Treasury,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum BodyPart {
					#[codec(index = 0)]
					Voice,
					#[codec(index = 1)]
					Members {
						#[codec(compact)]
						count: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Fraction {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					AtLeastProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					MoreThanProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::staging_xcm::v2::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::staging_xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::staging_xcm::v2::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::staging_xcm::v2::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::staging_xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::staging_xcm::v2::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::staging_xcm::v2::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::staging_xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::staging_xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::staging_xcm::v2::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::staging_xcm::v2::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Instruction2 {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::staging_xcm::v2::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::staging_xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::staging_xcm::v2::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::staging_xcm::v2::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::staging_xcm::double_encoded::DoubleEncoded2,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::staging_xcm::v2::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::staging_xcm::v2::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::staging_xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::staging_xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::staging_xcm::v2::Xcm2),
					#[codec(index = 22)]
					SetAppendix(runtime_types::staging_xcm::v2::Xcm2),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::staging_xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::staging_xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum NetworkId {
					#[codec(index = 0)]
					Any,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Polkadot,
					#[codec(index = 3)]
					Kusama,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum OriginKind {
					#[codec(index = 0)]
					Native,
					#[codec(index = 1)]
					SovereignAccount,
					#[codec(index = 2)]
					Superuser,
					#[codec(index = 3)]
					Xcm,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::staging_xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::staging_xcm::v2::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(#[codec(compact)] ::core::primitive::u64),
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Xcm(pub ::sp_std::vec::Vec<runtime_types::staging_xcm::v2::Instruction>);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Xcm2(
					pub ::sp_std::vec::Vec<runtime_types::staging_xcm::v2::Instruction2>,
				);
			}
			pub mod v3 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Moniker([::core::primitive::u8; 4usize]),
						#[codec(index = 2)]
						Index(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						Executive,
						#[codec(index = 4)]
						Technical,
						#[codec(index = 5)]
						Legislative,
						#[codec(index = 6)]
						Judicial,
						#[codec(index = 7)]
						Defense,
						#[codec(index = 8)]
						Administration,
						#[codec(index = 9)]
						Treasury,
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum BodyPart {
						#[codec(index = 0)]
						Voice,
						#[codec(index = 1)]
						Members {
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
						#[codec(index = 2)]
						Fraction {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						AtLeastProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						MoreThanProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: ::core::option::Option<
								runtime_types::staging_xcm::v3::junction::NetworkId,
							>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: ::core::option::Option<
								runtime_types::staging_xcm::v3::junction::NetworkId,
							>,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: ::core::option::Option<
								runtime_types::staging_xcm::v3::junction::NetworkId,
							>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::staging_xcm::v3::junction::BodyId,
							part: runtime_types::staging_xcm::v3::junction::BodyPart,
						},
						#[codec(index = 9)]
						GlobalConsensus(runtime_types::staging_xcm::v3::junction::NetworkId),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum NetworkId {
						#[codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
						#[codec(index = 4)]
						Westend,
						#[codec(index = 5)]
						Rococo,
						#[codec(index = 6)]
						Wococo,
						#[codec(index = 7)]
						Ethereum {
							#[codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec(index = 8)]
						BitcoinCore,
						#[codec(index = 9)]
						BitcoinCash,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::staging_xcm::v3::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
							runtime_types::staging_xcm::v3::junction::Junction,
						),
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::staging_xcm::v3::multiasset::AssetInstance),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct MultiAsset {
						pub id: runtime_types::staging_xcm::v3::multiasset::AssetId,
						pub fun: runtime_types::staging_xcm::v3::multiasset::Fungibility,
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::staging_xcm::v3::multiasset::WildMultiAsset),
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct MultiAssets(
						pub  ::sp_std::vec::Vec<
							runtime_types::staging_xcm::v3::multiasset::MultiAsset,
						>,
					);
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::staging_xcm::v3::multiasset::AssetId,
							fun: runtime_types::staging_xcm::v3::multiasset::WildFungibility,
						},
						#[codec(index = 2)]
						AllCounted(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						AllOfCounted {
							id: runtime_types::staging_xcm::v3::multiasset::AssetId,
							fun: runtime_types::staging_xcm::v3::multiasset::WildFungibility,
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::staging_xcm::v3::junctions::Junctions,
					}
				}

				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						LocationFull,
						#[codec(index = 5)]
						LocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						ExpectationFalse,
						#[codec(index = 23)]
						PalletNotFound,
						#[codec(index = 24)]
						NameMismatch,
						#[codec(index = 25)]
						VersionIncompatible,
						#[codec(index = 26)]
						HoldingWouldOverflow,
						#[codec(index = 27)]
						ExportError,
						#[codec(index = 28)]
						ReanchorFailed,
						#[codec(index = 29)]
						NoDeal,
						#[codec(index = 30)]
						FeesNotMet,
						#[codec(index = 31)]
						LockError,
						#[codec(index = 32)]
						NoPermission,
						#[codec(index = 33)]
						Unanchored,
						#[codec(index = 34)]
						NotDepositable,
						#[codec(index = 35)]
						UnhandledXcmVersion,
						#[codec(index = 36)]
						WeightLimitReached(::sp_weights::Weight),
						#[codec(index = 37)]
						Barrier,
						#[codec(index = 38)]
						WeightNotComputable,
						#[codec(index = 39)]
						ExceedsStackLimit,
					}
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete(::sp_weights::Weight),
						#[codec(index = 1)]
						Incomplete(
							::sp_weights::Weight,
							runtime_types::staging_xcm::v3::traits::Error,
						),
						#[codec(index = 2)]
						Error(runtime_types::staging_xcm::v3::traits::Error),
					}
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v3::Response,
						max_weight: ::sp_weights::Weight,
						querier: ::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v3::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::staging_xcm::v2::OriginKind,
						require_weight_at_most: ::sp_weights::Weight,
						call: runtime_types::staging_xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::staging_xcm::v3::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::staging_xcm::v3::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v3::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v3::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::staging_xcm::v3::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::staging_xcm::v3::QueryResponseInfo,
						assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::staging_xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::staging_xcm::v3::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::staging_xcm::v3::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: ::sp_weights::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::staging_xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::staging_xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::sp_std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::staging_xcm::v3::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::sp_std::vec::Vec<::core::primitive::u8>,
						module_name: ::sp_std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::staging_xcm::v3::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::staging_xcm::v3::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::staging_xcm::v3::junction::NetworkId,
						destination: runtime_types::staging_xcm::v3::junctions::Junctions,
						xcm: runtime_types::staging_xcm::v3::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
						target: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::staging_xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				// #[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// pub enum Instruction2 {
				// 	#[codec(index = 0)]
				// 	WithdrawAsset(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
				// 	#[codec(index = 1)]
				// 	ReserveAssetDeposited(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
				// 	#[codec(index = 2)]
				// 	ReceiveTeleportedAsset(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
				// 	#[codec(index = 3)]
				// 	QueryResponse {
				// 		#[codec(compact)]
				// 		query_id: ::core::primitive::u64,
				// 		response: runtime_types::staging_xcm::v3::Response,
				// 		max_weight: ::sp_weights::Weight,
				// 		querier: ::core::option::Option<
				// 			runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 		>,
				// 	},
				// 	#[codec(index = 4)]
				// 	TransferAsset {
				// 		assets: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
				// 		beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 	},
				// 	#[codec(index = 5)]
				// 	TransferReserveAsset {
				// 		assets: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
				// 		dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 		xcm: runtime_types::staging_xcm::v3::Xcm,
				// 	},
				// 	#[codec(index = 6)]
				// 	Transact {
				// 		origin_kind: runtime_types::staging_xcm::v2::OriginKind,
				// 		require_weight_at_most: ::sp_weights::Weight,
				// 		call: runtime_types::staging_xcm::double_encoded::DoubleEncoded2,
				// 	},
				// 	#[codec(index = 7)]
				// 	HrmpNewChannelOpenRequest {
				// 		#[codec(compact)]
				// 		sender: ::core::primitive::u32,
				// 		#[codec(compact)]
				// 		max_message_size: ::core::primitive::u32,
				// 		#[codec(compact)]
				// 		max_capacity: ::core::primitive::u32,
				// 	},
				// 	#[codec(index = 8)]
				// 	HrmpChannelAccepted {
				// 		#[codec(compact)]
				// 		recipient: ::core::primitive::u32,
				// 	},
				// 	#[codec(index = 9)]
				// 	HrmpChannelClosing {
				// 		#[codec(compact)]
				// 		initiator: ::core::primitive::u32,
				// 		#[codec(compact)]
				// 		sender: ::core::primitive::u32,
				// 		#[codec(compact)]
				// 		recipient: ::core::primitive::u32,
				// 	},
				// 	#[codec(index = 10)]
				// 	ClearOrigin,
				// 	#[codec(index = 11)]
				// 	DescendOrigin(runtime_types::staging_xcm::v3::junctions::Junctions),
				// 	#[codec(index = 12)]
				// 	ReportError(runtime_types::staging_xcm::v3::QueryResponseInfo),
				// 	#[codec(index = 13)]
				// 	DepositAsset {
				// 		assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
				// 		beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 	},
				// 	#[codec(index = 14)]
				// 	DepositReserveAsset {
				// 		assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
				// 		dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 		xcm: runtime_types::staging_xcm::v3::Xcm,
				// 	},
				// 	#[codec(index = 15)]
				// 	ExchangeAsset {
				// 		give: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
				// 		want: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
				// 		maximal: ::core::primitive::bool,
				// 	},
				// 	#[codec(index = 16)]
				// 	InitiateReserveWithdraw {
				// 		assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
				// 		reserve: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 		xcm: runtime_types::staging_xcm::v3::Xcm,
				// 	},
				// 	#[codec(index = 17)]
				// 	InitiateTeleport {
				// 		assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
				// 		dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 		xcm: runtime_types::staging_xcm::v3::Xcm,
				// 	},
				// 	#[codec(index = 18)]
				// 	ReportHolding {
				// 		response_info: runtime_types::staging_xcm::v3::QueryResponseInfo,
				// 		assets: runtime_types::staging_xcm::v3::multiasset::MultiAssetFilter,
				// 	},
				// 	#[codec(index = 19)]
				// 	BuyExecution {
				// 		fees: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
				// 		weight_limit: runtime_types::staging_xcm::v3::WeightLimit,
				// 	},
				// 	#[codec(index = 20)]
				// 	RefundSurplus,
				// 	#[codec(index = 21)]
				// 	SetErrorHandler(runtime_types::staging_xcm::v3::Xcm2),
				// 	#[codec(index = 22)]
				// 	SetAppendix(runtime_types::staging_xcm::v3::Xcm2),
				// 	#[codec(index = 23)]
				// 	ClearError,
				// 	#[codec(index = 24)]
				// 	ClaimAsset {
				// 		assets: runtime_types::staging_xcm::v3::multiasset::MultiAssets,
				// 		ticket: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 	},
				// 	#[codec(index = 25)]
				// 	Trap(#[codec(compact)] ::core::primitive::u64),
				// 	#[codec(index = 26)]
				// 	SubscribeVersion {
				// 		#[codec(compact)]
				// 		query_id: ::core::primitive::u64,
				// 		max_response_weight: ::sp_weights::Weight,
				// 	},
				// 	#[codec(index = 27)]
				// 	UnsubscribeVersion,
				// 	#[codec(index = 28)]
				// 	BurnAsset(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
				// 	#[codec(index = 29)]
				// 	ExpectAsset(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
				// 	#[codec(index = 30)]
				// 	ExpectOrigin(
				// 		::core::option::Option<
				// 			runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 		>,
				// 	),
				// 	#[codec(index = 31)]
				// 	ExpectError(
				// 		::core::option::Option<(
				// 			::core::primitive::u32,
				// 			runtime_types::staging_xcm::v3::traits::Error,
				// 		)>,
				// 	),
				// 	#[codec(index = 32)]
				// 	ExpectTransactStatus(runtime_types::staging_xcm::v3::MaybeErrorCode),
				// 	#[codec(index = 33)]
				// 	QueryPallet {
				// 		module_name: ::sp_std::vec::Vec<::core::primitive::u8>,
				// 		response_info: runtime_types::staging_xcm::v3::QueryResponseInfo,
				// 	},
				// 	#[codec(index = 34)]
				// 	ExpectPallet {
				// 		#[codec(compact)]
				// 		index: ::core::primitive::u32,
				// 		name: ::sp_std::vec::Vec<::core::primitive::u8>,
				// 		module_name: ::sp_std::vec::Vec<::core::primitive::u8>,
				// 		#[codec(compact)]
				// 		crate_major: ::core::primitive::u32,
				// 		#[codec(compact)]
				// 		min_crate_minor: ::core::primitive::u32,
				// 	},
				// 	#[codec(index = 35)]
				// 	ReportTransactStatus(runtime_types::staging_xcm::v3::QueryResponseInfo),
				// 	#[codec(index = 36)]
				// 	ClearTransactStatus,
				// 	#[codec(index = 37)]
				// 	UniversalOrigin(runtime_types::staging_xcm::v3::junction::Junction),
				// 	#[codec(index = 38)]
				// 	ExportMessage {
				// 		network: runtime_types::staging_xcm::v3::junction::NetworkId,
				// 		destination: runtime_types::staging_xcm::v3::junctions::Junctions,
				// 		xcm: runtime_types::staging_xcm::v3::Xcm,
				// 	},
				// 	#[codec(index = 39)]
				// 	LockAsset {
				// 		asset: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
				// 		unlocker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 	},
				// 	#[codec(index = 40)]
				// 	UnlockAsset {
				// 		asset: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
				// 		target: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 	},
				// 	#[codec(index = 41)]
				// 	NoteUnlockable {
				// 		asset: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
				// 		owner: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 	},
				// 	#[codec(index = 42)]
				// 	RequestUnlock {
				// 		asset: runtime_types::staging_xcm::v3::multiasset::MultiAsset,
				// 		locker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 	},
				// 	#[codec(index = 43)]
				// 	SetFeesMode { jit_withdraw: ::core::primitive::bool },
				// 	#[codec(index = 44)]
				// 	SetTopic([::core::primitive::u8; 32usize]),
				// 	#[codec(index = 45)]
				// 	ClearTopic,
				// 	#[codec(index = 46)]
				// 	AliasOrigin(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
				// 	#[codec(index = 47)]
				// 	UnpaidExecution {
				// 		weight_limit: runtime_types::staging_xcm::v3::WeightLimit,
				// 		check_origin: ::core::option::Option<
				// 			runtime_types::staging_xcm::v3::multilocation::MultiLocation,
				// 		>,
				// 	},
				// }
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum MaybeErrorCode {
					#[codec(index = 0)]
					Success,
					#[codec(index = 1)]
					Error(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					TruncatedError(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct PalletInfo {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct QueryResponseInfo {
					pub destination: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					#[codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: ::sp_weights::Weight,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::staging_xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::staging_xcm::v3::PalletInfo,
						>,
					),
					#[codec(index = 5)]
					DispatchResult(runtime_types::staging_xcm::v3::MaybeErrorCode),
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(::sp_weights::Weight),
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Xcm(pub ::sp_std::vec::Vec<runtime_types::staging_xcm::v3::Instruction>);
				// #[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// pub struct Xcm2(pub ::sp_std::vec::Vec<runtime_types::staging_xcm::v3::Instruction2>);
			}
			// #[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
			// pub enum VersionedAssetId {
			// 	#[codec(index = 3)]
			// 	V3(runtime_types::staging_xcm::v3::multiasset::AssetId),
			// }
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub enum VersionedMultiAssets {
				#[codec(index = 1)]
				V2(runtime_types::staging_xcm::v2::multiasset::MultiAssets),
				#[codec(index = 3)]
				V3(runtime_types::staging_xcm::v3::multiasset::MultiAssets),
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub enum VersionedMultiLocation {
				#[codec(index = 1)]
				V2(runtime_types::staging_xcm::v2::multilocation::MultiLocation),
				#[codec(index = 3)]
				V3(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
			}
			// #[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
			// pub enum VersionedResponse {
			// 	#[codec(index = 2)]
			// 	V2(runtime_types::staging_xcm::v2::Response),
			// 	#[codec(index = 3)]
			// 	V3(runtime_types::staging_xcm::v3::Response),
			// }
			// #[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
			// pub enum VersionedXcm {
			// 	#[codec(index = 2)]
			// 	V2(runtime_types::staging_xcm::v2::Xcm),
			// 	#[codec(index = 3)]
			// 	V3(runtime_types::staging_xcm::v3::Xcm),
			// }
			// #[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
			// pub enum VersionedXcm2 {
			// 	#[codec(index = 2)]
			// 	V2(runtime_types::staging_xcm::v2::Xcm2),
			// 	#[codec(index = 3)]
			// 	V3(runtime_types::staging_xcm::v3::Xcm2),
			// }
		}

		pub mod polkadot_runtime_common {
			use super::runtime_types;
			pub mod assigned_slots {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						PermanentSlotAssigned(
							runtime_types::polkadot_parachain_primitives::primitives::Id,
						),
						#[codec(index = 1)]
						TemporarySlotAssigned(
							runtime_types::polkadot_parachain_primitives::primitives::Id,
						),
						#[codec(index = 2)]
						MaxPermanentSlotsChanged { slots: ::core::primitive::u32 },
						#[codec(index = 3)]
						MaxTemporarySlotsChanged { slots: ::core::primitive::u32 },
					}
				}
			}
			pub mod auctions {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						AuctionStarted {
							auction_index: ::core::primitive::u32,
							lease_period: ::core::primitive::u32,
							ending: ::core::primitive::u32,
						},
						#[codec(index = 1)]
						AuctionClosed { auction_index: ::core::primitive::u32 },
						#[codec(index = 2)]
						Reserved {
							bidder: ::sp_core::crypto::AccountId32,
							extra_reserved: ::core::primitive::u128,
							total_amount: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						Unreserved {
							bidder: ::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 4)]
						ReserveConfiscated {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
							leaser: ::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 5)]
						BidAccepted {
							bidder: ::sp_core::crypto::AccountId32,
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
							amount: ::core::primitive::u128,
							first_slot: ::core::primitive::u32,
							last_slot: ::core::primitive::u32,
						},
						#[codec(index = 6)]
						WinningOffset {
							auction_index: ::core::primitive::u32,
							block_number: ::core::primitive::u32,
						},
					}
				}
			}
			pub mod claims {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						Claimed {
							who: ::sp_core::crypto::AccountId32,
							ethereum_address:
								runtime_types::polkadot_runtime_common::claims::EthereumAddress,
							amount: ::core::primitive::u128,
						},
					}
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct EthereumAddress(pub [::core::primitive::u8; 20usize]);
			}
			pub mod crowdloan {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						Created {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
						},
						#[codec(index = 1)]
						Contributed {
							who: ::sp_core::crypto::AccountId32,
							fund_index:
								runtime_types::polkadot_parachain_primitives::primitives::Id,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						Withdrew {
							who: ::sp_core::crypto::AccountId32,
							fund_index:
								runtime_types::polkadot_parachain_primitives::primitives::Id,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						PartiallyRefunded {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
						},
						#[codec(index = 4)]
						AllRefunded {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
						},
						#[codec(index = 5)]
						Dissolved {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
						},
						#[codec(index = 6)]
						HandleBidResult {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
							result: ::core::result::Result<
								(),
								runtime_types::sp_runtime::DispatchError,
							>,
						},
						#[codec(index = 7)]
						Edited {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
						},
						#[codec(index = 8)]
						MemoUpdated {
							who: ::sp_core::crypto::AccountId32,
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
							memo: ::sp_std::vec::Vec<::core::primitive::u8>,
						},
						#[codec(index = 9)]
						AddedToNewRaise {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
						},
					}
				}
			}
			pub mod paras_registrar {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						Registered {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
							manager: ::sp_core::crypto::AccountId32,
						},
						#[codec(index = 1)]
						Deregistered {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
						},
						#[codec(index = 2)]
						Reserved {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
							who: ::sp_core::crypto::AccountId32,
						},
						#[codec(index = 3)]
						Swapped {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
							other_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
						},
					}
				}
			}
			pub mod slots {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						NewLeasePeriod { lease_period: ::core::primitive::u32 },
						#[codec(index = 1)]
						Leased {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
							leaser: ::sp_core::crypto::AccountId32,
							period_begin: ::core::primitive::u32,
							period_count: ::core::primitive::u32,
							extra_reserved: ::core::primitive::u128,
							total_amount: ::core::primitive::u128,
						},
					}
				}
			}
		}

		pub mod pallet_state_trie_migration {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Error {
					#[codec(index = 0)]
					MaxSignedLimits,
					#[codec(index = 1)]
					KeyTooLong,
					#[codec(index = 2)]
					NotEnoughFunds,
					#[codec(index = 3)]
					BadWitness,
					#[codec(index = 4)]
					SignedMigrationNotAllowed,
					#[codec(index = 5)]
					BadChildRoot,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Migrated {
						top: ::core::primitive::u32,
						child: ::core::primitive::u32,
						compute:
							runtime_types::pallet_state_trie_migration::pallet::MigrationCompute,
					},
					#[codec(index = 1)]
					Slashed { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 2)]
					AutoMigrationFinished,
					#[codec(index = 3)]
					Halted { error: runtime_types::pallet_state_trie_migration::pallet::Error },
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum MigrationCompute {
					#[codec(index = 0)]
					Signed,
					#[codec(index = 1)]
					Auto,
				}
			}
		}
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_vec {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct BoundedVec<_0>(pub ::sp_std::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct WeakBoundedVec<_0>(pub ::sp_std::vec::Vec<_0>);
			}
		}
		pub mod primitive_types {
			use super::runtime_types;
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub struct H256(pub [::core::primitive::u8; 32usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					CodeUpdated,
					#[codec(index = 3)]
					NewAccount { account: ::sp_core::crypto::AccountId32 },
					#[codec(index = 4)]
					KilledAccount { account: ::sp_core::crypto::AccountId32 },
					#[codec(index = 5)]
					Remarked {
						sender: ::sp_core::crypto::AccountId32,
						hash: runtime_types::primitive_types::H256,
					},
				}
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct DispatchInfo {
					pub weight: ::sp_weights::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod messages {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum ProcessMessageError {
						#[codec(index = 0)]
						BadFormat,
						#[codec(index = 1)]
						Corrupt,
						#[codec(index = 2)]
						Unsupported,
						#[codec(index = 3)]
						Overweight(::sp_weights::Weight),
						#[codec(index = 4)]
						Yield,
					}
				}
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: codec :: Decode,
							:: codec :: Encode,
							Clone,
							Debug,
							PartialEq,
							Eq,
							scale_info::TypeInfo,
						)]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Endowed {
						account: ::sp_core::crypto::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					DustLost {
						account: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Transfer {
						from: ::sp_core::crypto::AccountId32,
						to: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					BalanceSet {
						who: ::sp_core::crypto::AccountId32,
						free: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Reserved {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					Unreserved {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					ReserveRepatriated {
						from: ::sp_core::crypto::AccountId32,
						to: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					Deposit { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 8)]
					Withdraw {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					Slashed { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 10)]
					Minted { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 11)]
					Burned { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 12)]
					Suspended {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 13)]
					Restored {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 14)]
					Upgraded { who: ::sp_core::crypto::AccountId32 },
					#[codec(index = 15)]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					Locked { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 18)]
					Unlocked {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 19)]
					Frozen { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 20)]
					Thawed { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event2 {
					#[codec(index = 0)]
					Endowed {
						account: ::sp_core::crypto::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					DustLost {
						account: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Transfer {
						from: ::sp_core::crypto::AccountId32,
						to: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					BalanceSet {
						who: ::sp_core::crypto::AccountId32,
						free: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Reserved {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					Unreserved {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					ReserveRepatriated {
						from: ::sp_core::crypto::AccountId32,
						to: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					Deposit { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 8)]
					Withdraw {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					Slashed { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 10)]
					Minted { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 11)]
					Burned { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 12)]
					Suspended {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 13)]
					Restored {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 14)]
					Upgraded { who: ::sp_core::crypto::AccountId32 },
					#[codec(index = 15)]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					Locked { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 18)]
					Unlocked {
						who: ::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 19)]
					Frozen { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 20)]
					Thawed { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
				}
			}
		}
		pub mod polkadot_runtime_parachains {
			use super::runtime_types;
			pub mod assigner_on_demand {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						OnDemandOrderPlaced {
							para_id: runtime_types::polkadot_parachain_primitives::primitives::Id,
							spot_price: ::core::primitive::u128,
						},
						#[codec(index = 1)]
						SpotTrafficSet {
							traffic: runtime_types::sp_arithmetic::fixed_point::FixedU128,
						},
					}
				}
			}
			pub mod inclusion {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						CandidateBacked(
							runtime_types::polkadot_primitives::v5::CandidateReceipt<
								runtime_types::primitive_types::H256,
							>,
							runtime_types::polkadot_parachain_primitives::primitives::HeadData,
							runtime_types::polkadot_primitives::v5::CoreIndex,
							runtime_types::polkadot_primitives::v5::GroupIndex,
						),
						#[codec(index = 1)]
						CandidateIncluded(
							runtime_types::polkadot_primitives::v5::CandidateReceipt<
								runtime_types::primitive_types::H256,
							>,
							runtime_types::polkadot_parachain_primitives::primitives::HeadData,
							runtime_types::polkadot_primitives::v5::CoreIndex,
							runtime_types::polkadot_primitives::v5::GroupIndex,
						),
						#[codec(index = 2)]
						CandidateTimedOut(
							runtime_types::polkadot_primitives::v5::CandidateReceipt<
								runtime_types::primitive_types::H256,
							>,
							runtime_types::polkadot_parachain_primitives::primitives::HeadData,
							runtime_types::polkadot_primitives::v5::CoreIndex,
						),
						#[codec(index = 3)]
						UpwardMessagesReceived {
							from: runtime_types::polkadot_parachain_primitives::primitives::Id,
							count: ::core::primitive::u32,
						},
					}
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum AggregateMessageOrigin {
					#[codec(index = 0)]
					Ump(runtime_types::polkadot_runtime_parachains::inclusion::UmpQueueId),
				}
				// #[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// pub struct AvailabilityBitfieldRecord<_0> {
				// 	pub bitfield: runtime_types::polkadot_primitives::v5::AvailabilityBitfield,
				// 	pub submitted_at: _0,
				// }
				// #[derive(:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq)]
				// pub struct CandidatePendingAvailability<_0, _1> {
				// 	pub core: runtime_types::polkadot_primitives::v5::CoreIndex,
				// 	pub hash: runtime_types::polkadot_core_primitives::CandidateHash,
				// 	pub descriptor: runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
				// 	pub availability_votes: ::subxt::utils::bits::DecodedBits<
				// 		::core::primitive::u8,
				// 		::subxt::utils::bits::Lsb0,
				// 	>,
				// 	pub backers: ::subxt::utils::bits::DecodedBits<
				// 		::core::primitive::u8,
				// 		::subxt::utils::bits::Lsb0,
				// 	>,
				// 	pub relay_parent_number: _1,
				// 	pub backed_in_number: _1,
				// 	pub backing_group: runtime_types::polkadot_primitives::v5::GroupIndex,
				// }
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum UmpQueueId {
					#[codec(index = 0)]
					Para(runtime_types::polkadot_parachain_primitives::primitives::Id),
				}
			}
			pub mod paras {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						# [codec (index = 0)] CurrentCodeUpdated (runtime_types :: polkadot_parachain_primitives :: primitives :: Id ,) , # [codec (index = 1)] CurrentHeadUpdated (runtime_types :: polkadot_parachain_primitives :: primitives :: Id ,) , # [codec (index = 2)] CodeUpgradeScheduled (runtime_types :: polkadot_parachain_primitives :: primitives :: Id ,) , # [codec (index = 3)] NewHeadNoted (runtime_types :: polkadot_parachain_primitives :: primitives :: Id ,) , # [codec (index = 4)] ActionQueued (runtime_types :: polkadot_parachain_primitives :: primitives :: Id , :: core :: primitive :: u32 ,) , # [codec (index = 5)] PvfCheckStarted (runtime_types :: polkadot_parachain_primitives :: primitives :: ValidationCodeHash , runtime_types :: polkadot_parachain_primitives :: primitives :: Id ,) , # [codec (index = 6)] PvfCheckAccepted (runtime_types :: polkadot_parachain_primitives :: primitives :: ValidationCodeHash , runtime_types :: polkadot_parachain_primitives :: primitives :: Id ,) , # [codec (index = 7)] PvfCheckRejected (runtime_types :: polkadot_parachain_primitives :: primitives :: ValidationCodeHash , runtime_types :: polkadot_parachain_primitives :: primitives :: Id ,) , }
				}
			}
			pub mod hrmp {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						OpenChannelRequested(
							runtime_types::polkadot_parachain_primitives::primitives::Id,
							runtime_types::polkadot_parachain_primitives::primitives::Id,
							::core::primitive::u32,
							::core::primitive::u32,
						),
						#[codec(index = 1)]
						OpenChannelCanceled(
							runtime_types::polkadot_parachain_primitives::primitives::Id,
							runtime_types::polkadot_parachain_primitives::primitives::HrmpChannelId,
						),
						#[codec(index = 2)]
						OpenChannelAccepted(
							runtime_types::polkadot_parachain_primitives::primitives::Id,
							runtime_types::polkadot_parachain_primitives::primitives::Id,
						),
						#[codec(index = 3)]
						ChannelClosed(
							runtime_types::polkadot_parachain_primitives::primitives::Id,
							runtime_types::polkadot_parachain_primitives::primitives::HrmpChannelId,
						),
						#[codec(index = 4)]
						HrmpChannelForceOpened(
							runtime_types::polkadot_parachain_primitives::primitives::Id,
							runtime_types::polkadot_parachain_primitives::primitives::Id,
							::core::primitive::u32,
							::core::primitive::u32,
						),
					}
				}
			}
			pub mod disputes {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: codec :: Decode,
						:: codec :: Encode,
						Clone,
						Debug,
						PartialEq,
						Eq,
						scale_info::TypeInfo,
					)]
					pub enum Event {
						#[codec(index = 0)]
						DisputeInitiated(
							runtime_types::polkadot_core_primitives::CandidateHash,
							runtime_types::polkadot_runtime_parachains::disputes::DisputeLocation,
						),
						#[codec(index = 1)]
						DisputeConcluded(
							runtime_types::polkadot_core_primitives::CandidateHash,
							runtime_types::polkadot_runtime_parachains::disputes::DisputeResult,
						),
						#[codec(index = 2)]
						Revert(::core::primitive::u32),
					}
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum DisputeLocation {
					#[codec(index = 0)]
					Local,
					#[codec(index = 1)]
					Remote,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum DisputeResult {
					#[codec(index = 0)]
					Valid,
					#[codec(index = 1)]
					Invalid,
				}
			}
		}
		pub mod polkadot_parachain_primitives {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct HeadData(pub ::sp_std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct HrmpChannelId {
					pub sender: runtime_types::polkadot_parachain_primitives::primitives::Id,
					pub recipient: runtime_types::polkadot_parachain_primitives::primitives::Id,
				}
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					// :: subxt :: ext :: codec :: CompactAs,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Id(pub ::core::primitive::u32);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct ValidationCode(pub ::sp_std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct ValidationCodeHash(pub runtime_types::primitive_types::H256);
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub enum Event {
					#[codec(index = 0)]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					KeyChanged {
						old_sudoer: ::core::option::Option<::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 2)]
					SudoAsDone {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					// :: subxt :: ext :: codec :: CompactAs,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					// :: subxt :: ext :: codec :: CompactAs,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Perbill(pub ::core::primitive::u32);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					// :: subxt :: ext :: codec :: CompactAs,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Percent(pub ::core::primitive::u8);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					// :: subxt :: ext :: codec :: CompactAs,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Permill(pub ::core::primitive::u32);
				#[derive(
					:: codec :: Decode,
					:: codec :: Encode,
					// :: subxt :: ext :: codec :: CompactAs,
					Clone,
					Debug,
					PartialEq,
					Eq,
					scale_info::TypeInfo,
				)]
				pub struct Perquintill(pub ::core::primitive::u64);
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
				#[codec(index = 13)]
				RootNotAllowed,
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub enum TokenError {
				#[codec(index = 0)]
				FundsUnavailable,
				#[codec(index = 1)]
				OnlyProvider,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
				#[codec(index = 7)]
				CannotCreateHold,
				#[codec(index = 8)]
				NotExpendable,
				#[codec(index = 9)]
				Blocked,
			}
			#[derive(
				:: codec :: Decode,
				:: codec :: Encode,
				Clone,
				Debug,
				PartialEq,
				Eq,
				scale_info::TypeInfo,
			)]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
	}
}
