/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[test]
#[fixed_stack_segment]
fn sanity_check() {
    use azure::AzSanityCheck;

    unsafe { AzSanityCheck() };
}
