//use indexmap::IndexMap;
use lazy_static::lazy_static;
use secmodel_core::*;
use serde::Serialize;
//use serde_json::json;
//use serde_json::value::Value;
use std::iter::Iterator;
use tera::{Context, Tera};
use thiserror::Error;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template(
            "default/actor",
            include_str!("../templates/default/actor.m"),
        ).unwrap();
        tera.add_raw_template(
            "default/agent",
            include_str!("../templates/default/agent.m"),
        ).unwrap();
        tera.add_raw_template(
            "default/application",
            include_str!("../templates/default/application.m"),
        ).unwrap();
        tera.add_raw_template(
            "default/store",
            include_str!("../templates/default/store.m"),
        ).unwrap();

        tera.add_raw_template(
            "default/model",
            include_str!("../templates/default/model.m"),
        ).unwrap();
        tera.add_raw_template(
            "default/server",
            include_str!("../templates/default/server.m"),
        ).unwrap();
        tera.add_raw_template(
            "default/endpoint",
            include_str!("../templates/default/endpoint.m"),
        ).unwrap();
        tera.add_raw_template(
            "default/network",
            include_str!("../templates/default/network.m"),
        ).unwrap();

        tera.add_raw_template(
            "default/application",
            include_str!("../templates/default/application.m"),
        ).unwrap();

        tera.add_raw_template(
            "default/flow",
            include_str!("../templates/default/flow.m"),
        ).unwrap();

        // tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Error rendering entity: {0}")]
    TemplateError(tera::Error),
}

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Node exists in graph: {0}")]
    NodeExists(String),
}

#[allow(dead_code)]
pub struct Vertex {
    id: String,
    shape: String,
    label: String,
}
#[allow(dead_code)]
pub struct Edge {
    a: String,
    b: String,
    link: String,
    label: String,
}
#[allow(dead_code)]
pub struct Graph {
    id: String,
    direction: String,
    vertices: Vec<Vertex>,
    graphs: Vec<Graph>,
    edges: Vec<Edge>,
}

pub enum Node<'a> {
    V(&'a Vertex),
    G(&'a Graph),
    E(&'a Edge),
}

pub enum MutNode<'a> {
    V(&'a mut Vertex),
    G(&'a mut Graph),
    E(&'a mut Edge),
}

pub trait Graphable {
    // iterate over all vertices in a
    fn vertex_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Vertex> + 'a>;
    fn graph_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Graph> + 'a>;
    fn edge_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Edge> + 'a>;
    fn vertex_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut Vertex> + 'a>;
    fn edge_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut Edge> + 'a>;
    fn get_node<'a>(&'a self, id: &str) -> Option<Node>;
}

impl Graphable for Graph {
    fn get_node<'a>(&'a self, id: &str) -> Option<Node> {
        if let Some(g) = self.graph_iter().find(|g| g.id == id) {
            Some(Node::G(g))
        } else if let Some(v) = self.vertex_iter().find(|v| v.id == id) {
            Some(Node::V(v))
        } else {
            None
        }
    }
    fn vertex_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Vertex> + 'a> {
        let mine = &self.vertices;
        let childrens = self.graphs.iter().map(|n| n.vertex_iter()).flatten();
        Box::new(mine.iter().chain(childrens))
    }
    fn graph_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Graph> + 'a> {
        let mine = &self.graphs;
        let childrens = self.graphs.iter().map(|n| n.graph_iter()).flatten();
        Box::new(mine.iter().chain(childrens))
    }
    fn edge_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Edge> + 'a> {
        let mine = &self.edges;
        let childrens = self.graphs.iter().map(|n| n.edge_iter()).flatten();
        Box::new(mine.iter().chain(childrens))
    }
    fn vertex_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut Vertex> + 'a> {
        let mine = &mut self.vertices;
        let childrens = self
            .graphs
            .iter_mut()
            .map(|n| n.vertex_iter_mut())
            .flatten();
        Box::new(mine.iter_mut().chain(childrens))
    }
    fn edge_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut Edge> + 'a> {
        let mine = &mut self.edges;
        let childrens = self.graphs.iter_mut().map(|n| n.edge_iter_mut()).flatten();
        Box::new(mine.iter_mut().chain(childrens))
    }
}

fn render_context<T: Serialize>(node: &T, id: &str, model: &Model) -> Context {
    let mut context = Context::new();
    context.insert("self", &serde_json::to_value(node).unwrap());
    context.insert("id", &serde_json::to_value(id).unwrap());
    context.insert("model", &serde_json::to_value(model).unwrap());
    context
}

pub trait Render {
    fn render(&self, id: &str, model: &Model) -> Result<String, RenderError>;
}

