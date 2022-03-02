struct RC4 {
    i: u8,
    j: u8,
    state: [u8; 256]
}

impl RC4 {
    fn new(key: &[u8]) -> RC4 {
        assert!(1 <= key.len() && key.len() <= 256);

        let mut rc4 = RC4 {
            i: 0,
            j: 0,
            state: [0; 256]
        };

        for i in 0..256 {
            rc4.state[i] = i as u8;
        }

        for i in 0..256 {
            rc4.j = rc4.j.wrapping_add(rc4.state[i].wrapping_add(key[i % key.len()]));
            rc4.state.swap(i, rc4.j as usize);
        }

        RC4 {
            i: 0,
            j: 0,
            state: rc4.state
        };

        return rc4;
    }

    fn update(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.state[self.i as usize]);
        self.state.swap(self.i as usize, self.j as usize);

        return self.state[
            self.state[self.i as usize].wrapping_add(
                self.state[self.j as usize]
            )
         as usize];
    }
}

fn main() {
    use std::time::Instant;
    let key = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    let mut rc4 = RC4::new(key);

    // rc4.update() returns 1 byte, so this loop returns 1 billion bytes
    let now = Instant::now();
    {
        for _i in 0..1_000_000_000 {
            rc4.update();
        }
    }
    let elapsed = now.elapsed();
    let rate = 1_000_000_000 as f64 / (1048576 as f64 * elapsed.as_secs_f64());
    println!("RC4 (MBps): {:.2?}", rate);
}
