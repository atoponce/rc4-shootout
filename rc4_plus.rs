struct RC4Plus {
    i: u8,
    j: u8,
    state: [u8; 256]
}

impl RC4Plus {
    fn new(key: &[u8], iv: &[u8]) -> RC4Plus {
        assert!(1 <= key.len() && key.len() <= 256);
        assert!(key.len() == iv.len());

        let mut rc4_plus = RC4Plus {
            i: 0,
            j: 0,
            state: [0; 256]
        };

        // Initialization
        for i in 0..256 {
            rc4_plus.state[i] = i as u8;
        }

        // Layer 1: Basic Scrambling
        for i in 0..256 {
            rc4_plus.j = rc4_plus.j.wrapping_add(rc4_plus.state[i].wrapping_add(key[i % key.len()]));
            rc4_plus.state.swap(i, rc4_plus.j as usize);
        }

        // Layer 2: Scrambling with IV
        for i in (0..128).rev() {
            rc4_plus.j = rc4_plus.j.wrapping_add(rc4_plus.state[i]) ^ key[i % key.len()].wrapping_add(iv[i % iv.len()]);
            rc4_plus.state.swap(i, rc4_plus.j as usize);
        }

        // Layer 3: Zigzag Scrambling
        for y in 0..256 {
            let i;

            if y % 2 == 0 {
                i = y / 2;
            } else {
                i = 256 - (y + 1) / 2;
            }

            rc4_plus.j = rc4_plus.j.wrapping_add(
                rc4_plus.state[i].wrapping_add(
                    key[i % key.len()]
                )
            );
            rc4_plus.state.swap(i, rc4_plus.j as usize);
        }

        RC4Plus {
            i: 0,
            j: 0,
            state: rc4_plus.state
        };

        return rc4_plus;
    }

    fn update(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.state[self.i as usize]);
        self.state.swap(self.i as usize, self.j as usize);

        let t1 = self.state[self.i as usize].wrapping_add(self.state[self.j as usize]);
        let t2 = (self.state[(self.i << 5 ^ self.j >> 3) as usize].wrapping_add(
            self.state[(self.j << 5 ^ self.i >> 3) as usize]
        )) ^ 0xaa;
        let t3 = self.j.wrapping_add(self.state[self.j as usize]);

        return (self.state[t1 as usize].wrapping_add(t2) ^ t3) as u8;
    }
}

fn main() {
    use std::time::Instant;
    let key = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    let iv = &[0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe];
    let mut rc4_plus = RC4Plus::new(key, iv);

    // rc4_plus.update() returns 1 byte, so this loop returns 1 billion bytes
    let now = Instant::now();
    {
        for _i in 0..1_000_000_000 {
            rc4_plus.update();
        }
    }
    let elapsed = now.elapsed();
    let rate = 1_000_000_000 as f64 / (1048576 as f64 * elapsed.as_secs_f64());
    println!("RC4+ (MBps): {:.2?}", rate);
}
