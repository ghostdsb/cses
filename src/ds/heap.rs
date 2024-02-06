pub struct Heap {
    pub values: Vec<u32>,
}

impl Heap {
    pub fn new() -> Self {
        Self { values: vec![0] }
    }

    pub fn from(vals: Vec<u32>) -> Self {
        let mut h = Self::new();
        for v in vals.iter() {
            h.values.push(*v);
        }

        h.heapify();
        h
    }

    pub fn print(&self){
        println!("{:?}", self.values);
    }

    pub fn insert(&mut self, val: u32) {
        self.values.push(val);
        let mut curr = self.values.len() - 1;
        while curr != 0 {
            if self.values[curr] < self.values[curr / 2] {
                (self.values[curr], self.values[curr / 2]) =
                    (self.values[curr / 2], self.values[curr]);
            }
            curr = curr / 2;
        }
        // println!("{:?}", self.values);
    }

    pub fn retrieve(&mut self) -> Option<u32> {
        let last = self.values.pop().unwrap();
        if self.values.len() == 1 {
            return Some(last);
        }
        if self.values.len() == 0 {
            return None;
        }
        let root = self.values[1];
        let mut curr = 1;
        self.values[curr] = last;
        while curr < self.values.len() {
            let (l, r) = (2 * curr, 2 * curr + 1);
            if l < self.values.len() && self.values[curr] > self.values[l] {
                (self.values[curr], self.values[l]) = (self.values[l], self.values[curr]);
                curr = l;
            } else if r < self.values.len() && self.values[curr] > self.values[r] {
                (self.values[curr], self.values[r]) = (self.values[r], self.values[curr]);
                curr = r;
            } else {
                break;
            }
        }
        // println!("{:?}", self.values);
        Some(root)
    }

    fn heapify(&mut self) {
        for i in (1..=self.values.len()).rev() {
            self.heapify_h(i)
        }
    }

    fn heapify_h(&mut self, i: usize) {
        let left = 2 * i;
        let right = 2 * i + 1;
        let mut smallest = i;
        if left < self.values.len() && self.values[i] > self.values[left] {
            smallest = left;
        }
        if right < self.values.len() && self.values[smallest] > self.values[right] {
            smallest = right;
        }
        if smallest != i {
            (self.values[i], self.values[smallest]) = (self.values[smallest], self.values[i]);
            self.heapify_h(smallest)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heapify_works() {
        let mut heap = Heap::from(vec![51, 15, 13, 14, 5,7, 10]);
        heap.print();
        let top = heap.retrieve();
        assert_eq!(top, Some(5));
        let top = heap.retrieve();
        assert_eq!(top, Some(7));
    }

    #[test]
    fn it_works() {
        let mut heap = Heap::new();
        heap.insert(51);
        heap.insert(15);
        heap.insert(13);
        heap.insert(14);
        heap.insert(5);
        heap.insert(7);
        heap.insert(10);
        heap.print();
        let top = heap.retrieve();
        assert_eq!(top, Some(5));
    }

    #[test]
    fn it_works_2() {
        let mut heap = Heap::new();
        heap.insert(5);
        heap.insert(1);
        heap.insert(3);

        let x = heap.retrieve();
        assert_eq!(x, Some(1));
        let x = heap.retrieve();
        assert_eq!(x, Some(3));

        heap.insert(2);

        let x = heap.retrieve();
        assert_eq!(x, Some(2));
        let x = heap.retrieve();
        assert_eq!(x, Some(5));
    }
}
