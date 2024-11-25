use indexmap::IndexMap;
use secmodel_core::Entity;
use secmodel_core::*;

pub trait Report {
    // iterate over all vertices in a
    fn report(&self, model: &Model) -> String;
}

pub trait DetailSection {
    // iterate over all vertices in a
    fn detail_section(&self, id: &str, model: &Model) -> String;
}

pub fn link(text: &str, link: &str) -> String {
    format!("[{text}]({link})")
}

pub fn local_link(text: &str, link: &str) -> String {
    format!("[{text}](#{link})")
}

pub fn section_header(level: usize, title: &str, anchor: &str) -> String {
    format!("{} {title} {{#{anchor}}}\n\n", "#".repeat(level))
}

pub fn entity_link(id: &str, model: &Model) -> String {
    if let Some(e) = model.entity_by_id(id) {
        local_link(&e.get_title(id), id)
    } else {
        format!("{id}")
    }
}

pub fn entity_links(ids: &[String], label: &str, model: &Model) -> String {
    let mut text = String::new();
    for id in ids {
        text.push_str(&format!("* {}\n", entity_link(id, model)));
    }
    if text.len() > 0 {
        format!("{label}:\n\n{text}\n")
    } else {
        text
    }
}

pub fn interfaces_list(interfaces: &[NetworkInterface], model: &Model) -> String {
    let mut text = String::new();
    for i in interfaces {
        let mut itext = String::new();
        if let Some(networkid) = &i.network {
            itext.push_str(&format!(" Network: {}", entity_link(networkid, model)));
        }
        if let Some(address) = &i.address {
            itext.push_str(&format!(" Address: {address}"));
        }
        if itext.len() > 1 {
            text.push_str(&format!("* {itext}\n"))
        }
    }
    if text.len() > 1 {
        format!("\nNetwork Interfaces:\n\n{text}\n\n")
    } else {
        text
    }
}

impl DetailSection for Network {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));

        // peers let peers = self.peers();
        text.push_str(&entity_links(&self.peers(), "Peer Networks", model));

        // servers
        let servers = model.network_connected_servers(id);
        text.push_str(&entity_links(&servers, "Connected Servers", model));

        // endponts
        let endpoints = model.network_connected_endpoints(id);
        text.push_str(&entity_links(&endpoints, "Connected Endpoints", model));

        //flows
        text.push_str(&entity_links(
            &model.entity_flows(id),
            "Connected Flows",
            model,
        ));

        // risks
        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        //let comments = model.entity_comments(id);
        text
    }
}

impl DetailSection for Server {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));

        // owner, os, uname
        if let Some(owner) = &self.owner {
            text.push_str(&format!("* Owner: {owner}\n"));
        }
        if let Some(os) = &self.os {
            text.push_str(&format!("* OS: {os}\n"));
        }
        if let Some(version) = &self.version {
            text.push_str(&format!("* Version: {version}\n"));
        }
        text.push_str("\n");
        // Interfaces
        if let Some(interfaces) = &self.interfaces {
            text.push_str(&interfaces_list(interfaces, model));
        }

        // applications
        if let Some(apps) = &self.applications {
            text.push_str(&entity_links(apps, "Hosted Applications", model));
        }

        // stores
        if let Some(stores) = &self.stores {
            text.push_str(&entity_links(stores, "Hosted Stores", model));
        }
        //flows
        text.push_str(&entity_links(
            &model.entity_flows(id),
            "Connected Flows",
            model,
        ));

        // risks
        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        text
    }
}

