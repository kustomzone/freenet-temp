use locutus_stdlib::prelude::*;
use state::State as PState;
use parameters::Parameters as PParameters;

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
    fn validate_state(_parameters: Parameters<'static>, state: State<'static>) -> bool {
        let state : PState = rmps::from_slice(state.into_owned().as_slice()).unwrap();
        let parameters : PParameters = rmps::from_slice(_parameters.as_ref()).unwrap();
        state.verify_with_public_key(&parameters.public_key)
    }

    fn validate_delta(_parameters: Parameters<'static>, _delta: StateDelta<'static>) -> bool {
        unimplemented!()
    }

    fn update_state(
        _parameters: Parameters<'static>,
        mut state: State<'static>,
        _delta: StateDelta<'static>,
    ) -> Result<UpdateModification, ContractError> {
        let new_state = state.to_mut();
        new_state.extend([1, 2, 3]);
        Ok(UpdateModification::ValidUpdate(state))
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
