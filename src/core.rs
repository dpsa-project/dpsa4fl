
use janus_client::{ClientParameters, aggregator_hpke_config, default_http_client, Client};
use janus_core::{time::RealClock};
use janus_messages::{HpkeConfig, Role, TaskId, Duration};
use url::*;
// use anyhow::Result;
use async_std::future::try_join;
use prio::vdaf::prio3::Prio3Aes128FixedPointBoundedL2VecSum;

use fixed::types::extra::{U15, U31, U63};
use fixed::{FixedI16, FixedI32, FixedI64};


////////////////////////////////////////////////////
// Parametrization

#[derive(Clone)]
pub struct Locations
{
    pub internal_leader: Url, // TODO: This internal URL should probably be configured somewhere else, actually
    pub internal_helper: Url, // TODO: Same.
    pub external_leader_tasks: Url,
    pub external_helper_tasks: Url,
    pub external_leader_main: Url,
    pub external_helper_main: Url,
    // controller: Url, // the server that controls the learning process
}

impl Locations
{
    pub fn get_external_aggregator_endpoints(&self) -> Vec<Url>
    {
        vec![self.external_leader_main.clone(),self.external_helper_main.clone()]
    }
}


////////////////////////////////////////////////////
// State

#[derive(Clone)]
pub struct CommonState_Parametrization
{
    pub location: Locations,
    pub gradient_len: usize,
}
