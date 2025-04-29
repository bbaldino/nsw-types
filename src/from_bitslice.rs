use bitvec::{field::BitField, order::Msb0, slice::BitSlice};

use crate::*;

pub trait BitSliceUxExts {
    fn as_u1(&self) -> u1;
    fn as_u2(&self) -> u2;
    fn as_u3(&self) -> u3;
    fn as_u4(&self) -> u4;
    fn as_u5(&self) -> u5;
    fn as_u6(&self) -> u6;
    fn as_u7(&self) -> u7;
}

impl BitSliceUxExts for &BitSlice<u8, Msb0> {
    fn as_u1(&self) -> u1 {
        assert!(self.len() == 1);
        let value: u8 = self.load_be();
        u1::new(value)
    }

    fn as_u2(&self) -> u2 {
        assert!(self.len() == 2);
        let value: u8 = self.load_be();
        u2::new(value)
    }

    fn as_u3(&self) -> u3 {
        assert!(self.len() == 3);
        let value: u8 = self.load_be();
        u3::new(value)
    }

    fn as_u4(&self) -> u4 {
        assert!(self.len() == 4);
        let value: u8 = self.load_be();
        u4::new(value)
    }

    fn as_u5(&self) -> u5 {
        assert!(self.len() == 5);
        let value: u8 = self.load_be();
        u5::new(value)
    }

    fn as_u6(&self) -> u6 {
        assert!(self.len() == 6);
        let value: u8 = self.load_be();
        u6::new(value)
    }

    fn as_u7(&self) -> u7 {
        assert!(self.len() == 7);
        let value: u8 = self.load_be();
        u7::new(value)
    }
}
