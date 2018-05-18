target extended-remote \\.\com10

set print asm-demangle on

# monitor tpiu config internal itm.fifo uart off 8000000
# monitor itm port 0 on

load
continue
