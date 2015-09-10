#![feature(test)]
#![feature(const_fn)]

extern crate test;
extern crate rand;
extern crate num;
pub mod tests;
use num::pow;

/// Items will be sorted based on the values returned by this trait
pub trait Indexed {
    fn get_index(&self) -> u64;
}

/// Sorts [source] and yield results in a new vector
pub fn sort<T: Clone + Default + Indexed> (source:&Vec<T>) -> Vec<T> {
    let radix = 8;
    let steps = (64 / radix) as usize;
    let buckets:usize = pow(2, radix);
    let mask:u64 = (buckets - 1) as u64;
    let mut shift:usize = 0;
    let mut bucket_sums = vec![0usize; buckets];
    let mut bucket_counts = vec![0usize; buckets];

    let count = source.len();
    let mut buffers = [Some(source.clone()), Some(vec![Default::default(); count])];
    for step in 0..steps {
        let (source, workspace) = {
            let mut iter = buffers.iter_mut();
            let b0 = iter.next().unwrap();
            let b1 = iter.next().unwrap();
            if step % 2 == 0 {
                (b0.as_ref().unwrap(), b1.as_mut().unwrap())
            } else {
                (b1.as_ref().unwrap(), b0.as_mut().unwrap())
            }
        };

        if step > 0 {
            for bucket in &mut bucket_counts {
                *bucket = 0;
            }
        }

        for entry in source.iter() {
            let code = entry.get_index();
            let idx = (code >> shift & mask) as usize;
            bucket_counts[idx] += 1;
        }

        bucket_sums[0] = 0;
        for i in 1..buckets {
            bucket_sums[i] = bucket_sums[i - 1] + bucket_counts[i - 1];
        }

        for entry in source.iter() {
            let entry = entry.clone();
            let code = entry.get_index();
            let idx = (code >> shift & mask) as usize;
            let sum = bucket_sums[idx];
            workspace[sum] = entry;
            bucket_sums[idx] = sum + 1;
        }

        shift = shift + radix;
    }

    buffers[steps % 2].take().unwrap()
}
