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
> Negated OR is the only logic gate because fuck you...


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
