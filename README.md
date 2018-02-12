# D7018 Task 5

## Implementation of three different tasks to be run on the STM32F401 microprocessor, Nucleo-64 development board.

* A periodic task executing each Xms (free of accumulated drift, and with minimal jitter), that blinks the on-board LED.
* A USART task receiving commands (pause, start, period 1-1000ms), received commands should be parsed and corresponding responses generated and sent over the USART. (Come up with a nice and simple user interface.)
* A logging task, run each second (period 1s), that prints statistics of CPU usage over the ITM port
* Idle should gather statics on sleep/up time, (there is a sleep counter in the cortex core) 
* Use shared resources (data structures) to ensure race free execution
