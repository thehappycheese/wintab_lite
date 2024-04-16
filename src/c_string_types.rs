use std::{
    ffi::c_uchar,
    ops::Deref
};


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CString40 {
    /// NOTE: the actual header definition says to use [c_char](std::ffi::c_char) which is signed [i8].
    /// This is super dumb. I think it is ok to not use [c_uchar] instead since we wont crash, our rust program
    /// will just interpret (correctly!) the signed integers as garbled nonsense unsigned ascii characters. 
    /// The worst conceivable outcome is that we set off the BELL character 40 times every time we print out a packet
    /// instead 
    inner:[c_uchar; 40]
}


impl Default for CString40 {
    fn default() -> Self {
        Self{inner: [0; 40]}
    }
}


impl Deref for CString40 {
    type Target=str;
    fn deref(&self) -> &Self::Target {
        let end_index = self.inner.iter().position(|char|*char == 0).unwrap_or(0);
        if end_index == 0 {
            return "";
        }else{
            let slice = &self.inner[..end_index];
            unsafe {
                std::str::from_utf8_unchecked(slice)
            }
        }
    }
}


impl std::fmt::Display for CString40 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}


impl std::fmt::Debug for CString40 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "c40\"{}\"", self.deref())
    }
}


impl CString40 {
    /// A utility function to write a string into the internal buffer.
    /// This function will truncate the string if it is longer than the CString40 buffer.
    /// If the input string is less than the internal buffer length,
    /// then the remainder of the inner buffer will be filled with null.
    pub fn write_str(&mut self, new_value:&str) {
        for (i, c) in self.inner.iter_mut().enumerate() {
            *c = match new_value.chars().nth(i) {
                Some(c) => match (c as i8).try_into() {
                    Ok(c) => c,
                    Err(_) => 32,
                },
                None => 0,
            }
        }
        // set the last value to null to guarantee that this is always a valid CString 🙄
        self.inner.last_mut().map(|c| *c = 0 );
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cstring40(){
        let mut buffer:[std::ffi::c_uchar; 40] = [0; 40];
        
        let cstring = CString40{inner: buffer.clone()};
        //let cast_string:&str = &cstring;
        //println!("cast_string='{cast_string}'");
        assert_eq!(cstring.len(), 0);

        let string_to_be_inserted = std::ffi::CString::new("12345").unwrap();
        // insert into buffer
        string_to_be_inserted.as_bytes().iter().enumerate().for_each(|(i, byte)|{
            buffer[i] = *byte;
        });

        let cstring = CString40{inner: buffer};
        // println!("cstring={cstring}");
        // println!("cstring debug={cstring:?}");
        assert_eq!(cstring.len(), 5);
        // println!("cast_string='{cast_string}'");
        assert_eq!(cstring.len(), 5);
    }
}
