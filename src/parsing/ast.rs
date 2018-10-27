use parsing::token::Token;
use parsing::token::TokenType;


pub enum Node {
    Word(String),
    Symbol(String),

    Float(f64),
    Int(i64),

    List(Vec<NodeWrapper>),

    Invoke {
        target: String,
        with: Vec<NodeWrapper>
    }
}

pub struct NodeWrapper {
    pub node: Node,
    pub position: (usize, usize)
}

impl<'a> NodeWrapper {
    pub fn empty() -> NodeWrapper {
        NodeWrapper {
            node: Node::List(Vec::new()),
            position: (0, 0)
        }
    }

    pub fn new_list(elements: Vec<NodeWrapper>, position: (usize, usize)) -> NodeWrapper {
        NodeWrapper {
            node: Node::List(elements),
            position
        }
    }

    pub fn new_word(value: String, position: (usize, usize)) -> NodeWrapper {
        NodeWrapper {
            node: Node::Word(value),
            position
        }
    }

    pub fn new_symbol(value: String, position: (usize, usize)) -> NodeWrapper {
        NodeWrapper {
            node: Node::Symbol(value),
            position
        }
    }

    pub fn new_float(value: &str, position: (usize, usize)) -> NodeWrapper {
        let f_value = value.parse::<f64>().unwrap();

        NodeWrapper {
            node: Node::Float(f_value),
            position
        }
    }

    pub fn new_int(value: &str, position: (usize, usize)) -> NodeWrapper {
        let i_value = value.parse::<i64>().unwrap();

        NodeWrapper {
            node: Node::Int(i_value),
            position
        }
    }

    pub fn new_invoke(target: String, with: Vec<NodeWrapper>, position: (usize, usize))
        -> NodeWrapper {
            NodeWrapper {
                node: Node::Invoke { target, with },
                position
            }
        }
}