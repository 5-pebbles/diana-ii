# Diana II

A 6-bit minimal instruction set computer designed around using `NOR` as a universal logic gate.

- **byte size:** 6-bits.

- **unique instructions:** 4.


## Instructions

| Binary |      Instruction     |  Description  |
|--------|----------------------|---------------|
|   00   |  `NOR [reg] [reg]`   |  Performs a negated OR on the first register. |
|   01   |  `PC [reg] [reg]`    |  Sets the program counter to the address `[reg, reg]`. |
|   10   |  `LOAD [reg] [reg]`  |  Loads data from the address `[reg, reg]` into `C`. |
|   11   |  `STORE [reg] [reg]` |  Stores the value in `C` at the address `[reg, reg]`. |

> [!Warning]
> This CPU is designed for hardware simplicity; your software problems are exactly that: **your** software problems.


**Layout:**

Each instruction is 6 bits in the format `[XX][YY][ZZ]`:

- **X:** 2-bit instruction identifier.
- **Y:** 2-bit first register identifier.
- **Z:** 2-bit second register identifier.


## Registers

| Binary | Name | Description |
|--------|------|-------------|
| **00** |   A  | General purpose register. |
| **01** |   B  | General purpose register. |
| **10** |   C  | General purpose register. |
| **11** |   -  | Read next instruction as a value. |


## Memory Layout

There are a total of 4096 unique address each containing 6 bits.

|    Address     |  Description  |
|----------------|---------------|
| `0x000..0xF3D` | General purpose RAM. |
| `0xF3E..0xF3F` | Program Counter(PC) (ROM). |
| `0xF80..0xFBF` | Left shift lookup table (ROM). |
| `0xFC0..0xFFF` | Right shift lookup table (ROM). |


## Language

Truth table for NOR:

| p | q | NOR |
|---|---|-----|
| 1 | 1 |  0  |
| 1 | 0 |  0  |
| 0 | 1 |  0  |
| 0 | 0 |  1  |

Due to the limited nature of this CPU, it does not make sense to write raw machine code...
I have instead written a compiled language with an expended instruction set; below is a list of instructions and their machine code equivalents:


```DII
! This is an example of a comment in the DIANA-II (extension: .DII) compiled language.
PC 0x0 0x0
! Loop... Loop... Loop.. Loop..
```

> [!Note]
> Instructions and regesters are uppercase because my 6-bit character encoding does not support lowercase...


<details>
  <summary><b><code> MOVE A 0xF </code></b></summary>
&nbsp;

The `MOVE` instruction is used to copy a value from the second register or immediate value to the first register.


**This can be done with:**

1. `NOR A 11-11-11`

2. `NOR A 0xF`

3. `NOR A A`

**Example:**
```
00-00-11
11-11-11
00-00-11
00-11-11
00-00-00
```

</details>


<details>
  <summary><b><code> RWRAP A </code></b></summary>
&nbsp;

This instruction shifts and wraps the bits in the provided register one position to the right; the resulting value is stored in `C`.

> **Note:** This is done using the lookup table at `0xFC0..0xFFF`.

**This can be done with:**

1. `LOAD 11-11-11 A`

**Example:**
```
10-11-00
11-11-11
```

</details>


<details>
  <summary><b><code> RSHIFT A </code></b></summary>
&nbsp;

This instruction shifts the bits in the provided register one position to the right, filling the leftmost bit with 0; the result is stored in `C`.

> **Note:** This is done using the lookup table at `0xFC0..0xFFF`.

**This can be done with:**

1. `LOAD 11-11-11 A`

2. `NOR C C`

3. `NOR C 10-00-00`

**Example:**
```
10-11-00
11-11-11
00-10-10
00-10-11
10-00-00
```

</details>


<details>
  <summary><b><code> TEMPLATE </code></b></summary>
&nbsp;

This is not an instruction, just a template for all other definitions.


**This can be done with:**

1. `NAME [reg] [reg]`

**Example:**
```
00-00-10
```

</details>
