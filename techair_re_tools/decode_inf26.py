#!/usr/bin/env python3

## Copyright (C) 2020, Edward O'Callaghan.
##
## This program is free software; you can redistribute it and/or
## modify it under the terms of the GNU General Public License
## as published by the Free Software Foundation; either version 2
## of the License, or (at your option) any later version.
##
## This program is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
## GNU General Public License for more details.
##
## You should have received a copy of the GNU General Public License
## along with this program; if not, write to the Free Software
## Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

from itertools import zip_longest
import pprint

def decode(seg):
    return [(a - b) % 256 for (a,b) in zip(seg, [0]+list(seg[:15]))][:14]

def grouper(iterable, n, padding=None):
    "Collect data into fixed-length chunks or blocks"
    # grouper('ABCDEFG', 3, 'x') --> ABC DEF Gxx"
    args = [iter(iterable)] * n
    return zip_longest(*args, fillvalue=padding)

def main():
    with open('Inf26.bin','rb') as f:
        content = f.read()
        x = [decode(b) for b in grouper(content, 16)]
        serials = map(lambda x: bytearray(x).decode("utf8"), x)
        pprint.pprint(list(serials))

if __name__ == "__main__":
    main()
