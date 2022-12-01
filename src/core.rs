
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
    pub leader: Url,
    pub helper: Url,
    // controller: Url, // the server that controls the learning process
}

impl Locations
{
    pub fn get_aggregator_endpoints(&self) -> Vec<Url>
    {
        vec![self.leader.clone(),self.helper.clone()]
    }
}


////////////////////////////////////////////////////
// State

pub struct CommonState_Parametrization
{
    pub location: Locations,
    pub gradient_len: usize,
}