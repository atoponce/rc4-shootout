struct VMPC {
    i: u8,
    j: u8,
    state: [u8; 256]
}

impl VMPC {
    fn new(key: &[u8], iv: &[u8]) -> VMPC {
        assert!(1 <= key.len() && key.len() <= 256);
        assert!(key.len() == iv.len());

        let mut vmpc = VMPC {
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

        VMPC {
            i: 0,
            j: 0,
            state: vmpc.state
        };

        return vmpc;
    }

    fn update(&mut self) -> u8 {
        let a = self.state[self.i as usize];
        self.j = self.state[self.j as usize].wrapping_add(a);
        let out = self.state[self.state[self.state[self.j as usize].wrapping_add(1) as usize] as usize];
        self.state.swap(self.i as usize, self.j as usize);

        return out;
    }
}

fn main() {
    use std::time::Instant;
    let key = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    let iv = &[0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe];
    let mut vmpc = VMPC::new(key, iv);

    // vmpc.update() returns 1 byte, so this loop returns 1 billion bytes
    let now = Instant::now();
    {
        for _i in 0..1_000_000_000 {
            vmpc.update();
        }
    }
    let elapsed = now.elapsed();
    let rate = 1_000_000_000 as f64 / (1048576 as f64 * elapsed.as_secs_f64());
    println!("VMPC (MBps): {:.2?}", rate);
}