impl DetailSection for Endpoint {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));

        // owner, os, uname
        if let Some(owner) = &self.owner {
            text.push_str(&format!("* Owner: {owner}\n"));
        }
        if let Some(os) = &self.os {
            text.push_str(&format!("* OS: {os}\n"));
        }
        if let Some(version) = &self.version {
            text.push_str(&format!("* Version: {version}\n"));
        }
        text.push_str("\n");
        // Interfaces
        if let Some(interfaces) = &self.interfaces {
            text.push_str(&interfaces_list(interfaces, model));
        }

        // agents
        if let Some(agents) = &self.agents {
            text.push_str(&entity_links(agents, "Hosted Agents", model));
        }
        //flows
        text.push_str(&entity_links(
            &model.entity_flows(id),
            "Connected Flows",
            model,
        ));

        // risks
        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        text
    }
}

impl DetailSection for Application {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));

        //flows
        text.push_str(&entity_links(
            &model.entity_flows(id),
            "Connected Flows",
            model,
        ));

        // risks
        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        //controls
        if let Some(controls) = &self.controls {
            text.push_str(&entity_links(controls, "Controls", model));
        }

        text
    }
}

impl DetailSection for Actor {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));

        //flows
        text.push_str(&entity_links(
            &model.entity_flows(id),
            "Connected Flows",
            model,
        ));

        // risks
        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        text
    }
}

impl DetailSection for Agent {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        // actor, endpoint
        if let Some(eid) = &self.actor {
            text.push_str(&format!("* Actor: {}\n", entity_link(eid, model)));
        }

        if let Some(eid) = &self.endpoint {
            text.push_str(&format!("* Endpoint: {}\n", entity_link(eid, model)));
        }

        if let Some(eid) = &self.server {
            text.push_str(&format!("* Server: {}\n", entity_link(eid, model)));
        }

        if let Some(eid) = &self.process {
            text.push_str(&format!("* Process: {}\n", entity_link(eid, model)));
        }

        text.push_str("\n");

        //flows
        text.push_str(&entity_links(
            &model.entity_flows(id),
            "Connected Flows",
            model,
        ));

        // risks
        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        text
    }
}

impl DetailSection for Flow {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        if let Some(channel) = &self.channel {
            text.push_str(&format!("* Channel: {}\n", &entity_link(channel, model)));
            text.push_str(&format!("\n"));
        }

        if let Some(sources) = &self.sources {
            text.push_str(&entity_links(sources, "Sources", model));
        }

        if let Some(destinations) = &self.destinations {
            text.push_str(&entity_links(destinations, "Destinations", model));
        }

        if let Some(data) = &self.data {
            text.push_str(&entity_links(data, "Data", model));
        }

        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        text
    }
}

impl DetailSection for Channel {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));

        if let Some(authentication) = &self.authentication {
            text.push_str(&format!(
                "* Authentication: {}\n",
                &entity_link(authentication, model)
            ));
        }
        if let Some(authorization) = &self.authorization {
            text.push_str(&format!(
                "* Authorization: {}\n",
                &entity_link(authorization, model)
            ));
        }
        if let Some(encryption) = &self.encryption {
            text.push_str(&format!(
                "* Encryption: {}\n",
                &entity_link(encryption, model)
            ));
        }

        if let Some(ports) = &self.ports {
            text.push_str(&format!(
                "* Ports: {}\n",
                ports
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        text.push_str(&format!("\n"));

        if let Some(protocols) = &self.protocols {
            text.push_str(&entity_links(protocols, "Protocols", model));
        }

        if let Some(over) = &self.over {
            text.push_str(&entity_links(over, "Runs on Channels:", model));
        }

        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        if let Some(controls) = &self.controls {
            text.push_str(&entity_links(controls, "Controls", model));
        }

        //flows
        text.push_str(&entity_links(
            &model.entity_flows(id),
            "Hosted Flows",
            model,
        ));

        text
    }
}

impl DetailSection for Data {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        // owner/steward/format/classification
        if let Some(class) = &self.classification {
            text.push_str(&format!(
                "* Classification: {}\n",
                &entity_link(class, model)
            ));
        }
        if let Some(format) = &self.format {
            text.push_str(&format!("* Format: {}\n", &entity_link(format, model)));
        }
        if let Some(owner) = &self.owner {
            text.push_str(&format!("* Owner: {}\n", &entity_link(owner, model)));
        }
        if let Some(steward) = &self.steward {
            text.push_str(&format!("* Steward: {}\n", &entity_link(steward, model)));
        }

        text.push_str("\n");

        if let Some(data) = &self.contains {
            text.push_str(&entity_links(data, "Contains", model));
        }

        if let Some(subjects) = &self.subjects {
            text.push_str(&entity_links(subjects, "Subjects", model));
        }

        if let Some(regulations) = &self.regulations {
            text.push_str(&entity_links(regulations, "Regulations", model));
        }

        if let Some(controls) = &self.controls {
            text.push_str(&entity_links(controls, "Controls", model));
        }

        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        //flows
        text.push_str(&entity_links(&model.entity_flows(id), "Data Flows", model));

        //stores
        text.push_str(&entity_links(&model.data_stores(id), "Data Stores", model));

        text
    }
}

