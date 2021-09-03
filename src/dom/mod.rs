pub mod style;

#[derive(Debug, Clone)]
pub enum NodeType {
    Container,
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub ty: NodeType,
    pub styles: style::Styles,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Dom {
    pub root: Node,
}
