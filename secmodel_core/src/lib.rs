use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
use indexmap::IndexMap;
use std::fmt;
use std::fs;
use thiserror::Error;

#[allow(dead_code)]
pub enum EntityID {
    Actor(String),
    Agent(String),
    Application(String),
    Authentication(String),
    Authorization(String),
    Channel(String),
    Classification(String),
    Control(String),
    Data(String),
    Encryption(String),
    Endpoint(String),
    Flow(String),
    Network(String),
    Process(String),
    Protocol(String),
    Regulation(String),
    Risk(String),
    Server(String),
    Store(String),
    Threat(String),
}

#[derive(Error, Debug)]
pub enum EntityError {
    #[error("Unrecognized entity type: {0}")]
    UnrecognizedEntityType(String),
}

impl fmt::Display for EntityID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityID::Actor(id) => write!(f, "actor.{id}"),
            EntityID::Agent(id) => write!(f, "agent.{id}"),
            EntityID::Application(id) => write!(f, "application.{id}"),
            EntityID::Authentication(id) => write!(f, "authentication.{id}"),
            EntityID::Authorization(id) => write!(f, "authorization.{id}"),
            EntityID::Channel(id) => write!(f, "channel.{id}"),
            EntityID::Classification(id) => write!(f, "classification.{id}"),
            EntityID::Control(id) => write!(f, "control.{id}"),
            EntityID::Data(id) => write!(f, "data.{id}"),
            EntityID::Encryption(id) => write!(f, "encryption.{id}"),
            EntityID::Endpoint(id) => write!(f, "endpoint.{id}"),
            EntityID::Flow(id) => write!(f, "flow.{id}"),
            EntityID::Network(id) => write!(f, "network.{id}"),
            EntityID::Process(id) => write!(f, "process.{id}"),
            EntityID::Protocol(id) => write!(f, "protocol.{id}"),
            EntityID::Regulation(id) => write!(f, "regulation.{id}"),
            EntityID::Risk(id) => write!(f, "risk.{id}"),
            EntityID::Server(id) => write!(f, "server.{id}"),
            EntityID::Store(id) => write!(f, "store.{id}"),
            EntityID::Threat(id) => write!(f, "threat.{id}"),
        }
    }
}

impl EntityID {
    pub fn from_str(id: &str) -> Result<EntityID, EntityError> {
        let mut parts = id.split(".");
        let etype = parts.next().unwrap();
        let name = parts.collect::<Vec<&str>>().join(".");
        match etype {
            "actor" => Ok(EntityID::Actor(name)),
            "agent" => Ok(EntityID::Agent(name)),
            "application" => Ok(EntityID::Application(name)),
            "authentication" => Ok(EntityID::Authentication(name)),
            "authorization" => Ok(EntityID::Authorization(name)),
            "channel" => Ok(EntityID::Channel(name)),
            "classification" => Ok(EntityID::Classification(name)),
            "control" => Ok(EntityID::Control(name)),
            "data" => Ok(EntityID::Data(name)),
            "encryption" => Ok(EntityID::Encryption(name)),
            "endpoint" => Ok(EntityID::Endpoint(name)),
            "flow" => Ok(EntityID::Flow(name)),
            "network" => Ok(EntityID::Network(name)),
            "process" => Ok(EntityID::Process(name)),
            "protocol" => Ok(EntityID::Protocol(name)),
            "regulation" => Ok(EntityID::Regulation(name)),
            "risk" => Ok(EntityID::Risk(name)),
            "server" => Ok(EntityID::Server(name)),
            "store" => Ok(EntityID::Store(name)),
            "threat" => Ok(EntityID::Threat(name)),
            _ => Err(EntityError::UnrecognizedEntityType(id.to_string())),
        }
    }
}

pub trait Entity {
    fn get_title(&self, id: &str) -> String {
        id.to_string()
    }
    fn get_description(&self) -> String {
        "Implement Entity::get_description".to_string()
    }
}

