use core::mem::MaybeUninit;

#[derive(Default)]
struct Value<'a> {
    inner: cb0r_sys::cbor_s,
}

enum CborError {
    MoreDataNeeded,
    BogusJunk,
    E(cb0r_sys::cb0r_e),
}

pub fn parse<'a>(v: &'a [u8]) -> Result<(Value<'a>, &'a[u8]), CborError> {
    let value = MaybeUninit::<Value<'a>>::uninit();
    let start = v.as_mut_ptr();
    let stop = p + v.len();
    let next = cb0r_sys::cb0r(start, stop, 0, value.inner.as_mut_ptr());
    let value = value.assume_init();

    // cb0r is funky an may read past the end of `v` (undefined behavior even in c)
    // even though this interface is safe, don't be supprised if some code isn't happy

    if next > stop {
        Err(CborError::MoreDataNeeded) 
    } else {
        use cb0r_sys::cb0r_e;
        // due to errors in the source code, no errors are emitted
        let t = value.type_();
        match t {
            cb0r_e::CB0R_ERR |
            cb0r_e::CB0R_EPARSE |
            cb0r_e::CB0R_EBAD |
            cb0r_e::CB0R_EBIG |
            cb0r_e::CB0R_EMAX => Err(CborError::E(t),
            _ => {
                Ok(value)
            }
        }
    }
}

#[test]
fn t1() {
    use hex_literal::hex;
    assert_eq!(parse(hex!("")), 
}
