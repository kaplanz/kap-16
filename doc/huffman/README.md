# huffman

In traditional architectures, a fixed-width opcode is used to determine the instruction type.
However, in a limited architecture such as 16-bits, it can be difficult to efficiently utilize the address space.
As fixed-width opcodes both add redundancy and shorten the remaining bits needed by instructions with several options, variable-width opcode encodings are necessary.
Despite the benefits, their creation is a tedious and error prone process when done manually.
To this end, [Huffman codings][huffman-codings] can provide an opportunity to greatly simplify this aspect of the design process.

As an optimal prefix code, Huffman codings are a natural solution to solving variable-width opcodes.
In this architecture, they have been used to determined the opcodes of the basic "core" instructions.

## Usage

To generate Huffman codings, run [`huffman.py`](./src/huffman.py):

```sh
./src/huffman.py data/instr.csv
```

Pass the `--help` flag for more info on running the script.

## Example

Using the example sentence found in [`example.txt`](./data/example.txt), we can extract an optimal encoding.

```
A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED
```

Counting each character yields the frequencies listed in [`example.csv`](./data/example.csv):

```
symbol,weight
_,10
D,10
A,11
E,7
C,2
B,6
```

Finally, we can generate Huffman codings:

```
symbol,weight,codeword
_,10,00
D,10,01
A,11,10
E,7,110
C,2,1110
B,6,1111
```

As we can see, the `codeword` field displays the binary codeword for each character.
Putting it all together, we can encode the sentence:

```
1000011101001000110010011101100111001001000111110010011111011111100010001111110100111001001011111011101000111111001
```

Note that due to the nature of Huffman codings, no extra information on the delimiters of character data is needed;
the encoding guarantees that each character can be decoded simply by following the Huffman tree.
In this example, we can see the first character is an `A`, as no other character's encoding begins with the string `10`.

[huffman-codings]: https://en.wikipedia.org/wiki/Huffman_coding
