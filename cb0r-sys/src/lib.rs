/* automatically generated by rust-bindgen */

#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum cb0r_e {
    CB0R_INT = 0,
    CB0R_NEG = 1,
    CB0R_BYTE = 2,
    CB0R_UTF8 = 3,
    CB0R_ARRAY = 4,
    CB0R_MAP = 5,
    CB0R_TAG = 6,
    CB0R_SIMPLE = 7,
    CB0R_TAGS = 8,
    CB0R_DATETIME = 9,
    CB0R_EPOCH = 10,
    CB0R_BIGNUM = 11,
    CB0R_BIGNEG = 12,
    CB0R_FRACTION = 13,
    CB0R_BIGFLOAT = 14,
    CB0R_BASE64URL = 15,
    CB0R_BASE64 = 16,
    CB0R_HEX = 17,
    CB0R_DATA = 18,
    CB0R_SIMPLES = 24,
    CB0R_FALSE = 25,
    CB0R_TRUE = 26,
    CB0R_NULL = 27,
    CB0R_UNDEF = 28,
    CB0R_FLOAT = 29,
    CB0R_ERR = 224,
    CB0R_EPARSE = 225,
    CB0R_EBAD = 226,
    CB0R_EBIG = 227,
    CB0R_EMAX = 228,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb0r_s {
    pub start: *mut u8,
    pub end: *mut u8,
    pub __bindgen_anon_1: cb0r_s__bindgen_ty_1,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub header: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cb0r_s__bindgen_ty_1 {
    pub length: u64,
    pub count: u64,
    pub value: u64,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_cb0r_s__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<cb0r_s__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(cb0r_s__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<cb0r_s__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(cb0r_s__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<cb0r_s__bindgen_ty_1>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cb0r_s__bindgen_ty_1),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<cb0r_s__bindgen_ty_1>())).count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cb0r_s__bindgen_ty_1),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<cb0r_s__bindgen_ty_1>())).value as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cb0r_s__bindgen_ty_1),
            "::",
            stringify!(value)
        )
    );
}
#[test]
fn bindgen_test_layout_cb0r_s() {
    assert_eq!(
        ::core::mem::size_of::<cb0r_s>(),
        32usize,
        concat!("Size of: ", stringify!(cb0r_s))
    );
    assert_eq!(
        ::core::mem::align_of::<cb0r_s>(),
        8usize,
        concat!("Alignment of ", stringify!(cb0r_s))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<cb0r_s>())).start as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cb0r_s),
            "::",
            stringify!(start)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<cb0r_s>())).end as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cb0r_s),
            "::",
            stringify!(end)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<cb0r_s>())).header as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(cb0r_s),
            "::",
            stringify!(header)
        )
    );
}
impl cb0r_s {
    #[inline]
    pub fn type_(&self) -> cb0r_e {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_type(&mut self, val: cb0r_e) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(type_: cb0r_e) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let type_: u32 = unsafe { ::core::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type cb0r_t = *mut cb0r_s;
extern "C" {
    pub fn cb0r(start: *mut u8, stop: *mut u8, skip: u32, result: cb0r_t) -> *mut u8;
}
extern "C" {
    pub fn cb0r_read(in_: *mut u8, len: u32, result: cb0r_t) -> bool;
}
extern "C" {
    pub fn cb0r_get(array: cb0r_t, index: u32, result: cb0r_t) -> bool;
}
extern "C" {
    pub fn cb0r_find(
        map: cb0r_t,
        type_: cb0r_e,
        number: u64,
        bytes: *mut u8,
        result: cb0r_t,
    ) -> bool;
}
extern "C" {
    pub fn cb0r_write(out: *mut u8, type_: cb0r_e, number: u64) -> u8;
}
extern "C" {
    pub fn cb0r_value(data: cb0r_t) -> *mut u8;
}
extern "C" {
    pub fn cb0r_vlen(data: cb0r_t) -> u32;
}
