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


# How does the GameBoy system architecture work?

This is a summary of the basic system architecture of the GameBoy, including all
components required to run a game correctly.

## CPU
