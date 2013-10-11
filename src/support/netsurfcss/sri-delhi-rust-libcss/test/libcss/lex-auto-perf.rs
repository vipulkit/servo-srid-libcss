extern mod std;
extern mod css;
extern mod parserutils ; 

use css::utils::errors::*;
use css::lex::lexer::*;
use css::charset::csdetect::*;

use parserutils::input::inputstream::*;

pub struct line_ctx_lex {
    buf:~[u8],

    exp:~[~[u8]],

    indata:bool,
    inexp:bool
}

fn check_newline(x: &u8) -> bool { *x == ('\n' as u8) }

pub type  line_func = ~extern fn(data:~str , pw:&mut line_ctx_lex) -> bool;

fn token_to_string(token:css_token_type)-> ~str {
    let mut returnString =~"";
    match token {
        CSS_TOKEN_IDENT=>{
            returnString += ~"IDENT:";
        },
        CSS_TOKEN_ATKEYWORD=>{
            returnString += ~"ATKEYWORD:";
        },
        CSS_TOKEN_HASH=>{
            returnString += ~"HASH:";
        },
        CSS_TOKEN_FUNCTION=>{
            returnString += ~"FUNCTION:";
        }, 
        CSS_TOKEN_STRING=>{
            returnString += ~"STRING:";
        }, 
        CSS_TOKEN_INVALID_STRING=>{
            returnString += ~"INVALID:";
        }, 
        CSS_TOKEN_URI=>{
            returnString += ~"URI:";
        }, 
        CSS_TOKEN_UNICODE_RANGE=>{
            returnString += ~"UNICODE-RANGE:";
        }, 
        CSS_TOKEN_CHAR=>{
            returnString += ~"CHAR:";
        },
        CSS_TOKEN_NUMBER=>{
            returnString += ~"NUMBER:";
        }, 
        CSS_TOKEN_PERCENTAGE=>{
            returnString += ~"PERCENTAGE:";
        }, 
        CSS_TOKEN_DIMENSION=>{
            returnString += ~"DIMENSION:";
        },
        CSS_TOKEN_CDO=>{
            returnString += ~"CDO";
        }, 
        CSS_TOKEN_CDC=>{
            returnString += ~"CDC";
        }, 
        CSS_TOKEN_S=>{
            returnString += ~"S";
        },
        CSS_TOKEN_COMMENT => {
            returnString += ~"COMMENT";
        },
        CSS_TOKEN_INCLUDES => {
            returnString += ~"INCLUDES";
        },
        CSS_TOKEN_DASHMATCH => {
            returnString += ~"DASHMATCH";
        },
        CSS_TOKEN_PREFIXMATCH => {
            returnString += ~"PREFIXMATCH";
        },
        CSS_TOKEN_SUFFIXMATCH => {
            returnString += ~"SUFFIXMATCH";
        },
        CSS_TOKEN_SUBSTRINGMATCH => {
            returnString += ~"SUBSTRINGMATCH";
        }
        CSS_TOKEN_EOF =>{
            returnString += ~"EOF";
        }
    }
    return returnString;   
}


fn match_vec_u8(expected_data: &[u8] , found_string: &str) -> bool {

    let mut found_string_vector = str::to_bytes(found_string);
    if found_string_vector.len() != expected_data.len() {
        // debug!("lenghts don't match");
        return false;
    }

    for vec::each2(expected_data , found_string_vector) |&e , &f| {
        if e != f {
            return false;
        }
    } 
    true
}

