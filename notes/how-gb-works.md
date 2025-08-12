# First, how does an emulator work, generally?

It's a loop. Fetch, decode, execute stages (with anything else more complicated
fitting into/around these stages).

CPU is the center, and can communicate with other components over a bus.
- Q: is constructing a bus in software truly the best option-- that is, create
a section of memory that each of our "devices" can read from at the right time?
    - thoughts: This could be done with a rust struct that can be borrowed by
    each device when it needs to be accessed.
- Q: If multiple "devices" (e.g. CPU, audio processor, input device(s)) need to
  be connected to the bus simultaneously, how independent are their operations?
  Can each device run on its own thread and have their bus accesses
  synchronized? Or should there be some single-threading of some of the
  operation? Is there perhaps a centralized clock that tells each device to
  perform a cycle?
- Q: It seems hard, and largely unnecessary given the single-core speed of
  modern processors, to write a truly parallel emulator for a simple system.
  So, what tradeoffs are there when serializing the execution of multiple stages
  and/or devices in a system? For example, what considerations should be made
  when the CPU processes data, then sends it to the screen buffer to be
  rendered? What order do things happen, and how much does this order matter?

We can think of the emulator following this basic formula:

```
    while (!stop_emulation)
    {
        executeCPU(cycles_to_execute);
        generateInterrupts();
        emulateGraphics();
        emulateSound();
        emulateOtherSoftware();
        timeSyncronization();
    }
```

This method is the basic one, where we merely interpret the instructions and
emulate the behavior of the CPU in software. There are other approaches, where
we can dynamically convert the instructions into code for the host CPU, but this
is more complicated, and truly uncessary for this project. Perhaps we can
revisit this method in an emulator for a more complicated system or one designed
to run on lower-power hardware (e.g., writing a GB emulator for a DS or 3DS).
For now, we will only focus on the simple interpreter method.

However, there are optimizations we can begin to think about for this method.
For reused code paths (loops), we can store information about the decoded
instructions rather than decoding every instructions from scratch every time.


# How does the GameBoy system architecture work?

This is a summary of the basic system architecture of the GameBoy, including all
components required to run a game correctly.

## CPU

### Registers
CPU registers are organized into pairs of 8-bit registers (16 bits total).
They can be accessed individually or together as if they were a single 16-bit
register. The stack pointer (SP) and program counter (PC) cannot be separated
and are each individually a single 16-bit register.

The common names of the registers are as follows:
```
A F
B C
D E
H L
SP
PC
```
Each one can be described like so:
- A: The accumulator register. This is the only register that can be used to
  perform arithmethic operations.
- F: The flags register. Only the high nibble of this register is used, and
  even then, only two of the bits are really used in practice. The
  layout is like so:
  - bit 7: zero flag (Z)
  - bit 6: Subtraction flag for BCD (N)
  - bit 5: Half-carry flag for BCD (H)
  - bit 4: Carry flag (C)
  - bits 3-0: unused.

  The important flags are Z and C, the zero and carry flags.

- B, C, D, E, H, L: General purpose CPU registers. These can be accessed
  individually or as BC, DE, or HL pairs.

- SP: The stack pointer. This is a 16-bit register which stores the address of
  the top of the stack.

- PC: The program counter. This is a 16-bit register which stores the address of
  the next instruction to be executed.
