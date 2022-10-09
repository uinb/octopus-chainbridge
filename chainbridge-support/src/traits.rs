use sp_std::prelude::*;
use frame_support::dispatch::DispatchError;
use frame_support::dispatch::DispatchResult;

pub trait Agent<AccountId> {
	type Origin;
	type Message;

	/// bind the origin to an appchain account without private key
	/// function RegisterInterchainAccount(counterpartyPortId: Identifier, connectionID: Identifier) returns (nil)
	fn register_agent(origin: Self::Origin) -> Result<AccountId, DispatchError>;

	/// function AuthenticateTx(msgs []Any, connectionId string, portId string) returns (error)
	fn authenticate_tx(origin: Self::Origin, msg: Self::Message) -> Result<(), DispatchError>;

	/// function ExecuteTx(sourcePort: Identifier, channel Channel, msgs []Any) returns (resultString, error)
	fn execute_tx(origin: Self::Origin, msg: Self::Message) -> DispatchResult;
}