impl Render for Network {
    fn render(&self, id: &str, model: &Model) -> Result<String, RenderError> {
        let mut context = render_context(self, id, model);
        let mut children = String::new();
        let mut relations = String::new();
        if let Some(interfaces) = &self.interfaces {
            for i in interfaces.iter() {
                if let Some(network) = &i.network {
                    if i.downstream.unwrap_or(false) {
                        relations.push_str(&format!("\n{id} --> {network}\n"));
                    } else {
                        relations.push_str(&format!("\n{network} --> {id}\n"));
                    }
                }
            }
        }

        if let Some(servers) = &model.server {
            for (serverid, server) in servers {
                if let Some(interfaces) = &server.interfaces {
                    if interfaces.len() == 1 {
                        for i in interfaces {
                            if let Some(network) = &i.network {
                                if network == id {
                                    let sid = format!("server.{serverid}");
                                    children.push_str(&server.render(&sid, model).unwrap());
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(endpoints) = &model.endpoint {
            for (endpointid, endpoint) in endpoints {
                if let Some(interfaces) = &endpoint.interfaces {
                    for i in interfaces {
                        if let Some(network) = &i.network {
                            if network == id {
                                let eid = format!("endpoint.{endpointid}");
                                children.push_str(&endpoint.render(&eid, model).unwrap());
                            }
                        }
                    }
                }
            }
        }

        relations.push_str(&format!(
            "click {id} \"#{id}\" \"{}\"\n",
            self.get_title(id)
        ));
        context.insert("children", &children);
        context.insert("relations", &relations);
        Ok(TEMPLATES.render("default/network", &context).unwrap())
    }
}

impl Render for Server {
    fn render(&self, id: &str, model: &Model) -> Result<String, RenderError> {
        let mut context = render_context(self, id, model);
        let mut children = String::new();
        let mut relations = String::new();
        if let Some(interfaces) = &self.interfaces {
            if interfaces.len() > 1 {
                for i in interfaces.iter() {
                    if let Some(network) = &i.network {
                        if i.downstream.unwrap_or(false) {
                            relations.push_str(&format!("\n{id} --> {network}\n"));
                        } else {
                            relations.push_str(&format!("\n{network} --> {id}\n"));
                        }
                    }
                }
            }
        }
        if let Some(apps) = &self.applications {
            for appid in apps.iter() {
                let parts: Vec<&str> = appid.split(".").collect();
                if let Some(app) = model.application.as_ref().unwrap().get(parts[1]) {
                    children.push_str(&app.render(appid, model).unwrap());
                } else {
                    eprintln!("Could not find application {appid}");
                }
            }
        }
        if let Some(stores) = &self.stores {
            for storeid in stores.iter() {
                let parts: Vec<&str> = storeid.split(".").collect();
                if let Some(store) = model.store.as_ref().unwrap().get(parts[1]) {
                    children.push_str(&store.render(storeid, model).unwrap());
                } else {
                    eprintln!("Could not find store {storeid}");
                }
            }
        }
        context.insert("children", &children);
        context.insert("relations", &relations);
        Ok(TEMPLATES.render("default/server", &context).unwrap())
    }
}

impl Render for Endpoint {
    fn render(&self, id: &str, model: &Model) -> Result<String, RenderError> {
        let mut context = render_context(self, id, model);
        let mut children = String::new();
        let mut relations = String::new();

        if let Some(interfaces) = &self.interfaces {
            if interfaces.len() > 1 {
                for i in interfaces.iter() {
                    if let Some(network) = &i.network {
                        if i.downstream.unwrap_or(false) {
                            relations.push_str(&format!("\n{id} --> {network}\n"));
                        } else {
                            relations.push_str(&format!("\n{network} --> {id}\n"));
                        }
                    }
                }
            }
        }
        if let Some(apps) = &self.applications {
            for appid in apps.iter() {
                let parts: Vec<&str> = appid.split(".").collect();
                if let Some(app) = model.application.as_ref().unwrap().get(parts[1]) {
                    children.push_str(&app.render(appid, model).unwrap());
                } else {
                    eprintln!("Could not find application {appid}");
                }
            }
        }

        if let Some(agents) = &self.agents {
            for agentid in agents.iter() {
                let parts: Vec<&str> = agentid.split(".").collect();
                if let Some(agent) = model.agent.as_ref().unwrap().get(parts[1]) {
                    children.push_str(&agent.render(agentid, model).unwrap());
                } else {
                    eprintln!("Could not find agent {agentid}");
                }
            }
        }

        if let Some(stores) = &self.stores {
            for storeid in stores.iter() {
                let parts: Vec<&str> = storeid.split(".").collect();
                if let Some(store) = model.store.as_ref().unwrap().get(parts[1]) {
                    children.push_str(&store.render(storeid, model).unwrap());
                } else {
                    eprintln!("Could not find store {storeid}");
                }
            }
        }
        context.insert("children", &children);
        context.insert("relations", &relations);
        Ok(TEMPLATES.render("default/server", &context).unwrap())
    }
}

impl Render for Agent {
    fn render(&self, id: &str, model: &Model) -> Result<String, RenderError> {
        let mut context = render_context(self, id, model);
        let mut children = String::new();
        if let Some(actorid) = &self.actor {
            let name = actorid.split(".").collect::<Vec<&str>>()[1];
            if let Some(actor) = model.actor.as_ref().unwrap().get(name) {
                children.push_str(&actor.render(&format!("{id}.{actorid}"), model).unwrap());
            } else {
                eprintln!("Could not find actor {actorid}");
            }
        }
        if let Some(endpointid) = &self.endpoint {
            let name = endpointid.split(".").collect::<Vec<&str>>()[1];
            if let Some(_endpoint) = model.endpoint.as_ref().unwrap().get(name) {
                //children.push_str(&endpoint.render(endpointid, model).unwrap());
                children.push_str(&format!("{id}.{endpointid}\n"))
            } else {
                eprintln!("Could not find endpoint {endpointid}");
            }
        }
        context.insert("children", &children);
        Ok(TEMPLATES.render("default/agent", &context).unwrap())
    }
}

impl Render for Actor {
    fn render(&self, id: &str, model: &Model) -> Result<String, RenderError> {
        let context = render_context(self, id, model);
        Ok(TEMPLATES.render("default/actor", &context).unwrap())
    }
}

impl Render for Application {
    fn render(&self, id: &str, model: &Model) -> Result<String, RenderError> {
        let context = render_context(self, id, model);
        let mut diagram = String::new();
        diagram.push_str(&TEMPLATES.render("default/application", &context).unwrap());
        Ok(diagram)
    }
}

impl Render for Store {
    fn render(&self, id: &str, model: &Model) -> Result<String, RenderError> {
        let context = render_context(self, id, model);
        let mut diagram = String::new();
        diagram.push_str(&TEMPLATES.render("default/store", &context).unwrap());
        Ok(diagram)
    }
}

impl Render for Flow {
    fn render(&self, id: &str, _model: &Model) -> Result<String, RenderError> {
        let mut diagram = String::new();
        let parts: Vec<&str> = id.split(".").collect();
        let shortid = parts[1];
        //diagram.push_str(&TEMPLATES.render("default/flow", &context).unwrap());
        if let Some(sources) = &self.sources {
            for source in sources.iter() {
                if let Some(destinations) = &self.destinations {
                    for destination in destinations.iter() {
                        diagram.push_str(&format!("\n{source} ==>|{shortid}| {destination}\n"));
                    }
                }
            }
        }
        Ok(diagram)
    }
}

impl Render for Model {
    fn render(&self, _id: &str, model: &Model) -> Result<String, RenderError> {
        let mut diagram = String::new();
        diagram.push_str("---\nconfig:\n  theme: neutral\n\n---\n");
        diagram.push_str("flowchart TD\n\n");
        match &self.network {
            Some(nm) => {
                for (id, network) in nm.iter() {
                    let id = &format!("network.{id}");
                    diagram.push_str(&Render::render(network, &id, model).unwrap());
                }
            }
            _ => (),
        };
        if let Some(agents) = &self.agent {
            for (id, agent) in agents.iter() {
                let id = &format!("agent.{id}");
                diagram.push_str(&Render::render(agent, &id, model).unwrap());
            }
        };

        if let Some(servers) = &self.server {
            for (id, server) in servers.iter() {
                let id = &format!("server.{id}");
                diagram.push_str(&Render::render(server, &id, model).unwrap());
            }
        };
        if let Some(endpoints) = &self.endpoint {
            for (id, endpoint) in endpoints.iter() {
                let id = &format!("endpoint.{id}");
                diagram.push_str(&Render::render(endpoint, &id, model).unwrap());
            }
        };

        if let Some(applications) = &self.application {
            for (id, application) in applications.iter() {
                let id = &format!("application.{id}");
                diagram.push_str(&Render::render(application, &id, model).unwrap());
            }
        };

        if let Some(flow) = &self.flow {
            for (id, flow) in flow.iter() {
                let id = &format!("flow.{id}");
                diagram.push_str(&Render::render(flow, &id, model).unwrap());
            }
        };

        Ok(diagram)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let m = secmodel_core::load("../test/test_model.toml").unwrap();
        Render::render(&m, "", &m).unwrap();
    }

    #[test]
    fn test_graph() {
        let mut g = Graph {
            id: "Goo".to_string(),
            direction: "TD".to_string(),
            vertices: vec![],
            graphs: vec![],
            edges: vec![],
        };
        g.vertices.push(Vertex {
            id: "Bar".to_string(),
            shape: "circ".to_string(),
            label: "Bar".to_string(),
        });
        assert_eq!(2, g.vertex_iter().count());
    }
}
