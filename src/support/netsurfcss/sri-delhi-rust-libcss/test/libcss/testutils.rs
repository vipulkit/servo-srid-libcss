#[link(name = "testutils",
       vers = "0.2",
       url = "https://github.com/webconvforge/sri-delhi-rust-libcss/tree/master/libparserutils")];

#[crate_type = "lib"];
extern mod css;
use std::io::*;

pub type  line_func =  
    ~extern fn(data:~str , pw:LINE_CTX_DATA_TYPE) -> bool;

pub struct line_ctx_csdetect {
    buf:~[u8],
    enc:~str,
    indata:bool,
    inenc:bool
}
/*pub type  line_func =  
    ~extern fn(data:~str , pw:&mut line_ctx) -> bool;*/

pub struct line_ctx_lex {
    buf:~[u8],

    exp:~[~str],

    indata:bool,
    inexp:bool
}

pub enum LINE_CTX_DATA_TYPE {
    CSDETECT(@mut line_ctx_csdetect),
    LEX(@mut line_ctx_lex)
}

pub fn css__parse_filesize( fileName:~str)->uint {
    // debug!(~"css__parse_filesize : "+ fileName);
    let r:@Reader = file_reader(&Path(fileName)).unwrap(); 
    r.seek(0,SeekEnd);
    r.tell()
}

pub fn css__parse_strnchr(string:&~str, chr:char)-> (~str,uint) {
    let length = string.len();
	let mut i : uint = 0;
    while i < length {
        if (*string)[i] as char == chr {
            return (string.slice(i,length).to_owned(),i);
        }
		
		i = i + 1;
    }
    return (~"",string.len());
}
pub fn css__parse_testfile(filename:~str,  callback:line_func, pw:LINE_CTX_DATA_TYPE)->bool {
    // debug!(~"css__parse_testfile : "+ filename);
    let r:@Reader = file_reader(&Path(filename)).unwrap();
    let mut data:~str;
    let mut string: ~str;

    while(!r.eof()) {               
        data = r.read_line();
        // io::print(fmt!("data is %? " , str::to_bytes(data)));
        let mut iter = 0;
        let numOfbuffers= data.len()/300 + 1 ;
        while iter < (numOfbuffers-1) {
            string = data.slice(iter * 300 ,(iter +1) * 300).to_owned();
            if string.len() == 0 {
                loop;
            }

            if !(*callback)(string.clone(), pw) {
                return false;
            }
            iter += 1;
        }
        string = data.slice(iter * 300, data.len()).to_owned();
        if string.len() > 0 {
            if !(*callback)( string.clone(), pw) {
                return false;
            }   
        }
    }

    if !(*callback)( ~"#", pw) {
        return false;
    }   

    true
}