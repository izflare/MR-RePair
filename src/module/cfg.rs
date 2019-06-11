#[derive(Debug)]
pub struct Grammar {pub terminal: Vec<u8>, pub rule: Vec<Vec<u32>>, pub sequence: Vec<u32>,}

// pub trait ContextFree {
//     fn new() -> Grammar;
//     fn derive(&self, w: &mut Vec<u8>) -> ();
// }
//
// impl ContextFree for Grammar {
impl Grammar {
    pub fn new() -> Self {
        Self {terminal: Vec::new(), rule: Vec::new(), sequence: Vec::new(),}
    }

    pub fn derive(&self, w: &mut Vec<u8>) -> () {
        // zero is a special symbol
        fn dfs(i: usize, z: &Vec<u8>, g: &Vec<Vec<u32>>, w: &mut Vec<u8>) -> () {
            if i < z.len() + 1 {w.push(z[i - 1]);}
            else {
                let rs = &g[i - z.len() - 1];
                for e in rs {
                    dfs(*e as usize, z, g, w);
                }
            }
        }

        for c in &self.sequence {dfs(*c as usize, &self.terminal, &self.rule, w);}
    }
}


