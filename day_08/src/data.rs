pub type NodeRef<'a> = &'a str;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Node<'a> {
    pub name: NodeRef<'a>,
    pub left: NodeRef<'a>,
    pub right: NodeRef<'a>,
}

pub fn parse_node(s: &str) -> Node {
    Node {
        name: &s[0..=2],
        left: &s[7..=9],
        right: &s[12..=14],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_node_from_str() {
        assert_eq!(
            parse_node(&get_input::<String>(Input::Test1)[3]),
            Node {
                name: "BBB",
                left: "AAA",
                right: "ZZZ",
            }
        );
    }
}
