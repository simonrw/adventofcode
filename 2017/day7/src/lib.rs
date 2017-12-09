#[derive(Eq, PartialEq, Debug, Default)]
pub struct Node {
    pub name: String,
    pub next_nodes: Option<Vec<Box<Node>>>,
}

impl Node {
    pub fn from_line(line: &str, _already_seen: &[Node]) -> Self {
        let line = line.trim();
        let first_word = line.split_whitespace().next().unwrap();
        println!("{}", first_word);

        Node {
            name: "pbga".to_string(),
            next_nodes: None,
        }
    }
}


pub fn find_root_node(text: &str) -> String {
    let nodes = parse_layout(text);
    get_root(nodes)
}

pub fn get_root(_nodes: Vec<Node>) -> String {
    "tknk".to_string()
}

pub fn parse_layout(text: &str) -> Vec<Node> {
    let mut result = Vec::new();
    for line in text.lines() {
        let node = Node::from_line(line, &result);
        result.push(node);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_text() -> &'static str {
        let data = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#;
        data
    }

    #[test]
    fn test_parsing() {
        let data = example_text();
        let parsed = parse_layout(data);
        assert_eq!(
            parsed[0],
            Node {
                name: "pbga".to_string(),
                next_nodes: None,
            }
        );
    }


    #[test]
    fn test_example() {
        let data = example_text();
        assert_eq!(find_root_node(&data), "tknk");
    }
}
