pub struct UnionFind {
    id: Vec<i32>,
    size: Vec<i32>,
    count: i32,
}

impl UnionFind {
    pub fn new(count: i32) -> Self {
        UnionFind {
            count,
            id: (0..count).collect(),
            size: vec![1; count as usize],
        }
    }

    pub fn is_connected(&mut self, p: i32, q: i32) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn find(&mut self, p: i32) -> i32 {
        let mut p = p;
        while p != self.id[p as usize] {
            self.id[p as usize] = self.id[self.id[p as usize] as usize];
            p = self.id[p as usize] as i32
        }
        p as i32
    }

    pub fn union_node(&mut self, p: i32, q: i32) {
        let i = self.find(p);
        let j = self.find(q);
        if i == j {
            return;
        }
        if self.size[i as usize] < self.size[j as usize] {
            self.id[i as usize] = j;
            self.size[j as usize] += self.size[i as usize];
        } else {
            self.id[j as usize] = i;
            self.size[i as usize] += self.size[j as usize];
        }
        self.count -= 1;
    }

    pub fn union_count(&self) -> i32 {
        self.count
    }

    pub fn node_size(&mut self, p: i32) -> i32 {
        let root = self.find(p);
        return self.size[root as usize];
    }
}
