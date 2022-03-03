pub struct GrammarTreeNode {
    rule: String,
    value: String,
    children: Vec<Box<GrammarTreeNode>>,
}
