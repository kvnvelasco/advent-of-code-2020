use std::cell::RefCell;
use std::collections::HashSet;

type BagIndex = HashSet<String>;

struct BagNode {
    count: usize,
    kind: String,
    children: RefCell<Vec<BagNode>>,
}
