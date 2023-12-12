struct Memo {
    left: Option<isize>,
    right: isize,
    sum: usize,
}

impl Memo {
    pub fn new(right: isize) -> Self {
        Self { left: None, right, sum: 0 }
    }

    pub fn push(&mut self, value: isize) -> Option<isize> {
        self.left = Some(self.right);
        self.right = value;
        let left = self.left?;
        let diff = value - left;
        self.sum += diff.abs() as usize;
        if diff != 0 {
            Some(diff)
        } else {
            None
        }
    }
}

pub struct Sequence {
    first: Vec<isize>,
    memos: Vec<Memo>,
}

impl Sequence {
    pub fn new(first: Vec<isize>) -> Self {
        Self { first, memos: Vec::new() }
    }

    pub fn advance(&mut self) -> isize {
        /*
         * To avoid NxM/2 space, we want to compute the sequences
         * lazily, tracking only the past two values, plus a running
         * total to identify when all values are zero.
         */
        let pairs = self.first.iter().zip(
            self.first.iter().skip(1));
        for (l, r) in pairs {
            let memos = self.memos.iter_mut();
            let mut carry: isize = r-l;
            for memo in memos {
                if let Some(value) = memo.push(carry) {
                    carry = value;
                } else {
                    carry = 0;
                }
            }
            if carry != 0 {
                self.memos.push(Memo::new(carry));
            }
        }
        self.memos.iter().map(|memo| memo.right).sum::<isize>() +
            self.first.last().expect("No numbers?")
    }
}
