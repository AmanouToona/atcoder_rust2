use amplify::confinement::Collection;
use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

#[derive(Default)]
struct TrieNode {
    children: HashMap<usize, TrieNode>,
    y: Vec<usize>,
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, x: usize, y: &[usize]) {
        let mut node = &mut self.root;
        for yy in y.iter() {
            node = node.children.entry(*yy).or_default();
        }
        node.y.push(x);
    }
}

struct Solve {
    trie: Trie,
    g: Vec<Vec<(usize, usize)>>,
    path: Vec<usize>,
    used: Vec<bool>,
}

impl Solve {
    fn new(trie: Trie, g: Vec<Vec<(usize, usize)>>) -> Self {
        let n = g.len();
        Solve {
            trie,
            g,
            path: Vec::new(),
            used: vec![false; n + 1],
        }
    }

    fn dfs(&mut self, u: usize) {
        self.used[u] = true;
        let g = self.g[u].clone();
        for &(v, y) in g.iter() {
            if self.used[v] {
                continue;
            }
            self.path.push(y);
            self.trie.insert(v, &self.path);
            self.dfs(v);
            self.path.pop();
        }
    }

    fn make_ans(&self) {
        // let mut q = VecDeque::new();
        let mut ans: Vec<usize> = Vec::new();

        let mut nxt: Vec<usize> = self.trie.root.children.keys().cloned().collect();
        nxt.sort();

        println!("nxt {:?}", nxt);
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        xy: [(usize, usize); N],
    }

    let mut g = vec![Vec::new(); N + 1];
    for (i, &(x, y)) in xy.iter().enumerate() {
        g[x].push((i, y));
    }

    let trie = Trie::new();
    let mut Solve = Solve::new(trie, g);
    for i in 0..=N {
        if Solve.used[i] {
            continue;
        }
        Solve.dfs(i);
    }

    Solve.make_ans();
}