pub type Markdown = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FlowControl {
    description: Option<Markdown>,
    mitigates: Option<Vec<RiskID>>,
    remediates: Option<Vec<RiskID>>,
    flows: Option<Vec<FlowID>>,
    controller: Option<String>,
    sources: Option<Vec<String>>,
    destinations: Option<Vec<String>>,
    action: Option<FlowControlAction>,
}

#[derive(Serialize, Deserialize)]
enum FlowControlAction {
    #[serde(alias = "allow", alias = "ALLOW")]
    Allow,
    #[serde(alias = "deny", alias = "DENY")]
    Deny,
}

pub type DataID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Data {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub contains: Option<Vec<DataID>>,
    pub classification: Option<String>,
    pub regulations: Option<Vec<RegulationID>>,
    pub subjects: Option<Vec<String>>,
    pub owner: Option<ActorID>,
    pub steward: Option<String>,
    pub format: Option<String>,
    pub risks: Option<Vec<RiskID>>,
    pub controls: Option<Vec<ControlID>>,
}

impl Entity for Data {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

#[allow(dead_code)]
pub type StoreID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Store {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub data: Option<Vec<DataID>>,
    pub format: Option<String>,
    pub backing: Option<StoreID>,
    pub encryption: Option<EncryptionID>,
    pub authentication: Option<AuthenticationID>,
    pub authorization: Option<AuthorizationID>,
    pub controls: Option<Vec<ControlID>>,
    pub backup: Option<String>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Store {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type FlowID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flow {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub channel: Option<ChannelID>,
    pub sources: Option<Vec<String>>,
    pub destinations: Option<Vec<String>>,
    pub data: Option<Vec<DataID>>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Flow {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type ClassificationID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Classification {
    pub title: Option<String>,
    pub description: Option<Markdown>,
}

impl Entity for Classification {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type ControlID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Control {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub mitigates: Option<Vec<RiskID>>,
    pub remediates: Option<Vec<RiskID>>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Control {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkControls {
    description: Option<Markdown>,
    ingress_default: Option<FlowControlAction>,
    egress_default: Option<FlowControlAction>,
    ingress: Option<Vec<FlowControl>>,
    egress: Option<Vec<FlowControl>>,
}

pub type NetworkID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Network {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub interfaces: Option<Vec<NetworkInterface>>,
    pub protocols: Option<Vec<String>>,
    pub ipv4_ranges: Option<Vec<String>>,
    pub ipv6_ranges: Option<Vec<String>>,
    pub controls: Option<NetworkControls>,
    pub host_networks: Option<Vec<NetworkID>>,
    pub vpn: Option<bool>,
    pub risks: Option<Vec<RiskID>>,
}

impl Network {
    pub fn peers_upstream(&self) -> Vec<String> {
        let mut peers = Vec::new();
        if let Some(interfaces) = &self.interfaces {
            for i in interfaces.iter() {
                if let Some(network) = &i.network {
                    if !i.downstream.unwrap_or(false) {
                        peers.push(network.clone())
                    }
                }
            }
        }
        peers
    }
    pub fn peers_downstream(&self) -> Vec<String> {
        let mut peers = Vec::new();
        if let Some(interfaces) = &self.interfaces {
            for i in interfaces.iter() {
                if let Some(network) = &i.network {
                    if i.downstream.unwrap_or(false) {
                        peers.push(network.clone())
                    }
                }
            }
        }
        peers
    }
    pub fn peers(&self) -> Vec<String> {
        let mut peers = Vec::new();
        if let Some(interfaces) = &self.interfaces {
            for i in interfaces.iter() {
                if let Some(network) = &i.network {
                    peers.push(network.clone())
                }
            }
        }
        peers
    }
}

impl Entity for Network {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkInterface {
    pub description: Option<Markdown>,
    pub downstream: Option<bool>,
    pub network: Option<NetworkID>,
    pub address: Option<String>,
    pub hostnames: Option<Vec<String>>,
}

#[allow(dead_code)]
pub type ServerID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Server {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub interfaces: Option<Vec<NetworkInterface>>,
    pub applications: Option<Vec<ApplicationID>>,
    pub stores: Option<Vec<StoreID>>,
    pub owner: Option<ActorID>,
    pub os: Option<String>,
    pub version: Option<String>,
    pub risks: Option<Vec<RiskID>>,
}
impl Entity for Server {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

#[allow(dead_code)]
pub type DeploymentID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Deployment {
    pub description: Option<Markdown>,
    pub targets: Option<Vec<ServerID>>,
    pub applications: Option<Vec<ApplicationID>>,
    pub stores: Option<Vec<StoreID>>,
}

pub type Port = usize;

pub type ChannelID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Channel {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub over: Option<Vec<NetworkID>>,
    pub protocols: Option<Vec<String>>,
    pub ports: Option<Vec<Port>>,
    pub encryption: Option<String>,
    pub authentication: Option<String>,
    pub authorization: Option<String>,
    pub controls: Option<Vec<String>>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Channel {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type ApplicationID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Application {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub controls: Option<Vec<ControlID>>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Application {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

#[allow(dead_code)]
pub type ProcessID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Process {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Process {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

#[allow(dead_code)]
pub type ProtocolID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Protocol {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Protocol {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

#[allow(dead_code)]
pub type EndpointID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Endpoint {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub interfaces: Option<Vec<NetworkInterface>>,
    pub applications: Option<Vec<ApplicationID>>,
    pub agents: Option<Vec<ApplicationID>>,
    pub stores: Option<Vec<StoreID>>,
    pub owner: Option<ActorID>,
    pub os: Option<String>,
    pub version: Option<String>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Endpoint {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type RiskID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Risk {
    pub title: Option<String>,
    pub description: Option<Markdown>,
}

impl Entity for Risk {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

#[allow(dead_code)]
pub type ThreatID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Threat {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub sophistication: Option<String>,
    pub motivation: Option<Markdown>,
    pub risk: Option<Vec<RiskID>>,
    pub actor: Option<Vec<ActorID>>,
}

impl Entity for Threat {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type ActorID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Actor {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Actor {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type AgentID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Agent {
    pub title: Option<String>,
    pub description: Option<Markdown>,
    pub actor: Option<ActorID>,
    pub endpoint: Option<EndpointID>,
    pub server: Option<ServerID>,
    pub process: Option<ProcessID>,
    pub risks: Option<Vec<RiskID>>,
}

impl Entity for Agent {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type RegulationID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Regulation {
    pub title: Option<String>,
    pub description: Option<Markdown>,
}

impl Entity for Regulation {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type AuthenticationID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authentication {
    pub title: Option<String>,
    pub description: Option<Markdown>,
}

impl Entity for Authentication {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type AuthorizationID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authorization {
    pub title: Option<String>,
    pub description: Option<Markdown>,
}

impl Entity for Authorization {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

pub type EncryptionID = String;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Encryption {
    pub title: Option<String>,
    pub description: Option<Markdown>,
}

impl Entity for Encryption {
    fn get_title(&self, id: &str) -> String {
        self.title.clone().clone().unwrap_or(id.to_string())
    }
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or("".to_string())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Comment {
    pub title: Option<String>,
    pub entity: String,
    pub comment: Markdown,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub title: Option<String>,
    pub actor: Option<IndexMap<String, Actor>>,
    pub agent: Option<IndexMap<String, Agent>>,
    pub application: Option<IndexMap<String, Application>>,
    pub channel: Option<IndexMap<String, Channel>>,
    pub data: Option<IndexMap<String, Data>>,
    pub flow: Option<IndexMap<String, Flow>>,
    pub store: Option<IndexMap<String, Store>>,

    pub network: Option<IndexMap<String, Network>>,
    pub server: Option<IndexMap<String, Server>>,
    pub endpoint: Option<IndexMap<String, Endpoint>>,
    pub process: Option<IndexMap<String, Process>>,
    pub protocol: Option<IndexMap<String, Protocol>>,

    pub deployment: Option<IndexMap<String, Deployment>>,

    pub control: Option<IndexMap<String, Control>>,
    pub classification: Option<IndexMap<String, Classification>>,
    pub risk: Option<IndexMap<String, Risk>>,
    pub threat: Option<IndexMap<String, Threat>>,
    pub regulation: Option<IndexMap<String, Regulation>>,

    pub authentication: Option<IndexMap<String, Authentication>>,
    pub authorization: Option<IndexMap<String, Authorization>>,
    pub encryption: Option<IndexMap<String, Encryption>>,

    pub comment: Option<Vec<Comment>>,
}

impl Model {
    pub fn application_by_id<'a>(&'a self, id: &str) -> Option<&'a Application> {
        if let Some(m) = &self.application {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }
    pub fn network_by_id<'a>(&'a self, id: &str) -> Option<&'a Network> {
        if let Some(m) = &self.network {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }
    pub fn endpoint_by_id<'a>(&'a self, id: &str) -> Option<&'a Endpoint> {
        if let Some(m) = &self.endpoint {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn server_by_id<'a>(&'a self, id: &str) -> Option<&'a Server> {
        if let Some(m) = &self.server {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn actor_by_id<'a>(&'a self, id: &str) -> Option<&'a Actor> {
        if let Some(m) = &self.actor {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn flow_by_id<'a>(&'a self, id: &str) -> Option<&'a Flow> {
        if let Some(m) = &self.flow {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn agent_by_id<'a>(&'a self, id: &str) -> Option<&'a Agent> {
        if let Some(m) = &self.agent {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn store_by_id<'a>(&'a self, id: &str) -> Option<&'a Store> {
        if let Some(m) = &self.store {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn risk_by_id<'a>(&'a self, id: &str) -> Option<&'a Risk> {
        if let Some(m) = &self.risk {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn data_by_id<'a>(&'a self, id: &str) -> Option<&'a Data> {
        if let Some(m) = &self.data {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn channel_by_id<'a>(&'a self, id: &str) -> Option<&'a Channel> {
        if let Some(m) = &self.channel {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn authentication_by_id<'a>(&'a self, id: &str) -> Option<&'a Authentication> {
        if let Some(m) = &self.authentication {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn authorization_by_id<'a>(&'a self, id: &str) -> Option<&'a Authorization> {
        if let Some(m) = &self.authorization {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn encryption_by_id<'a>(&'a self, id: &str) -> Option<&'a Encryption> {
        if let Some(m) = &self.encryption {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn process_by_id<'a>(&'a self, id: &str) -> Option<&'a Process> {
        if let Some(m) = &self.process {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn protocol_by_id<'a>(&'a self, id: &str) -> Option<&'a Protocol> {
        if let Some(m) = &self.protocol {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn regulation_by_id<'a>(&'a self, id: &str) -> Option<&'a Regulation> {
        if let Some(m) = &self.regulation {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn control_by_id<'a>(&'a self, id: &str) -> Option<&'a Control> {
        if let Some(m) = &self.control {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn classification_by_id<'a>(&'a self, id: &str) -> Option<&'a Classification> {
        if let Some(m) = &self.classification {
            let parts: Vec<&str> = id.split(".").collect();
            m.get(parts[1])
        } else {
            None
        }
    }

    pub fn entity_by_id<'a>(&'a self, id: &str) -> Option<Box<&dyn Entity>> {
        let eid = EntityID::from_str(id).unwrap();
        match eid {
            EntityID::Application(..) => self
                .application_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Actor(..) => self
                .actor_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Agent(..) => self
                .agent_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Server(..) => self
                .server_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Network(..) => self
                .network_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Endpoint(..) => self
                .endpoint_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Process(..) => self
                .process_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Protocol(..) => self
                .protocol_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Data(..) => self
                .data_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Store(..) => self
                .store_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Flow(..) => self
                .flow_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Channel(..) => self
                .channel_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Risk(..) => self
                .risk_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Control(..) => self
                .control_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Classification(..) => self
                .classification_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Regulation(..) => self
                .regulation_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Authorization(..) => self
                .authorization_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Authentication(..) => self
                .authentication_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Encryption(..) => self
                .encryption_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
            EntityID::Threat(..) => self
                .flow_by_id(id)
                .and_then(|a| Some(Box::new(a as &dyn Entity))),
        }
    }

    pub fn network_servers<'a>(&self, network_id: &str) -> Vec<String> {
        let mut my_servers = Vec::new();
        if let Some(servers) = &self.server {
            for (serverid, server) in servers {
                if let Some(interfaces) = &server.interfaces {
                    if interfaces.len() == 1 {
                        for i in interfaces {
                            if let Some(network) = &i.network {
                                if network == network_id {
                                    let sid = format!("server.{serverid}");
                                    my_servers.push(sid)
                                }
                            }
                        }
                    }
                }
            }
        }
        my_servers
    }
    pub fn network_connected_servers<'a>(&self, network_id: &str) -> Vec<String> {
        let mut my_servers = Vec::new();
        if let Some(servers) = &self.server {
            for (serverid, server) in servers {
                if let Some(interfaces) = &server.interfaces {
                    for i in interfaces {
                        if let Some(network) = &i.network {
                            if network == network_id {
                                let sid = format!("server.{serverid}");
                                my_servers.push(sid)
                            }
                        }
                    }
                }
            }
        }
        my_servers
    }
    // those endpoints connected to *only* this network
    pub fn network_endpoints<'a>(&self, network_id: &str) -> Vec<String> {
        let mut my_endpoints = Vec::new();
        if let Some(endpoints) = &self.endpoint {
            for (endpointid, endpoint) in endpoints {
                if let Some(interfaces) = &endpoint.interfaces {
                    if interfaces.len() == 1 {
                        for i in interfaces {
                            if let Some(network) = &i.network {
                                if network == network_id {
                                    let sid = format!("endpoint.{endpointid}");
                                    my_endpoints.push(sid)
                                }
                            }
                        }
                    }
                }
            }
        }
        my_endpoints
    }
    // those endpoints connected to this network
    pub fn network_connected_endpoints<'a>(&self, network_id: &str) -> Vec<String> {
        let mut my_endpoints = Vec::new();
        if let Some(endpoints) = &self.endpoint {
            for (endpointid, endpoint) in endpoints {
                if let Some(interfaces) = &endpoint.interfaces {
                    for i in interfaces {
                        if let Some(network) = &i.network {
                            if network == network_id {
                                let sid = format!("endpoint.{endpointid}");
                                my_endpoints.push(sid)
                            }
                        }
                    }
                }
            }
        }
        my_endpoints
    }

    pub fn data_stores(&self, data_id: &str) -> Vec<String> {
        let target = data_id.to_string();
        if let Some(stores) = &self.store {
            stores
                .iter()
                .filter_map(|(sid, store)| {
                    if store.data.clone().unwrap_or(Vec::new()).contains(&target) {
                        Some(format!("store.{sid}"))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn entity_comments<'a>(&self, entity_id: &str) -> Vec<&Comment> {
        if let Some(comments) = &self.comment {
            comments.iter().filter(|c| c.entity == entity_id).collect()
        } else {
            Vec::new()
        }
    }

    pub fn entity_flows(&self, entity_id: &str) -> Vec<String> {
        let target = entity_id.to_string();
        if let Some(flows) = &self.flow {
            flows
                .iter()
                .filter_map(|(fid, flow)| {
                    if flow.sources.clone().unwrap_or(Vec::new()).contains(&target)
                        || flow
                            .destinations
                            .clone()
                            .unwrap_or(Vec::new())
                            .contains(&target)
                        || flow.data.clone().unwrap_or(Vec::new()).contains(&target)
                        || flow.channel.clone().unwrap_or(String::new()) == target
                    {
                        Some(format!("flow.{fid}"))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Error reading model: {0}")]
    ReadError(String),
    #[error("Error parsing model: {0}")]
    ParseError(String),
    #[error("Error Compiling model: {0}")]
    CompileError(String),
}

pub fn load(path: &str) -> Result<Model, ModelError> {
    let modelsrc = match fs::read_to_string(path) {
        Ok(m) => Ok(m),
        Err(e) => Err(ModelError::ReadError(e.to_string())),
    }?;

    match toml::from_str(&modelsrc) {
        Ok(m) => Ok(m),
        Err(e) => Err(ModelError::ParseError(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load("../test/test_model.toml").unwrap();
    }
}