pub fn handle_line(args: ~[u8],  ctx:@mut line_ctx_lex)->bool
{
    let mut data : ~[u8] = args ;
    // unsafe{debug!(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
    if  (data.len() == 0) {
        // debug!("error");
        return true;
    }

    if (data[0] == '#' as u8) {
        if (ctx.inexp) {
            /* This marks end of testcase, so run it */

            run_test(copy ctx.buf , copy ctx.exp);
            ctx.exp= ~[];
            ctx.buf=~[];
        }
        if (ctx.indata && match_vec_u8(data, &"#expected")) {
            ctx.indata = false;
            ctx.inexp = true;
        } else if (!ctx.indata) {
            ctx.indata = match_vec_u8(data,&"#data");
            ctx.inexp  = match_vec_u8(data,&"#expected");
        } else {
            ctx.buf += copy data;
            ctx.buf.push('\n' as u8);
        }
    }
    else {
        if ctx.indata {
            ctx.buf += copy data;
            ctx.buf.push('\n' as u8);
            
        }
        if (ctx.inexp) {

            let mut unescaped_data = ~[];
            let mut counter = 1;

            while (counter <= data.len()) {
                if (data[counter -1] == 92) && (data[counter] == 110) {
                    unescaped_data.push(10 as u8);
                    counter += 2;
                }
                else if (data[counter -1] == 92) && (data[counter] == 116) {
                    unescaped_data.push(9 as u8);
                    counter += 2;
                } 
                else if (data[counter -1] == 92) && (data[counter] == 92) {
                    unescaped_data.push(92 as u8);
                    counter += 2;
                } 
                else {
                    unescaped_data.push(data[counter - 1]);
                    counter +=1;
                }
            }
            if counter <= data.len() {
                unescaped_data.push(data[counter-1]);    
            }
            
            ctx.exp.push(unescaped_data);
        }
    }

    return true;
}

fn testMain(fileName: ~str) {
    debug!("Entering: testMain, fileName == %?", fileName);
    let ctx: @mut line_ctx_lex = @mut line_ctx_lex
    {
        mut buf:~[],
        mut exp : ~[],

        mut indata:false,
        mut inexp:false
    };

    let file_content_result = io::read_whole_file(&Path(fileName)) ;
    let mut file_content : ~[u8] ;
    match file_content_result {
        Ok(x) => {
            file_content = x ;
        },
        Err(_) => {
            file_content = ~[] ;
            debug!(fmt!("\n Error opening file"));
            assert!(false) ;
        }
    }        
    let mut vec_lines = vec::split(file_content, check_newline) ;

    for vec_lines.each |&each_line| {
        handle_line(each_line,ctx);
    }
    
    if unsafe {copy ctx.buf.len()} > 0 {
        run_test(copy ctx.buf,copy ctx.exp);
    }
}


pub fn run_test(data:~[u8], exp:~[~[u8]]) {
    debug!("Entering: run_test");
    let (inputStreamOption, _)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT as int), Some(~css__charset_extract));

    let inputstream = 
        match(inputStreamOption) {
            Some(x)   => x,
            None        => {
                fail!(~"InputStream is not created, hence lexer can't be initialised");
            }
        };

    let mut start_time = std::time::precise_time_ns();
    let mut lexer = css_lexer::css__lexer_create(inputstream);
    let mut end_time = std::time::precise_time_ns();

    let creation_time = (end_time as float - start_time as float);

    start_time = std::time::precise_time_ns();
    lexer.css__lexer_append_data(data);
    lexer.css__lexer_append_data(~[]);
    end_time = std::time::precise_time_ns();

    let append_time = (end_time as float - start_time as float);
    
    let mut index=0;
    start_time = std::time::precise_time_ns();
    loop {
        debug!("Entering: run_test loop");
        let (error,_)= lexer.css__lexer_get_token();

        match(error)    {
            CSS_OK => {},
            _=>{
                break;
            }
        } // match
        index += 1;

        if (index == exp.len()) {break;}
    }
    end_time = std::time::precise_time_ns();
    let get_token_time = (end_time as float - start_time as float);

    io::println(fmt!("%.3f, %.3f, %.3f", creation_time, append_time, get_token_time));
}


#[test]
fn tests1() {
    debug!("Entering: tests1");
    testMain(~"data/lex/tests1.dat");
}

#[test]
fn tests2() {
    testMain(~"data/lex/tests2.dat");
}

#[test]
fn regression() {
    testMain(~"data/lex/regression.dat");
}
