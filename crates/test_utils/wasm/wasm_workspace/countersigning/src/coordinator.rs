use crate::integrity::*;
use hdk::prelude::*;

#[hdk_extern]
fn create_a_thing(_: ()) -> ExternResult<HeaderHash> {
    create_entry(&EntryTypes::Thing(Thing::Valid))
}

#[hdk_extern]
fn create_an_invalid_thing(_: ()) -> ExternResult<HeaderHash> {
    create_entry(&EntryTypes::Thing(Thing::Invalid))
}

fn create_countersigned(
    responses: Vec<PreflightResponse>,
    thing: Thing,
) -> ExternResult<HeaderHash> {
    let thing = EntryTypes::Thing(thing);
    let entry_def_index = EntryDefIndex::try_from(&thing)?;
    let visibility = EntryVisibility::from(&thing);

    let thing = match thing {
        EntryTypes::Thing(t) => t,
    };

    let entry = Entry::CounterSign(
        Box::new(
            CounterSigningSessionData::try_from_responses(responses).map_err(
                |countersigning_error| wasm_error!(WasmErrorInner::Guest(countersigning_error.to_string())),
            )?,
        ),
        thing.try_into()?,
    );
    HDK.with(|h| {
        h.borrow().create(CreateInput::new(
            entry_def_index,
            visibility,
            entry,
            // Countersigned entries MUST have strict ordering.
            ChainTopOrdering::Strict,
        ))
    })
}

#[hdk_extern]
fn create_an_invalid_countersigned_thing(
    responses: Vec<PreflightResponse>,
) -> ExternResult<HeaderHash> {
    create_countersigned(responses, Thing::Invalid)
}

#[hdk_extern]
fn create_a_countersigned_thing(responses: Vec<PreflightResponse>) -> ExternResult<HeaderHash> {
    create_countersigned(responses, Thing::Valid)
}

fn generate_preflight_request(
    agents: Vec<(AgentPubKey, Vec<Role>)>,
    thing: Thing,
) -> ExternResult<PreflightRequest> {
    let hash = hash_entry(&thing)?;
    let thing = EntryTypes::Thing(thing);
    let entry_type = thing.try_into()?;
    PreflightRequest::try_new(
        hash,
        agents,
        None,
        session_times_from_millis(5000)?,
        HeaderBase::Create(CreateBase::new(entry_type)),
        PreflightBytes(vec![]),
    )
    .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))
}

#[hdk_extern]
fn generate_countersigning_preflight_request(
    agents: Vec<(AgentPubKey, Vec<Role>)>,
) -> ExternResult<PreflightRequest> {
    generate_preflight_request(agents, Thing::Valid)
}

#[hdk_extern]
fn generate_invalid_countersigning_preflight_request(
    agents: Vec<(AgentPubKey, Vec<Role>)>,
) -> ExternResult<PreflightRequest> {
    generate_preflight_request(agents, Thing::Invalid)
}

#[hdk_extern]
fn accept_countersigning_preflight_request(
    preflight_request: PreflightRequest,
) -> ExternResult<PreflightRequestAcceptance> {
    hdk::prelude::accept_countersigning_preflight_request(preflight_request)
}

#[hdk_extern]
fn must_get_header(header_hash: HeaderHash) -> ExternResult<SignedHeaderHashed> {
    hdk::prelude::must_get_header(header_hash)
}

#[hdk_extern]
fn must_get_entry(entry_hash: EntryHash) -> ExternResult<EntryHashed> {
    hdk::prelude::must_get_entry(entry_hash)
}

#[hdk_extern]
fn must_get_valid_element(header_hash: HeaderHash) -> ExternResult<Element> {
    hdk::prelude::must_get_valid_element(header_hash)
}

#[hdk_extern]
fn get_agent_activity(input: GetAgentActivityInput) -> ExternResult<AgentActivity> {
    HDK.with(|h| h.borrow().get_agent_activity(input))
}