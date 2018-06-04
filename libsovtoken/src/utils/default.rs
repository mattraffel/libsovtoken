/*! 
 * Supplies default data for *TESTS ONLY*
 * 
*/

use indy::ErrorCode;
use libc::c_char;
use utils::ffi_support::c_pointer_from_string;

pub fn inputs_json_pointer() -> *const c_char {
    let json = json!({
        "ver": 1,
        "inputs": [
            {
                "address": "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC",
                "seqNo": 2
            },
            {
                "address": "pay:sov:XuBhXW6gKcUAq6fmyKsdxxjOZEbLy66FEDkQwTPeoXBmTZKy",
                "seqNo": 3
            } 
        ]
    });

    return c_pointer_from_string(json.to_string());
}

pub fn outputs_json_pointer() -> *const c_char {
    let json = json!({
        "ver": 1,
            "ver": 1,
            "outputs": [
                {
                    "address": "pay:sov:ql33nBkjGw6szxPT6LLRUIejn9TZAYkVRPd0QJzfJ8FdhZWs",
                    "amount": 10
                }
            ]
    });

    return c_pointer_from_string(json.to_string());
}

pub extern fn empty_callback_string(
    _: i32,
    e: ErrorCode,
    _: *const c_char
) -> ErrorCode {
    return e;
}
