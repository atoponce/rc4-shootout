struct Vmpc {
    i: u8,
    j: u8,
    state: [u8; 256]
}

impl Vmpc {
    fn new(key: &[u8], iv: &[u8]) -> Vmpc {
        assert!(!key.is_empty() && key.len() <= 256);
        assert!(key.len() == iv.len());

        let mut vmpc = Vmpc {
            i: 0,
            j: 0,
            state: [0; 256]
        };

        for i in 0..256 {
            vmpc.state[i] = i as u8;
        }

        for _ in 0..3 {
            for i in 0..256 {
                vmpc.j = vmpc.j.wrapping_add(vmpc.state[i].wrapping_add(key[i % key.len()]));
                vmpc.state.swap(i, vmpc.j as usize);
            }
        }

        for _ in 0..3 {
            for i in 0..256 {
                vmpc.j = vmpc.j.wrapping_add(vmpc.state[i].wrapping_add(iv[i % iv.len()]));
                vmpc.state.swap(i, vmpc.j as usize);
            }
        }

        vmpc = Vmpc {
            i: 0,
            j: 0,
            state: vmpc.state
        };

        vmpc
    }

    fn update(&mut self) -> u8 {
        let a = self.state[self.i as usize];
        self.j = self.state[self.j as usize].wrapping_add(a);
        let out = self.state[self.state[self.state[self.j as usize].wrapping_add(1) as usize] as usize];
        self.state.swap(self.i as usize, self.j as usize);

        out
    }
}

fn main() {
    use std::time::Instant;
    let key = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    let iv = &[0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe];
    let mut vmpc = Vmpc::new(key, iv);

    // vmpc.update() returns 1 byte, so this loop returns 1 billion bytes
    let now = Instant::now();
    {
        for _i in 0..1_000_000_000 {
            vmpc.update();
        }
    }
    let elapsed = now.elapsed();
    let rate = 1_000_000_000_f64 / (1_048_576_f64 * elapsed.as_secs_f64());
    println!("VMPC (MBps): {:.2?}", rate);
}