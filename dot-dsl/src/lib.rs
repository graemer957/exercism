pub mod graph {
    use crate::graph::graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
                .collect();
            self
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(String::as_str)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Default, Eq, PartialEq)]
            pub struct Edge {
                left: String,
                right: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(left: &str, right: &str) -> Self {
                    Self {
                        left: left.to_string(),
                        right: right.to_string(),
                        ..Self::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
                        .collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(String::as_str)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Default, Eq, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        ..Self::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
                        .collect();
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(String::as_str)
                }
            }
        }
    }
}
