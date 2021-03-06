use std::iter;

pub type Domino = (uint, uint);

/// A table keeping track of available dominoes.
///
/// Effectively a 6x6 matrix. Each position denotes whether a domino is available with that column
/// dots and row dots. Positions are mirrored ((3,4) == (4,3)), except for positions with equal row
/// and column numbers.
struct AvailabilityTable {
    m: Vec<uint>
}

impl AvailabilityTable {
    fn new() -> AvailabilityTable {
        AvailabilityTable { m: Vec::from_fn(6*6, |_| 0) }
    }

    fn get(&self, x: uint, y: uint) -> uint {
        self.m[(x-1) * 6 + (y-1)]
    }

    fn set(&mut self, x: uint, y: uint, v: uint) {
        let m = self.m.as_mut_slice();
        m[(x-1) * 6 + (y-1)] = v;
    }

    fn add(&mut self, x: uint, y: uint) {
        if x == y {
            let n = self.get(x, y);
            self.set(x, y, n + 1) // Along the diagonal
        }
        else {
            let m = self.get(x, y);
            self.set(x, y, m + 1);
            let n = self.get(y, x);
            self.set(y, x, n + 1);
        }
    }

    fn remove(&mut self, x: uint, y: uint) {
        if self.get(x, y) > 0 {
            if x == y {
                let n = self.get(x, y);
                self.set(x, y, n - 1) // Along the diagonal
            }
            else {
                let m = self.get(x, y);
                self.set(x, y, m - 1);
                let n = self.get(y, x);
                self.set(y, x, n - 1);
            }
        }
        else {
            // For this toy code hard explicit fail is best
            fail!("remove for 0 stones: ({}, {})", x, y)
        }
    }
    
    fn pop_first(&mut self, x: uint) -> Option<uint> {
        for y in iter::range_inclusive(1, 6) {
            if self.get(x, y) > 0 {
                self.remove(x, y);
                return Some(y)
            }
        }
        None
    }
}

pub fn chain(dominoes: &Vec<Domino>) -> Option<Vec<Domino>> {
    match dominoes.len() {
        0 => Some(vec!()),
        1 => Some(vec!(dominoes[0])),
        _ => {
            // First check if the total number of each amount of dots is even, if not it's not
            // possible to complete a cycle. This follows from that it's an Eulerian path.
            let mut v = vec!(0u, 0, 0, 0, 0, 0);
            // Keep the mutable borrow in a small scope here to allow v.iter().
            {
                let vs = v.as_mut_slice();
                for dom in dominoes.iter() {
                    vs[dom.val0()-1] += 1;
                    vs[dom.val1()-1] += 1;
                }
            }
            for n in v.iter() {
                if n % 2 != 0 {
                    return None
                }
            }
            Some(chain_worker(dominoes))
        }
    }
}

fn chain_worker(dominoes: &Vec<Domino>) -> Vec<Domino> {
    let mut doms = dominoes.clone();
    let first = doms.pop().unwrap();
    let mut t = AvailabilityTable::new();
    for dom in doms.iter() {
        t.add(dom.val0(), dom.val1())
    }
    let mut v: Vec<Domino> = Vec::new();
    v.push(first);
    let mut n = first.val1(); // Number to connect to
    loop {
        match t.pop_first(n) {
            None => break,
            Some(m) => {
                v.push((n, m));
                n = m;
            }
        }
    }
    v
}
