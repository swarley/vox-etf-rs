extern crate eetf;
extern crate num_bigint;
extern crate rutie;

use std::convert::TryFrom;
use eetf::*;
use num_bigint::Sign;
use rutie::rubysys::value::ValueType;
use rutie::*;

module!(ETF);

methods!(
    ETF,
    _itself,
    fn encode(data: AnyObject) -> RString {
        if let Err(ref e) = data {
            rutie::VM::raise(Class::from_existing("ArgumentError"), &e.message());
        }
        let data = data.unwrap();

        let mut buf = Vec::new();
        let term = encode_object(data);
        term.encode(&mut buf).unwrap();

        let encoding = Encoding::find("UTF-8").unwrap();
        RString::from_bytes(buf.as_slice(), &encoding)
    }
);

fn encode_object(data: AnyObject) -> Term {
    match data.ty() {
        ValueType::Nil => Term::from(Atom::from("nil")),
        ValueType::True => Term::from(Atom::from("true")),
        ValueType::False => Term::from(Atom::from("false")),
        ValueType::Symbol => encode_symbol(data.try_convert_to::<Symbol>().unwrap()),
        ValueType::RString => encode_rstring(data.try_convert_to::<RString>().unwrap()),
        ValueType::Array => encode_array(data.try_convert_to::<Array>().unwrap()),
        ValueType::Hash => encode_hash(data.try_convert_to::<Hash>().unwrap()),
        ValueType::Float => encode_float(data.try_convert_to::<rutie::Float>().unwrap()),
        ValueType::Fixnum => encode_fixnum(data.try_convert_to::<Fixnum>().unwrap()),
        ValueType::Bignum => encode_bignum(data.try_convert_to::<Integer>().unwrap()),
        _ => {
            if data.respond_to("to_hash") {
                encode_object(unsafe { data.send("to_hash", &[]) })
            } else {
                rutie::VM::raise(Class::from_existing("ArgumentError"), "Unsupported!");
                Term::from(FixInteger::from(0))
            }
        }
    }
}

fn encode_symbol(sym: Symbol) -> Term {
    let bin = Binary::from(sym.to_str().as_bytes());
    Term::from(bin)
}

fn encode_rstring(rstring: RString) -> Term {
    let bin = Binary::from(rstring.to_bytes_unchecked());
    Term::from(bin)
}

fn encode_fixnum(num: Fixnum) -> Term {
    let bit_length = unsafe { num.send("bit_length", &[]) }
        .try_convert_to::<Fixnum>()
        .unwrap()
        .to_i64();
    if bit_length < 32 {
        Term::from(FixInteger::from(num.to_i32()))
    } else {
        Term::from(BigInteger::from(num.to_i64()))
    }
}

fn encode_bignum(num: Integer) -> Term {
    let bit_length = unsafe { num.send("bit_length", &[]) }
        .try_convert_to::<Fixnum>()
        .unwrap()
        .to_i64();
    if bit_length < 64 {
        Term::from(BigInteger::from(num.to_i64()))
    } else {
        Term::from(BigInteger::from(num.to_i64()))
    }
}

fn encode_array(arr: Array) -> Term {
    let mut terms = Vec::<Term>::new();
    for obj in arr.into_iter() {
        terms.push(encode_object(obj));
    }

    Term::from(List::from(terms))
}

fn encode_hash(hash: Hash) -> Term {
    let mut pairs = Vec::<(Term, Term)>::new();
    hash.each(|key, value| {
        let key_term = encode_object(key);
        let value_term = encode_object(value);
        pairs.push((key_term, value_term));
    });

    Term::from(Map::from(pairs))
}

// Why cant we use eetf::Float::from?
fn encode_float(float: rutie::Float) -> Term {
    Term::from(eetf::Float::try_from(float.to_f64()).unwrap())
}