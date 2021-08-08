#!/usr/bin/env python3

import ctypes
cmod = ctypes.CDLL("cmod.so")

r = cmod.inc(3)
print(r)
r = cmod.inc(3)
print(r)

cmod.dump(b"value");