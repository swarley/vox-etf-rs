extern crate eetf;
extern crate num_bigint;
extern crate rutie;

mod decode;
mod encode;

use rutie::*;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_vox_etf() {
    Module::from_existing("Vox").define(|itself| {
        itself.define_nested_module("ETF").define(|etf| {
            etf.def_self("decode", decode::decode);
            etf.def_self("encode", encode::encode);
        });
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
