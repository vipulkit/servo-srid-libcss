a = {}
a.getOwnPropertyDescriptor = XML;
b = Proxy.create(a)
for (x in this)
try {
  (function() {
    "use strict";
    b[2] = x // don't assert
  })()
} catch (e) {}
