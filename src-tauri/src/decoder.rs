use serde_json::{json, Value};

use std::{error::Error, collections::HashMap};
use base64::{Engine as _, engine::general_purpose};

pub fn decode_contents(contents: Vec<u8>) -> Result<Value, Box<dyn Error>> {
    let mut data = json!({});
    let mut decoder = Decoder::new(&contents);
    let length = decoder.read_u32() as usize;

    println!("[INFO] Mod contains {} bytes.", length);

    loop {
        let (field_type, field_name, field_value) = decoder.get_field();
        if field_value == FieldValue::Unknown {
            if decoder.cursor != length {
                println!("[ERROR] Found unknown field: [{}] {}\n\tCursor position: {}", field_type, field_name, decoder.cursor - field_name.len());
            }
            break
        }
        build_json(&mut data, field_name, &field_value);
    }

    println!("[INFO] Succesfully decoded {} fields.", decoder.fields_decoded);

    Ok(data)
}

fn build_json<T: serde_json::value::Index>(data: &mut Value, field_index: T, field_value: &FieldValue) {
    match field_value {
        FieldValue::Double(v) => { data[field_index] = json!(*v); },
        FieldValue::Bool(v) => { data[field_index] = json!(*v); },
        FieldValue::String(v) => { data[field_index] = json!(*v); },
        FieldValue::Int(v) => { data[field_index] = json!(*v); },
        FieldValue::Int64(v) => { data[field_index] = json!(*v); },
        FieldValue::Binary(v) => { data[field_index] = json!(*v); },
        FieldValue::Object(v) => {
            data[&field_index] = json!({});
            for (name, value) in v.iter() {
                build_json(&mut data[&field_index], name.clone(), value);
            }
        },
        FieldValue::Array(v) => {
            data[&field_index] = json!(vec![0; v.len()]);
            for (index, value) in v.iter().enumerate() {
                build_json(&mut data[&field_index], index, value);
            }
        },
        FieldValue::Null | FieldValue::Unknown => (),
    }
}

#[derive(Debug, PartialEq)]
enum FieldValue {
    Double(f64),
    String(String),
    Object(HashMap<String, FieldValue>),
    Array(Vec<FieldValue>),
    Bool(bool),
    Int(i32),
    Int64(i64),
    Binary(String),
    Unknown,
    Null
}

struct Decoder<'a> {
    cursor: usize,
    data: &'a Vec<u8>,
    fields_decoded: usize
}

impl<'a> Decoder<'a> {
    pub fn new(contents: &'a Vec<u8>) -> Self {
        Self {
            cursor: 0,
            data: contents,
            fields_decoded: 0
        }
    }

    fn get_field(&mut self) -> (u8, String, FieldValue) {
        let field_type = self.read_u8();
        let field_name = self.read_string_uknl();
        (field_type, field_name, self.get_value_from(field_type))
    }

    fn get_value_from(&mut self, field_type: u8) -> FieldValue {
        self.fields_decoded += 1;
        match field_type {
            0x1 => FieldValue::Double(self.read_double()),
            0x2 => FieldValue::String(self.read_string()),
            0x3 => FieldValue::Object(self.read_object()),
            0x4 => FieldValue::Array(self.read_array()),
            0x5 => FieldValue::Binary(self.read_binary()),
            0x8 => FieldValue::Bool(self.read_bool()),
            0xA => FieldValue::Null,
            0x10 => FieldValue::Int(self.read_i32()),
            0x12 => FieldValue::Int64(self.read_i64()),
            _ => FieldValue::Unknown
        }
    }

    pub fn read_bytes(&mut self, size: usize) -> &[u8] {
        let bytes = &self.data[self.cursor..(self.cursor + size)];
        self.cursor += size;
        bytes
    }
    
    pub fn read_i32(&mut self) -> i32 {
        i32::from_le_bytes(self.read_bytes(4).try_into().unwrap())
    }
    
    pub fn read_i64(&mut self) -> i64 {
        i64::from_le_bytes(self.read_bytes(8).try_into().unwrap())
    }

    pub fn read_u8(&mut self) -> u8 {
        u8::from_le_bytes(self.read_bytes(1).try_into().unwrap())
    }
    
    pub fn read_u32(&mut self) -> u32 {
        u32::from_le_bytes(self.read_bytes(4).try_into().unwrap())
    }

    pub fn read_double(&mut self) -> f64 {
        f64::from_le_bytes(self.read_bytes(8).try_into().unwrap())
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_u8() == 1
    }

    pub fn read_array(&mut self) -> Vec<FieldValue> {
        self.read_u32(); // Array bytes
        let mut v: Vec<FieldValue> = vec![];

        loop {
            let field_type = self.read_u8();
            if field_type == 0x00 { break }

            let field_name = self.read_string_uknl(); // Ignore field name
            let field = self.get_value_from(field_type);
            
            if field == FieldValue::Unknown {
                println!("[ERROR] Found unknown field: [{}] {}\n\tCursor position: {}", field_type, field_name, self.cursor - field_name.len());
                panic!();
            }

            v.push(field);
        }

        v
    }

    pub fn read_object(&mut self) -> HashMap<String, FieldValue> {
        self.read_u32(); // Array bytes
        let mut m: HashMap<String, FieldValue> = HashMap::new();

        loop {
            let field_type = self.read_u8();
            if field_type == 0x00 { break }

            let field_name = self.read_string_uknl();
            let field = self.get_value_from(field_type);
            
            if field == FieldValue::Unknown {
                println!("[ERROR] Found unknown field: [{}] {}\n\tCursor position: {}", field_type, field_name, self.cursor - field_name.len());
                panic!();
            }

            m.insert(field_name, field);
        }

        m
    }
    
    pub fn read_binary(&mut self) -> String {
        let length = self.read_u32();
        let bytes = self.read_bytes(length as usize + 1);
        general_purpose::STANDARD.encode(bytes[..(length as usize - 1)].to_vec())
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_u32();
        let bytes = self.read_bytes(length as usize);
        String::from_utf8(bytes[..(length as usize - 1)].to_vec()).unwrap()
    }

    // Read string with unknwon legnth, for field names
    pub fn read_string_uknl(&mut self) -> String {
        let mut str = String::new();
        for b in self.data[self.cursor..].iter() {
            self.cursor += 1;
            if *b == 0 { break } // Null terminator
            str.push(char::from(*b));
        }
        str
    }
}