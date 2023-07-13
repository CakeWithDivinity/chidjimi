use crate::parser::JsonObject;

pub fn serialize(object: JsonObject) -> Vec<u8> {
    match object {
        JsonObject::Null => vec![0xc0],
        JsonObject::Boolean(false) => vec![0xc2],
        JsonObject::Boolean(true) => vec![0xc3],
        JsonObject::Number(val) => {
            if val == val.trunc() {
                if val < 0.0 {
                    serialize_int(val)
                } else {
                    serialize_uint(val as u64)
                }
            } else {
                todo!()
            }
        }
        _ => todo!(),
    }
}

fn serialize_uint(val: u64) -> Vec<u8> {
    match val {
        val if val <= (1 << 7) - 1 => vec![val as u8],
        val if val <= u8::MAX.into() => vec![0xcc, val as u8],
        val if val <= u16::MAX.into() => {
            let mut val = (val as u16).to_be_bytes().to_vec();
            val.insert(0, 0xcd);
            val
        }
        val if val <= u32::MAX.into() => {
            let mut val = (val as u32).to_be_bytes().to_vec();
            val.insert(0, 0xce);
            val
        }
        _ => {
            let mut val = val.to_be_bytes().to_vec();
            val.insert(0, 0xcf);
            val
        }
    }
}

fn serialize_int(val: f64) -> Vec<u8> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_null() {
        assert_eq!(serialize(JsonObject::Null), vec![0xc0]);
    }

    #[test]
    fn test_serialize_false() {
        assert_eq!(serialize(JsonObject::Boolean(false)), vec![0xc2]);
    }

    #[test]
    fn test_serialize_true() {
        assert_eq!(serialize(JsonObject::Boolean(true)), vec![0xc3]);
    }

    #[test]
    fn test_serialize_uint() {
        assert_eq!(serialize(JsonObject::Number(127.0)), vec![0x7f]);
        assert_eq!(serialize(JsonObject::Number(255.0)), vec![0xcc, 0xff]);
        assert_eq!(
            serialize(JsonObject::Number(65535.0)),
            vec![0xcd, 0xff, 0xff]
        );
        assert_eq!(
            serialize(JsonObject::Number(4294967295.0)),
            vec![0xce, 0xff, 0xff, 0xff, 0xff]
        );
        assert_eq!(
            serialize(JsonObject::Number(u64::MAX as f64)),
            vec![0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
    }
}
