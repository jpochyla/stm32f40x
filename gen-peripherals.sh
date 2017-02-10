#!/bin/sh
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.
#
# Author: Jorge Aparicio <japaricious@gmail.com>
# Author: Brandon Edens <brandonedens@gmail.com>
# Author: Jan Pochyla <jpochyla@gmail.com>
# Date: 2016-11-19

set -ex

main() {
    local svd=STM32F40x.svd

    if [ ! -f $svd ]; then
        curl -LOs https://github.com/posborne/cmsis-svd/raw/master/data/STMicro/$svd
    fi

    svd2rust -i $svd dac > src/dac.rs

    svd2rust -i $svd dma1 > src/dma.rs
    sed -i '' 's/\(pub struct \)Dma1/\1Dma/' src/dma.rs

    svd2rust -i $svd crc > src/crc.rs
    svd2rust -i $svd iwdg > src/iwdg.rs
    svd2rust -i $svd wwdg > src/wwdg.rs

    svd2rust -i $svd i2c1 > src/i2c.rs
    sed -i '' 's/\(pub struct I2c\)1/\1/' src/i2c.rs

    svd2rust -i $svd flash > src/flash.rs
    svd2rust -i $svd rcc > src/rcc.rs
    svd2rust -i $svd pwr > src/pwr.rs
    svd2rust -i $svd syscfg > src/syscfg.rs
    svd2rust -i $svd rng > src/rng.rs

    svd2rust -i $svd adc1 > src/adc.rs
    sed -i '' 's/\(pub struct Adc\)1/\1/' src/adc.rs

    svd2rust -i $svd gpioa > src/gpio.rs
    sed -i '' 's/\(pub struct Gpio\)a/\1/' src/gpio.rs

    svd2rust -i $svd tim6 > src/basic_timer.rs
    sed -i '' 's/\(pub struct \)Tim6/\1BasicTimer/' src/basic_timer.rs

    svd2rust -i $svd tim2 > src/general_timer.rs
    sed -i '' 's/\(pub struct \)Tim2/\1GeneralTimer/' src/general_timer.rs

    svd2rust -i $svd tim1 > src/advanced_timer.rs
    sed -i '' 's/\(pub struct \)Tim1/\1AdvancedTimer/' src/advanced_timer.rs

    svd2rust -i $svd usart1 > src/usart.rs
    sed -i '' 's/\(pub struct \)Usart1/\1Usart/' src/usart.rs

    svd2rust -i $svd spi1 > src/spi.rs
    sed -i '' 's/\(pub struct \)Spi1/\1Spi/' src/spi.rs

    svd2rust -i $svd can > src/can.rs
    sed -i '' 's/\(pub struct \)Can1/\1Can/' src/can.rs

    svd2rust -i $svd exti > src/exti.rs
    svd2rust -i $svd rtc > src/rtc.rs
    svd2rust -i $svd otg_fs_global > src/otg_fs_global.rs
    svd2rust -i $svd otg_fs_host > src/otg_fs_host.rs
    svd2rust -i $svd otg_fs_device > src/otg_fs_device.rs
    svd2rust -i $svd otg_fs_pwrclk > src/otg_fs_pwrclk.rs
    svd2rust -i $svd nvic > src/nvic.rs

    set +e
    rustfmt --write-mode overwrite src/*.rs
    set -e

    xargo build --target thumbv7em-none-eabihf
}

main
