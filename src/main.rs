use std::{time::Instant, io::{self, Read}};
use std::env;

use rand::prelude::*;
use rand_core::RngCore;

use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

const SIZE: usize = 2 << 32;


struct LimitRead<R>{
    reader: R,
    size: usize,
}

impl<A> LimitRead<A> {
    fn from(r: A, size: usize)  -> Self where A: Read {
        LimitRead{reader: r, size: size}
    }
}


impl<A: Read> Read for LimitRead<A> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> where A: Read {
        if self.size == 0 {
            return Ok(0)
        }
        let r = if buf.len() > self.size {
                     self.reader.read(&mut buf[0..self.size]) 
                
            } else {
                self.reader.read(buf)
            };
        
        match r {
            Ok(0) => Ok(0),
            Ok(n) => {self.size -=n; Ok(n)}
            e => e
        }

}
}
fn main() {
    let args: Vec<String> = env::args().collect();
    eprintln!("args {:?}", args);

    use std::str::FromStr;

    let size = usize::from_str(&args[1]).unwrap();

    let rng = rand::thread_rng();
    let xxx = Xoshiro256PlusPlus::from_rng(rng).unwrap();
    let mut rr = LimitRead::from(Box::new(xxx) as Box<dyn RngCore>, size);
    let start = Instant::now();
    std::io::copy(&mut rr,  &mut std::io::stdout()).unwrap();
    let dur = start.elapsed();
    eprintln!("size {}", size);
    eprintln!("sekunden verstrichen: {:?} ", size as  f64 / dur.as_secs_f64()/1024 as f64 /1024 as f64 /1024 as f64);
    
}

