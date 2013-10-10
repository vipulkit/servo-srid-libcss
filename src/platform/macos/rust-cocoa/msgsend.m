// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#include <AppKit/AppKit.h>
#include <Foundation/Foundation.h>
#include <objc/message.h>
#include <stdint.h>
#include <stdio.h>

double invoke_msg_double(id theReceiver, SEL theSelector) {
    double (*f)(id self, SEL op, ...) = (void *)objc_msgSend_fpret;
    return f(theReceiver, theSelector);
}

id invoke_msg_id(id theReceiver, SEL theSelector) {
    return objc_msgSend(theReceiver, theSelector);
}

id invoke_msg_id_NSRect(id theReceiver, SEL theSelector, NSRect *rect) {
    return objc_msgSend(theReceiver, theSelector, *rect);
}

id invoke_msg_id_id_id_id_id_id(id theReceiver, SEL theSelector, id a, id b, id c, id d, id e) {
    return objc_msgSend(theReceiver, theSelector, a, b, c, d, e);
}

long invoke_msg_long(id theReceiver, SEL theSelector) {
    return (long)objc_msgSend(theReceiver, theSelector);
}

void invoke_msg_void(id theReceiver, SEL theSelector) {
    objc_msgSend(theReceiver, theSelector);
}

void invoke_msg_void_bool(id theReceiver, SEL theSelector, bool a) {
    objc_msgSend(theReceiver, theSelector, a);
}

void invoke_msg_void_id(id theReceiver, SEL theSelector, id id) {
    objc_msgSend(theReceiver, theSelector, id);
}

