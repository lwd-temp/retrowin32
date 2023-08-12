# This code is the target for the 32->64bit transition
# and calls a 64-bit address given on the stack then
# does a far return (back to 32-bit mode).
#
# We embed a single copy of this function verbatim,
# calling it from each instance of the trampoline.

call64:
    # stack contents:
    #   0   32-bit return address
    #   4   32-bit segment selector
    #   8   64-bit target addr (8 bytes)
    #   16  exe return address
    #   20... argn passed from exe

    callq *8(%esp)

    # We want 'far ret', https://www.felixcloutier.com/x86/ret opcode cb
    # It is only available in Clang if we use att syntax(!)
    # Note that it pops 32-bit CS/EIP, even though we are in 64-bit mode.
    lret $8  # ret to 0:4, clean target addr
