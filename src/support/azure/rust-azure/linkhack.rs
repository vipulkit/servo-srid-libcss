/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Some crumminess to make sure we link correctly

#[cfg(target_os = "linux")]
#[link_args = "-L. -lazure -lstdc++ -lskia -lfontconfig -lX11"]
#[nolink]
extern { }

#[cfg(target_os = "android")]
#[link_args = "-L. -lazure -lstdc++ -lskia -lexpat -lfontconfig -lEGL"]
#[nolink]
extern { }

#[cfg(target_os = "macos")]
#[link_args = "-L. -lazure -lstdc++ -framework ApplicationServices \
			   -lskia -framework IOSurface -lobjc -framework OpenGL \
			   -framework Foundation -framework QuartzCore"]
#[nolink]
extern { }