impl DetailSection for Store {
    fn detail_section(&self, id: &str, model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));

        if let Some(backing) = &self.backing {
            text.push_str(&format!(
                "* Backing Store: {}\n",
                &entity_link(backing, model)
            ));
        }
        if let Some(format) = &self.format {
            text.push_str(&format!("* Format: {}\n", &entity_link(format, model)));
        }

        if let Some(backup) = &self.backup {
            text.push_str(&format!(
                "* Backup Scheme: {}\n",
                &entity_link(backup, model)
            ));
        }

        if let Some(authentication) = &self.authentication {
            text.push_str(&format!(
                "* Authentication: {}\n",
                &entity_link(authentication, model)
            ));
        }
        if let Some(authorization) = &self.authorization {
            text.push_str(&format!(
                "* Authorization: {}\n",
                &entity_link(authorization, model)
            ));
        }
        if let Some(encryption) = &self.encryption {
            text.push_str(&format!(
                "* Encryption: {}\n",
                &entity_link(encryption, model)
            ));
        }

        text.push_str("\n");

        if let Some(data) = &self.data {
            text.push_str(&entity_links(data, "Data", model));
        }

        if let Some(controls) = &self.controls {
            text.push_str(&entity_links(controls, "Controls", model));
        }

        //flows
        text.push_str(&entity_links(
            &model.entity_flows(id),
            "Connected Flows",
            model,
        ));

        // risks
        if let Some(risks) = &self.risks {
            text.push_str(&entity_links(risks, "Risks", model));
        }

        text
    }
}

impl DetailSection for Authentication {
    fn detail_section(&self, id: &str, _model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        text
    }
}

impl DetailSection for Authorization {
    fn detail_section(&self, id: &str, _model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        text
    }
}

impl DetailSection for Encryption {
    fn detail_section(&self, id: &str, _model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        text
    }
}

impl DetailSection for Control {
    fn detail_section(&self, id: &str, _model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        text
    }
}

impl DetailSection for Risk {
    fn detail_section(&self, id: &str, _model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        text
    }
}

impl DetailSection for Threat {
    fn detail_section(&self, id: &str, _model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        text
    }
}

impl DetailSection for Classification {
    fn detail_section(&self, id: &str, _model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        text
    }
}

impl DetailSection for Protocol {
    fn detail_section(&self, id: &str, _model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        text
    }
}

impl DetailSection for Regulation {
    fn detail_section(&self, id: &str, _model: &Model) -> String {
        let mut text = String::new();
        text.push_str(&section_header(3, &self.get_title(id), id));
        text.push_str(&format!("{}\n\n", self.get_description()));
        text
    }
}

fn entity_section(
    entity: &str,
    title: &str,
    entities: &IndexMap<String, impl DetailSection>,
    model: &Model,
) -> String {
    let mut text = String::new();
    text.push_str(&format!("## {title}\n\n"));
    for (id, e) in entities.iter() {
        let id = &format!("{entity}.{id}");
        text.push_str(&e.detail_section(&id, model));
        text.push_str("\n----\n\n");
    }
    text
}

