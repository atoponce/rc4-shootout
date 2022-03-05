struct RC4A {
    i: u8,
    j1: u8,
    j2: u8,
    state1: [u8; 256],
    state2: [u8; 256]
}

impl RC4A {
    fn new(key1: &[u8]) -> RC4A {
        assert!(1 <= key1.len() && key1.len() <= 256);

        let mut rc4a = RC4A { i: 0, j1: 0, j2: 0, state1: [0; 256], state2: [0; 256] };

        for i in 0..256 {
            rc4a.state1[i] = i as u8;
        }

        for i in 0..256 {
            rc4a.j1 = rc4a.j1.wrapping_add(rc4a.state1[i].wrapping_add(key1[i % key1.len()]));
            rc4a.state1.swap(i, rc4a.j1 as usize);
        }

        rc4a.state2 = rc4a.state1;
        rc4a.state2.reverse(); // Cheating maybe, but it's a benchmark.

        RC4A { i: 0, j1: 0, j2: 0, state1: rc4a.state1, state2: rc4a.state2 };

        return rc4a;
    }

    fn update(&mut self) -> u16 {
        self.i = self.i.wrapping_add(1);

        self.j1 = self.j1.wrapping_add(self.state1[self.i as usize]);
        self.state1.swap(self.i as usize, self.j1 as usize);
        let res1 = self.state2[(self.state1[self.i as usize].wrapping_add(self.state1[self.j1 as usize])) as usize] as u16;

        self.j2 = self.j2.wrapping_add(self.state2[self.i as usize]);
        self.state2.swap(self.i as usize, self.j2 as usize);
        let res2 = self.state1[(self.state2[self.i as usize].wrapping_add(self.state2[self.j2 as usize])) as usize] as u16;

        return res1 << 8 | res2;
    }
}

fn main() {
    use std::time::Instant;
    let key = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    let mut rc4a = RC4A::new(key);

    // rc4a.update() returns 2 bytes, so this loop returns 2 billion bytes
    let now = Instant::now();
    {
        for _i in 0..1_000_000_000 {
            rc4a.update();
        }
    }
    let elapsed = now.elapsed();
    let rate = 2_000_000_000 as f64 / (1048576 as f64 * elapsed.as_secs_f64());
    println!("RC4A (MBps): {:.2?}", rate);
}
