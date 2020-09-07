extern crate eetf;
extern crate num_bigint;
extern crate rutie;

use eetf::*;
use num_bigint::Sign;
use rutie::rubysys::value::ValueType;
use rutie::*;
use std::io::Cursor;

const LSWORD_FIRST: i32 = 0x02;
const LSBYTE_FIRST: i32 = 0x20;
const PACK_LITTLE_ENDIAN: i32 = LSWORD_FIRST | LSBYTE_FIRST;
const PACK_NEGATIVE: i32 = 0x200;

module!(ETF);

methods!(
    ETF,
    _itself,
    fn decode(data: RString) -> AnyObject {
        if let Err(ref e) = data {
            rutie::VM::raise(Class::from_existing("ArgumentError"), &e.message());
        }
        let data = data.unwrap();

        let bytes = data.to_bytes_unchecked();
        let cursor = Cursor::new(&bytes);
        let decoded = Term::decode(cursor).unwrap();

        handle_term(decoded)
    }
);

fn handle_term(term: Term) -> AnyObject {
    match term {
        Term::Atom(a) => handle_atom(a),
        Term::FixInteger(f) => handle_int(f).into(),
        Term::BigInteger(b) => handle_big_int(b),
        Term::Float(fl) => handle_float(fl).into(),
        Term::Map(m) => handle_map(m).into(),
        Term::Tuple(t) => handle_tuple(t).into(),
        Term::List(l) => handle_list(l).into(),
        Term::ImproperList(i) => handle_improper_list(i).into(),
        Term::Binary(b) => handle_binary(b).into(),
        Term::BitBinary(bb) => handle_bit_binary(bb).into(),
        _ => NilClass::new().into(),
    }
}

fn handle_int(int: FixInteger) -> Fixnum {
    Fixnum::new(int.value.into())
}

fn handle_big_int(big_int: BigInteger) -> AnyObject {
    let (sign, bytes) = big_int.value.to_bytes_le();
    let negative = if sign == Sign::Minus {
        PACK_NEGATIVE
    } else {
        0
    };

    let num = Integer::unpack(
        bytes.as_slice(),
        bytes.len(),
        1,
        0,
        PACK_LITTLE_ENDIAN | negative,
    );
    num.into()
}

fn handle_float(fl: eetf::Float) -> rutie::Float {
    rutie::Float::new(fl.value)
}

fn handle_map(map: Map) -> Hash {
    let mut hash = Hash::new();
    for (key, value) in map.entries {
        let hash_key = normalize_hash_key(handle_term(key));
        hash.store(hash_key, handle_term(value));
    }
    hash
}

fn normalize_hash_key(obj: AnyObject) -> AnyObject {
    if obj.ty() == ValueType::RString {
        let string = obj.try_convert_to::<RString>().unwrap();
        Symbol::new(string.to_str()).into()
    } else {
        obj
    }
}

fn handle_tuple(tup: Tuple) -> Array {
    let mut arr = Array::with_capacity(tup.elements.len());
    for term in tup.elements {
        arr.push(handle_term(term));
    }
    arr
}

fn handle_list(list: List) -> Array {
    let mut arr = Array::with_capacity(list.elements.len());
    for term in list.elements {
        arr.push(handle_term(term));
    }
    arr
}

// ??????
fn handle_improper_list(impr_list: ImproperList) -> Array {
    let mut arr = Array::with_capacity(impr_list.elements.len() + 1);
    for term in impr_list.elements {
        arr.push(handle_term(term));
    }
    arr.push(handle_term(*impr_list.last));
    arr
}

fn handle_atom(atom: Atom) -> AnyObject {
    match atom.name.as_str() {
        "true" => Boolean::new(true).into(),
        "false" => Boolean::new(false).into(),
        "nil" => NilClass::new().into(),
        name => Symbol::new(name).into(),
    }
}

fn handle_binary(bin: Binary) -> RString {
    let encoding = Encoding::find("UTF-8").unwrap();
    RString::from_bytes(bin.bytes.as_slice(), &encoding)
}

fn handle_bit_binary(bit_bin: BitBinary) -> RString {
    let encoding = Encoding::find("UTF-8").unwrap();
    RString::from_bytes(bit_bin.bytes.as_slice(), &encoding)
}