impl Report for Model {
    fn report(&self, model: &Model) -> String {
        let mut text = String::new();
        let header = r#"
---
title: Security Architecture Model
author: Taconic System
header-includes: |
    \usepackage{fancyhdr}
    \pagestyle{fancy}
    \fancyhead[CO,CE]{Security Architecture Model}
    \fancyfoot[CO,CE]{Taconic Systems}
    \fancyfoot[LE,RO]{\thepage}
---

"#;
        text.push_str(header);
        text.push_str("# Overview\n\n");

        text.push_str(&format!("```mermaid\n"));
        text.push_str(&secmodel_mermaid::Render::render(self, "", model).unwrap());
        text.push_str(&format!("```\n\n"));

        text.push_str("# Entities\n\n");

        // networks
        if let Some(networks) = &self.network {
            text.push_str(&entity_section("network", "Networks", networks, model));
        };

        // servers
        if let Some(servers) = &self.server {
            text.push_str(&entity_section("server", "Servers", servers, model));
        };

        // endpoints
        if let Some(endpoints) = &self.endpoint {
            text.push_str(&entity_section("endpoint", "Endpoints", endpoints, model));
        };

        // applications
        if let Some(applications) = &self.application {
            text.push_str(&entity_section(
                "application",
                "Applications",
                applications,
                model,
            ));
        };

        // actors
        if let Some(actors) = &self.actor {
            text.push_str(&entity_section("actor", "Actors", actors, model));
        };

        // agents
        if let Some(agents) = &self.agent {
            text.push_str(&entity_section("agent", "Agents", agents, model));
        };

        // flows
        if let Some(flows) = &self.flow {
            text.push_str(&entity_section("flow", "Flows", flows, model));
        };

        // Channels
        if let Some(channels) = &self.channel {
            text.push_str(&entity_section("channel", "Channels", channels, model));
        };

        // Authentication Schemes
        if let Some(authentications) = &self.authentication {
            text.push_str(&entity_section(
                "authentication",
                "Authentication Schemes",
                authentications,
                model,
            ));
        };

        // Authorizations Schemes
        if let Some(authorizations) = &self.authorization {
            text.push_str(&entity_section(
                "authorization",
                "Authorization Schemes",
                authorizations,
                model,
            ));
        };

        // Encryption Schemes
        if let Some(encryptions) = &self.encryption {
            text.push_str(&entity_section(
                "encryption",
                "Encryption Schemes",
                encryptions,
                model,
            ));
        };

        // Data
        if let Some(datas) = &self.data {
            text.push_str(&entity_section("data", "Data Types", datas, model));
        };

        // Store
        if let Some(stores) = &self.store {
            text.push_str(&entity_section("store", "Data Stores", stores, model));
        };

        // Protocols
        if let Some(protocols) = &self.protocol {
            text.push_str(&entity_section("protocol", "Protocols", protocols, model));
        };

        // Classification
        if let Some(classifications) = &self.classification {
            text.push_str(&entity_section(
                "classification",
                "Data Classifications",
                classifications,
                model,
            ));
        };

        // Controls
        if let Some(controls) = &self.control {
            text.push_str(&entity_section(
                "control",
                "Security Controls",
                controls,
                model,
            ));
        };

        // Risks
        if let Some(risks) = &self.risk {
            text.push_str(&entity_section("risk", "Risks", risks, model));
        };

        // Threats
        if let Some(threats) = &self.threat {
            text.push_str(&entity_section("threat", "Threats", threats, model));
        };

        // Regulations
        if let Some(regulations) = &self.regulation {
            text.push_str(&entity_section(
                "regulation",
                "Regulations",
                regulations,
                model,
            ));
        };

        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m = secmodel_core::load("../test/test_model.toml").unwrap();
        Report::report(&m, &m);
    }
}
