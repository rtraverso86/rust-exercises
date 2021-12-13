// the module name is implicit in the filename, no need to specify it

pub mod hosting;

pub mod serving {
    pub fn take_order() {}

    pub fn serve_order() {}

    fn take_payment() {} // private to the module
}
