
extern crate capnp;


pub mod msgs {
    include!(concat!(env!("OUT_DIR"), "/rsmsg/hil_quaternion_capnp.rs"));
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
