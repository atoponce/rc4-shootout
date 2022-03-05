struct Spritz {
    i: u8,
    j: u8,
    k: u8,
    a: u8,
    w: u8,
    z: u8,
    state: [u8; 256]
}

impl Spritz {
    fn new() -> Spritz {
        let mut spritz = Spritz {
            i: 0,
            j: 0,
            k: 0,
            a: 0,
            w: 0,
            z: 0,
            state: [0; 256]
        };

        for i in 0..256 {
            spritz.state[i] = i as u8;
        }
        
        return spritz;
    }
    
    fn absorb(&mut self, data: &[u8]) {
        for byte in data {
            self.absorb_byte(&byte);
        }
    }

    fn absorb_byte(&mut self, byte: &u8) {
        self.absorb_nibble(byte & 0x0f); // low nibble
        self.absorb_nibble(byte & 0xf0); // high nibble
    }

    fn absorb_nibble(&mut self, nibble: u8) {
        if self.a >= 128 {
            self.shuffle();
        }

        self.state.swap(self.a as usize, (128 + nibble) as usize);

        self.a += 1;
    }

    fn absorb_stop(&mut self) {
        if self.a >= 128 {
            self.shuffle();
        }

        self.a += 1;
    }

    fn shuffle(&mut self) {
        self.whip();
        self.crush();
        self.whip();
        self.crush();
        self.whip();
        self.a = 0;
    }

    fn whip(&mut self) {
        for _i in 0..512 {
            self.update();
        }
        self.w = self.w.wrapping_add(2);
    }

    fn crush(&mut self) {
        for v in 0..128 {
            if self.state[v as usize] > self.state[(255 - v) as usize] {
                self.state.swap(v as usize, (255 - v) as usize);
            }
        }
    }

    fn squeeze(&mut self, r: u64) -> Vec<u8> {
        if self.a > 0 {
            self.shuffle();
        }

        let mut p = Vec::with_capacity(r as usize);

        for _i in 0..r {
            p.push(self.drip())
        }

        return p;
    }

    fn drip(&mut self) -> u8 {
        if self.a > 0 {
            self.shuffle();
        }

        self.update();
        
        return self.output();
    }

    fn update(&mut self) {
        self.i = self.i.wrapping_add(self.w);
        self.j = self.k.wrapping_add(self.state[self.j.wrapping_add(self.state[self.i as usize]) as usize]);
        self.k = self.k.wrapping_add(self.k).wrapping_add(self.state[self.j as usize]);
        self.state.swap(self.i as usize, self.j as usize);
    }

    fn output(&mut self) -> u8 {
        self.z = self.state[self.j.wrapping_add(
            self.state[self.i.wrapping_add(
                self.state[self.z.wrapping_add(self.k) as usize]
            ) as usize],
        ) as usize];

        return self.z
    }
}

fn main() {
    use std::time::Instant;
    let key = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    let mut spritz = Spritz::new();

    spritz.absorb(key);
    spritz.absorb_stop();

    // spritz.squeeze(1) returns 1 byte, so this loop returns 1 billion bytes
    let now = Instant::now();
    {
        for _i in 0..1_000_000_000 {
            spritz.squeeze(1);
        }
    }
    let elapsed = now.elapsed();
    let rate = 1_000_000_000 as f64 / (1048576 as f64 * elapsed.as_secs_f64());
    println!("Spritz (MBps): {:.2}", rate);
}
