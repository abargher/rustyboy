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


# How does the GameBoy system architecture work?

This is a summary of the basic system architecture of the GameBoy, including all
components required to run a game correctly.

## CPU
