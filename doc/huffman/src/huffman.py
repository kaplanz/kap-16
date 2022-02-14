#!/usr/bin/env python3
# File:        huffman.py
# Author:      Zakhary Kaplan <https://zakhary.dev>
# Created:     18 Jul 2021
# Version:     0.1.0
# Description: Huffman coding calculator.
# SPDX-License-Identifier: MIT

import argparse
import csv
from dataclasses import dataclass
from heapq import heappop, heappush
import sys
from typing import Any, Dict, List, Optional


def main():
    # Setup parser
    parser = argparse.ArgumentParser()
    # Positional arguments
    parser.add_argument(
        "input",
        type=str,
        metavar="FILE",
        help="input symbols and weights (csv)",
    )
    # Optional arguments
    parser.add_argument(
        "-o",
        "--output",
        type=str,
        metavar="FILE",
        help="output file",
    )
    # Parse args
    args = parser.parse_args()

    # Read the input data
    weights = {}
    with open(args.input, "r") as i:
        reader = csv.reader(i)
        assert next(reader) == ["symbol", "weight"], "Invalid headers in input"
        for row in reader:
            assert len(row) == 2, f"Invalid row in input: {row}"
            s, w = row[0], int(row[1])
            weights[s] = w

    # Construct Huffman coding from input symbols
    huff = Huffman(weights)

    # Extract table of codewords
    codewords = {}
    for symbol in weights.keys():
        codewords[symbol] = huff.codeword(symbol)

    # Open output file
    if args.output:
        o = open(args.output, "w")
    else:
        o = sys.stdout
    # Write codewords to output
    writer = csv.writer(o)
    writer.writerow(["symbol", "weight", "codeword"])
    for symbol, codeword in codewords.items():
        writer.writerow((symbol, weights[symbol], codeword))
    # Close output file
    if args.output:
        o.close()


class Huffman:
    """Huffman code generator."""

    def __init__(self, symbols: Dict[str, int]) -> None:
        self.symbols = symbols
        self.code: BTree = BTree(Data(0, []))
        self.construct()  # construct Huffman coding

    def construct(self):
        # Build a priority queue from the symbols
        pq = []
        for k, v in self.symbols.items():
            heappush(pq, BTree(Data(v, [k])))
        # Construct a binary tree from the heap
        while len(pq) > 1:
            # Pop smaller element to the left
            right = heappop(pq)
            left = heappop(pq)
            # Create new BTree from popped elements
            node = BTree(
                Data(
                    left.data.weight + right.data.weight,
                    left.data.symbols + right.data.symbols,
                )
            )
            # Replace popped elements with new node
            node.left = left
            node.right = right
            heappush(pq, node)
        # Save the BTree as the code
        self.code = pq[0]

    def codeword(self, symbol: str) -> Optional[str]:
        path: List[bool] = []
        cptr = self.code
        # Loop until we've found this symbol's leaf
        while cptr and cptr.left and symbol in cptr.data.symbols:
            path.append(symbol in cptr.left.data.symbols)
            cptr = cptr.left if path[-1] else cptr.right
        # Create the codeword from the path traversed
        return "".join(str(int(b)) for b in path)


class BTree:
    """Simple binary tree implementation."""

    def __init__(self, data: Any) -> None:
        self.data = data
        self.left: Optional[BTree] = None
        self.right: Optional[BTree] = None

    def __lt__(self, other) -> bool:
        return self.data < other.data


@dataclass
class Data:
    """A single Huffman codepoint."""

    weight: int
    symbols: List[str]

    def __lt__(self, other) -> bool:
        return self.weight < other.weight


if __name__ == "__main__":
    main()
