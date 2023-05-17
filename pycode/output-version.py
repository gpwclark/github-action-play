#!/usr/bin/env python

import requirements
import os
with open('pycode/requirements.txt', 'r') as fd:
     for req in requirements.parse(fd):
         if req.name == "sample":
             print(f"version={req.specs[0][1]}")
