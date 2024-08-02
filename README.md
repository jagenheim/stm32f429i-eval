stm32f429i-eval
===============

_stm32f429i-eval_ contains a basic board support package for the
[STM32F429I-EVAL][] microcontroller board to write firmwares using the Rust
language. This experimentation board features a lot of different things.
It also contains a (non-removable) capable ST-Link V2 debugging interface.

Most of this board is copied from the [discovery board package][]

Also the [stm bsp][] was used as a reference as there were no datasheet available for some
of the things on the eval board.

[STM32F429I-EVAL]: https://www.st.com/en/evaluation-tools/stm32429i-eval.html
[discovery board package]: https://github.com/stm32-rs/stm32f429i-disc
[stm bsp]: https://github.com/STMicroelectronics/stm324x9i-eval-bsp/tree/8a5f75c82c205453715cbd71d6d6a655aac85041


License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
