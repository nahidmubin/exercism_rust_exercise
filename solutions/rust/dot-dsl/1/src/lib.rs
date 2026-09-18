pub mod graph {
    use std::collections::HashMap;

use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    

    impl Graph {
        pub fn new() -> Self {
            Self { nodes: Vec::new(), edges: Vec::new(), attrs: HashMap::new() }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())).collect();
            self
        }

        pub fn node(&self, title: &str) -> Option<&Node> {
            self.nodes.iter().find(|&n| n.name == title)
        }
    }

    pub mod graph_items {

        pub mod node{
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self { name: String::from(name), attrs: HashMap::new() }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())).collect();
                    self
                }
                pub fn attr(&self, title: &str) -> Option<&str> {
                    self.attrs.get(title).map(|val| val.as_str())
                }
            }
        }


        pub mod edge{
            use std::collections::HashMap;

use crate::graph::graph_items::node::Node;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                name: (Node, Node),
                attrs: HashMap<String, String>
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Self {name: (Node::new(a), Node::new(b)), attrs: HashMap::new()}
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())).collect();
                    self
                }

                pub fn attr(&self, title: &str) -> Option<&str> {
                    self.attrs.get(title).map(|val| val.as_str())
                }
            }
        }
    }
}

