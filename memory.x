MEMORY
{
  CCMRAM : ORIGIN = 0x10000000, LENGTH = 64K
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 112K
}

_stack_start = ORIGIN(CCMRAM) + LENGTH(CCMRAM);
