pub struct Race {
    time: usize,
    dist: usize,
}

impl Race {
    pub fn new(time: usize, dist: usize) -> Self {
        Self { time, dist }
    }

    pub fn ways_to_win(&self) -> usize {
        let mut t = self.time / 2;
        let mut count: usize = 0;
        while t * (self.time - t) > self.dist {
            count += 2;
            t -= 1;
        }
        if self.time % 2 == 0 {
            count -= 1;
        }
        count
    }
}
