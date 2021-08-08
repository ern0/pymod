#!/usr/bin/env python3

import ctypes
rsmod = ctypes.CDLL("rsmod.so")

r = rsmod.inc(3)
print(r)
r = rsmod.inc(3)
print(r)

rsmod.dump(b"value");