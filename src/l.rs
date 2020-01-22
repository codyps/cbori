
struct Value<'a> {
    // all bytes within this value
    v: &[u8],

    // bytes in header. 1-9 (1 + 0-8 bytes count)
    header_sz: u8,
    kind: CborMajor,

    // union {
    //   count
    //   value
    // }
}


fn cbor_(i: &[u8], skip: usize, count_prop: Option<&mut usize>) -> Result<(Option<CborValue<'_>>, &[u8]), ()>
{
    let mut count_actual = 0;

    loop {
        if i.len() == 0 {
            return Ok((None, i));
        }

        let mut element_bytes = 1;
        let f = i[0];
        let mut hsz = 1;
        let mut count = 0;

        match f {
            // 
            0x1b|0x3b|0xfb|
            0x1a|0x3a|0xfa|
            0x19|0x39|0xf9|
            0x18|0x38|0xf8|
            0x00..=0x17|
            0x20..=0x37|
            0xe0..=0xf7 => {
                let mut through = if let 0x1b|0x3b|0xfb { true } else { false };
                if through {
                    element_bytes += 4;
                }
                
                if let 0x1a|0x3a|0xfa = f {
                    through = true;
                }

                if through {
                    element_bytes += 2;
                }

                if let 0x19|0x39|0xf9 = f {
                    through = true;
                }
                
                if through {
                    element_bytes += 1;
                }
                
                if let 0x18|0x38|0xf8 = f {
                    through = true;
                }

                if through {
                    element_bytes += 1;
                    if element_bytes > i.len() {
                        return Err(());
                    }
                }
                
            },
            // bytes/string: 64-bit len
            0x5b|0x7b => {
                return Err(());
            },
            // bytes/string: 32-bit len
            0x5a|0x7a|
            0x59|0x79|
            0x58|0x78 => {

                let mut through = if let 0x5a|0x7a = f { true } else { false };

                if through {
                    // 32-bit len
                    header_sz += 2; 
                    if i.len() < header_sz {
                        return Err(());
                    }

                    element_bytes += (i as u32) << 24;
                    element_bytes += (i as u32) << 16;
                }

                if let 0x59|0x79 = f {
                    // 16-bit len
                    through = true;
                }

                if through {
                    header_sz += 1;

                    if i.len() < header_sz {
                        return Err(());
                    }

                    element_bytes += (i[header_sz - 1] as u32) << 8;
                }

                if let 0x58|0x78 = f {
                    // 8-bit len
                    through = true;
                }

                if through {
                    header_sz += 1;
                    if i.len() < header_sz {
                        return Err(());
                    }

                    element_bytes += header_sz + (i[header_sz - 1] as u32);
                } else {
                    panic!();
                }
            },
            // bytes/strings: embedded lengths
            0x40..=0x57|
            0x60..=0x77 => {
                element_bytes += f & 0x1f;
                if element_bytes > i.len() {
                    return Err(());
                }
            },
    
            // array/map: 64-bit count
            0x9b|0xbb => {
                // ebig
                return Err(());
            },
            // array/map: 32/16/8-bit count
            0x9a|0xba|
            0x99|0xb9|
            0x98|0xb8|
            0x80..=0x97|
            0xa0..=0xb7 => {
                let mut through = if let 0x9a|0xba = f { true } else { false }

                if through {
                    header_sz += 2;
                    if i.len() < header_sz {
                        return Err(());
                    }

                    count = (i[1] as u32) << 24;
                    count |= (i[2] as u32) << 16;
                }

                if let 0x99|0xb9 = f {
                    through = true;
                }

                if through {
                    header_sz += 1;
                    if i.len() < header_sz {
                        return Err(());
                    }

                    count |= (i[header_sz - 1] as u32) << 8;
                }

                if let 0x98|0xb8 = f {
                    header_sz += 1;
                    if i.len() < header_sz {
                        return Err(());
                    }

                    count |= (i[header_sz - 1) as u32;
                }

                if let 0x80..=0x97|0xa0..=0xb7 = f {
                    count = f & 0x1f;
                }

                // l_skip
                
                if count > 0 {
                    if (f & 0x20) != 0 { 
                        // double items for maps
                        count <<= 1;
                    }

                    let next_s = &i[header_sz..];

                    // FIXME: avoid recursion
                    let (nv, rem) = cbor_(next_s, count - 1, None)?;

                    count_actual = count;
                    // we want to recover the consumed bytes from the `cbor_()` call as a count
                    // given the structure of the return value, nothing actually includes that info
                    // directly.
                    //
                    // In c, we'd do some pointer arithmetic on the remaining data start pointer.
                    // That's _technically_ legal in rust, but is even nastier than it is in C.
                    element_bytes = rem.as_ptr() - i.as_ptr();
                } else {
                    element_bytes += header_sz;
                }

                if element_bytes > i.len() {
                    return Err(());
                }
            },

            0x5f| // bytes
            0x7f| // text
            0x9f| // array
            0xbf => { // map
                count = None;
                let next_s = &i[header_sz..];
                let (nv, rem) = cbor_(next_s, count, &mut count_actual)?;
                if let None = nv {
                    // values are required
                    // TODO: validate number of values?
                    return Err(());
                }

                // XXX: this is ugly
                element_bytes = rem.as_ptr() - i.as_ptr();
            },

            // tags
            0xdb|0xda|0xd9|0xd8|
            0xc0..=0xd7 => {
                let mut through = if let 0xdb = f { true } else { false };

                if through {
                    header_sz += 4;
                }

                if let 0xda = f {
                    through = true;
                }

                if through {
                    header_sz += 2;
                }

                if let 0xd9 = f {
                    through = true;
                }

                if through {
                    header_sz += 1;
                }

                if let 0xd8 = f {
                    through = true;
                }

                if through {
                    header_sz += 1;
                    if i.len() < header_sz {
                        return Err(());
                    }
                }

                let next_s = &i[header_sz..];

                let (nv, rem) = cbor_(next_s, 0, None)?;
                if let None = nv {
                    // error, a value is required
                    return Err(());
                }

                // XXX: dirty
                element_bytes = rem.as_ptr() - i.as_ptr();
            },
            0xff => {
                if let None = skip {
                    count_prop = count_actual;
                    // return a the remainder
                    // but no value. consider effect on recursive calls that check for `None` here.
                    // May need to have them examine some other return data.
                    return Ok(None, &i[header_sz..]);
                } else {
                    // unexpected break
                    return Err(());
                }
            },
            _ =>  {
                // invalid byte
                return Err(());
            },
        }

        if skip != 0 {
            // compose value?
            // consider splitting this into another fn
        }
    }
}



		if (!skip) {
			struct slice v = slice_from_parts(input->d, i.d - input->d + element_bytes);
			*input = slice_to_end(i, element_bytes);

			if (!val) {
				return 1;
			}

			enum cbor_major kind = f >> 5;

			*val = (struct cbor_value) {
				.v = v,
				.kind = kind,
				.hsz = hsz,
			};

			switch (kind) {
			case CBORM_UINT:
			case CBORM_NINT:
			case CBORM_TAG:
				switch (hsz) {
				case 9:
					val->value |= (uint64_t)i.d[hsz - 8] << 56;
					val->value |= (uint64_t)i.d[hsz - 7] << 48;
					val->value |= (uint64_t)i.d[hsz - 6] << 40;
					val->value |= (uint64_t)i.d[hsz - 5] << 32;
					/* fallthrough */
				case 5:
					val->value |= (uint32_t)i.d[hsz - 4] << 24;
					val->value |= (uint32_t)i.d[hsz - 3] << 16;
					/* fallthrough */
				case 3:
					val->value |= (uint32_t)i.d[hsz - 2] << 8;
					/* fallthrough */
				case 2:
					val->value |= i.d[hsz - 1];
					break;
				case 1:
					val->value = i.d[0] & 0x1f;
					break;
				}
			case CBORM_BYTES:
			case CBORM_TEXT:
				// XXX: handle indefinite bytes/text
				break;
			case CBORM_ARRAY:
			case CBORM_MAP:
				val->count = count_actual;
				break;
			case CBORM_MISC:
				// XXX: decode to specific types
				break;
			}

			return 1;
		}

		if (skip != CBOR_STREAM)
			skip--;
		count_actual++;

		i = slice_to_end(i, element_bytes);
	}
}
