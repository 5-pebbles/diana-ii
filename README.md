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

> [!Note]
> Instructions and registers are uppercase because my 6-bit character encoding does not support lowercase...


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

|     Address     |  Description  |
|-----------------|---------------|
| `0x000..=0xF3D` | General purpose RAM. |
| `0xF3E..=0xF3F` | Program Counter(PC) (ROM). |
| `0xF80..=0xFBF` | Left shift lookup table (ROM). |
| `0xFC0..=0xFFF` | Right shift lookup table (ROM). |


## Language

Due to the limited nature of this CPU, it does not make sense to write raw machine code.

I've written a compiled language to aid development; however, it does not contain any abstractions that could hurt performance.

> [!Warning]
> Many instructions have **side effects** see:&nbsp; _**AND**_, _**NAND**_


**Logic Instructions:**

<details>
  <summary><b><code> NOT B </code></b></summary>
&nbsp;

This instruction flips all bits in the provided register.

| p | NOT |
|---|-----|
| 1 |  0  |
| 0 |  1  |


**This can be done with:**

1. `NOR B B`

**Example:**
```
00-01-01
```

</details>


<details>
  <summary><b><code> AND A B </code></b></summary>
&nbsp;

This instruction performs a logical `AND` on the provided values, storing the result in the first register.

**Warning:** The second register is flipped; its value can be restored with a `NOT` operation. If an immediate value is used, it is flipped at compile time.

| p | q | AND |
|---|---|-----|
| 1 | 1 |  1  |
| 1 | 0 |  0  |
| 0 | 1 |  0  |
| 0 | 0 |  0  |


**This can be done with:**

1. `NOR A A`

2. `NOR B B`

3. `NOR A B`

**Example:**
```
00-00-00
00-01-01
00-00-01
```

</details>


<details>
  <summary><b><code> NAND A B </code></b></summary>
&nbsp;

This instruction performs a logical `NAND` on the provided values, storing the result in the first register.

**Warning:** The second register is flipped; its value can be restored with a `NOT` operation. If an immediate value is used, it is flipped at compile time.

| p | q | AND |
|---|---|-----|
| 1 | 1 |  0  |
| 1 | 0 |  1  |
| 0 | 1 |  1  |
| 0 | 0 |  1  |


**This can be done with:**

1. `NOR A A`

2. `NOR B B`

3. `NOR A B`

4. `NOR A A`

**Example:**
```
00-00-00
00-01-01
00-00-01
00-00-00
```

</details>


<details>
  <summary><b><code> OR A 0x4 </code></b></summary>
&nbsp;

This instruction performs a logical `OR` on the provided values, storing the result in the first register.

| p | q | OR |
|---|---|----|
| 1 | 1 | 1  |
| 1 | 0 | 1  |
| 0 | 1 | 1  |
| 0 | 0 | 0  |


**This can be done with:**

1. `NOR A 0x4`

2. `NOR A A`

**Example:**
```
00-00-11
00-01-00
00-00-00
```

</details>


**Arithmetic Instructions:**

<details>
  <summary><b><code> LWRAP A </code></b></summary>
&nbsp;

This instruction shifts and wraps the bits in the provided register one position to the left; the resulting value is stored in `C`.

> **Note:** This is done using the lookup table at `0xF80..=0xFBF`.

**This can be done with:**

1. `LOAD 11-11-10 A`

**Example:**
```
10-11-00
11-11-10
```

</details>


<details>
  <summary><b><code> RWRAP A </code></b></summary>
&nbsp;

This instruction shifts and wraps the bits in the provided register one position to the right; the resulting value is stored in `C`.

> **Note:** This is done using the lookup table at `0xFC0..=0xFFF`.

**This can be done with:**

1. `LOAD 11-11-11 A`

**Example:**
```
10-11-00
11-11-11
```

</details>


<details>
  <summary><b><code> LSHIFT A </code></b></summary>
&nbsp;

This instruction shifts the bits in the provided register one position to the left, filling the rightmost bit with 0; the result is stored in `C`.

> **Note:** This is done using the lookup table at `0xF80..=0xFBF`.

**This can be done with:**

1. `LOAD 11-11-10 A`

2. `NOR C C`

3. `NOR C 00-00-01`

**Example:**
```
10-11-00
11-11-10
00-10-10
00-10-11
00-00-01
```

</details>


<details>
  <summary><b><code> RSHIFT A </code></b></summary>
&nbsp;

This instruction shifts the bits in the provided register one position to the right, filling the leftmost bit with 0; the result is stored in `C`.

> **Note:** This is done using the lookup table at `0xFC0..=0xFFF`.

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


**Other Instructions:**

<details>
  <summary><b><code> COPY A 0xF </code></b></summary>
&nbsp;

The `COPY` instruction copies a value to the first register from the second register or an immediate value.


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
