pub struct GrammarTreeNode {
    rule: String,
    value: String,
    children: Vec<Box<GrammarTreeNode>>,
}

impl GrammarTreeNode {
    pub fn new() -> GrammarTreeNode {
        GrammarTreeNode {
            rule: String::new(),
            value: String::new(),
            children: Vec::new(),
        }
    }

    pub fn set_rule(&mut self, must_set: bool, rule: String) {
        if must_set || self.rule.is_empty() {
            self.rule = rule;
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn append_child(&mut self, node: Box<GrammarTreeNode>) {
        self.children.push(node)
    }
}
