use soroban_sdk::{Env, String};

pub trait TakeFirstNCharsAndConcat {
    fn take_first_n_chars(&self, e: &Env, n: usize) -> String;
    fn concat(&self, e: &Env, other: String) -> String;
}

impl TakeFirstNCharsAndConcat for String {
    fn take_first_n_chars(&self, e: &Env, n: usize) -> String {
        let len = self.len() as usize;
        let mut slice: [u8; 100] = [0; 100];
        let min_len = len.min(n);
        self.copy_into_slice(&mut slice[..len]);

        String::from_str(&e, core::str::from_utf8(&slice[..min_len]).unwrap())
    }

    fn concat(&self, e: &Env, other: String) -> String {
        let len_0 = self.len() as usize;
        let len_1 = other.len() as usize;
        let mut slice: [u8; 35] = [0; 35];
        let combined_len = len_0 + len_1;

        self.copy_into_slice(&mut slice[..len_0]);
        other.copy_into_slice(&mut slice[len_0..combined_len]);

        String::from_str(&e, core::str::from_utf8(&slice[..combined_len]).unwrap())
    }
}
