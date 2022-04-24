use locutus_stdlib::prelude::*;
use state::State as PState;
use parameters::Parameters as PParameters;
use delta::Delta as PDelta;

mod state;
mod parameters;
mod delta;
mod state_summary;

extern crate rmp;
extern crate rsa;
extern crate serde;
extern crate rmp_serde as rmps;

struct Contract;

#[contract]
impl ContractInterface for Contract {
    fn validate_state(parameters: Parameters<'static>, state: State<'static>) -> bool {
        let state : PState = state.into_owned().into();
        let parameters : PParameters = parameters.as_ref().into();
        state.verify_with_public_key(&parameters.public_key)
    }

    fn validate_delta(parameters: Parameters<'static>, _delta: StateDelta<'static>) -> bool {
        let delta : PDelta = _delta.into_owned().as_slice().into();
        if delta.entries.is_empty() {
            // There is no point in an empty delta
            return false;
        }
        let parameters : PParameters = parameters.as_ref().into();
        for entry in delta.entries.iter() {
            if !entry.verify_with_public_key(&parameters.public_key) {
                return false;
            }
        }
        return true;
    }

    fn update_state(
        _parameters: Parameters<'static>,
        mut state: State<'static>,
        _delta: StateDelta<'static>,
    ) -> Result<UpdateModification, ContractError> {
        // TOOD: Not sure if this is the right way to mutate state
        let delta : PDelta = _delta.into_owned().as_slice().into();
        let mut new_state : PState = state.to_mut().as_slice().into();
        new_state.entries.extend(delta.entries);
        let new_state : Vec<u8> = new_state.into();
        let new_state = State::from(new_state);
        return Result::Ok(UpdateModification::ValidUpdate(new_state));
    }

    fn summarize_state(
        _parameters: Parameters<'static>,
        state: State<'static>,
    ) -> StateSummary<'static> {
        let state = state.as_ref();
        // eprintln!("state: {state:?}");
        // eprintln!("summary: {:?}", &state[0..1]);
        StateSummary::from(state[0..1].to_vec())
    }

    fn get_state_delta(
        _parameters: Parameters<'static>,
        _state: State<'static>,
        _summary: StateSummary<'static>,
    ) -> StateDelta<'static> {
        unimplemented!()
    }

    fn update_state_from_summary(
        _parameters: Parameters<'static>,
        _state: State<'static>,
        _summary: StateSummary<'static>,
    ) -> Result<UpdateModification, ContractError> {
        unimplemented!()
    }
}
