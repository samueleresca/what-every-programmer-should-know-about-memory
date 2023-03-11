use std::arch::x86_64::_mm_stream_si32;

pub fn nocache_writes(n: usize) -> Vec<i32> {
    let mut data = vec![0i32; n];
    unsafe {
        for i in 0..n {
            _mm_stream_si32(data.as_mut_ptr().add(i), i as i32);
        }
    }

    data
}

pub fn standard_writes(n: usize) -> Vec<i32> {
    let mut data = vec![0i32; n];
    for i in 0..n {
        data[i] = i as i32;
    }
    data
}

#[cfg(test)]
mod tests {
    use crate::bypass_cache::{nocache_writes, standard_writes};

    #[test]
    fn nocache_writes_works() {
        let result = nocache_writes(3);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 2);
    }

    #[test]
    fn standard_writes_works() {
        let result = standard_writes(3);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 2);
    }
}
