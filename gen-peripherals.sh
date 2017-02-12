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
# Author: Pavol Rusnak <stick@gk2.sk>
# Date: 2017-02-12

set -ex

main() {
    local svd=STM32F40x.svd

    if [ ! -f $svd ]; then
        curl -LOs https://github.com/posborne/cmsis-svd/raw/master/data/STMicro/$svd
    fi

    svd2rust -i $svd adc1 | sed 's/\(pub struct Adc\)1/\1/' > src/adc.rs # adc1 + adc2 + adc3

    svd2rust -i $svd c_adc | sed 's/\(pub struct \)CAdc/\1AdcCommon/' > src/adc_common.rs

    svd2rust -i $svd can1 | sed 's/\(pub struct Can\)1/\1/' > src/can.rs # can1 + can2

    svd2rust -i $svd crc > src/crc.rs

    svd2rust -i $svd dac > src/dac.rs

    svd2rust -i $svd dbg > src/dbg.rs

    svd2rust -i $svd dcmi > src/dcmi.rs

    svd2rust -i $svd dma1 | sed 's/\(pub struct Dma\)1/\1/' > src/dma.rs # dma1 + dma2

    svd2rust -i $svd ethernet_dma > src/ethernet_dma.rs

    svd2rust -i $svd ethernet_mac > src/ethernet_mac.rs

    svd2rust -i $svd ethernet_mmc > src/ethernet_mmc.rs

    svd2rust -i $svd ethernet_ptp > src/ethernet_ptp.rs

    svd2rust -i $svd exti > src/exti.rs

    svd2rust -i $svd flash > src/flash.rs

    svd2rust -i $svd fsmc > src/fsmc.rs

    svd2rust -i $svd gpioi | sed 's/\(pub struct Gpio\)i/\1/' > src/gpio.rs # gpioa ... gpioi

    svd2rust -i $svd i2c1 | sed 's/\(pub struct I2c\)1/\1/' > src/i2c.rs # i2c1 ... i2c3

    svd2rust -i $svd i2s2ext | sed 's/\(pub struct \)I2s2ext/\1I2sExt/' > src/i2s_ext.rs # i2s2ext + i2s3ext

    svd2rust -i $svd iwdg > src/iwdg.rs

    svd2rust -i $svd nvic > src/nvic.rs

    svd2rust -i $svd otg_fs_device > src/otg_fs_device.rs

    svd2rust -i $svd otg_fs_global > src/otg_fs_global.rs

    svd2rust -i $svd otg_fs_host > src/otg_fs_host.rs

    svd2rust -i $svd otg_fs_pwrclk > src/otg_fs_pwrclk.rs

    svd2rust -i $svd otg_hs_device > src/otg_hs_device.rs

    svd2rust -i $svd otg_hs_global > src/otg_hs_global.rs

    svd2rust -i $svd otg_hs_host > src/otg_hs_host.rs

    svd2rust -i $svd otg_hs_pwrclk > src/otg_hs_pwrclk.rs

    svd2rust -i $svd pwr > src/pwr.rs

    svd2rust -i $svd rcc > src/rcc.rs

    svd2rust -i $svd rng > src/rng.rs

    svd2rust -i $svd rtc > src/rtc.rs

    svd2rust -i $svd sdio > src/sdio.rs

    svd2rust -i $svd spi1 | sed 's/\(pub struct Spi\)1/\1/' > src/spi.rs # spi1 ... spi3

    svd2rust -i $svd syscfg > src/syscfg.rs

    svd2rust -i $svd tim1 | sed 's/\(pub struct \)Tim1/\1TimerAdvanced/' > src/timer_advanced.rs # tim1 + tim8

    svd2rust -i $svd tim2 | sed 's/\(pub struct \)Tim2/\1TimerFourChannel32bit/' > src/timer_four_channel_32bit.rs # tim2 + tim5

    svd2rust -i $svd tim3 | sed 's/\(pub struct \)Tim3/\1TimerFourChannel16bit/' > src/timer_four_channel_16bit.rs # tim3 + tim4

    svd2rust -i $svd tim6 | sed 's/\(pub struct \)Tim6/\1TimerBasic/' > src/timer_basic.rs # tim6 + tim7

    svd2rust -i $svd tim9 | sed 's/\(pub struct \)Tim9/\1TimerTwoChannel/' > src/timer_two_channel.rs # tim9 + tim12

    svd2rust -i $svd tim10 | sed 's/\(pub struct \)Tim10/\1TimerOneChannel/' > src/timer_one_channel.rs # tim10 + tim11 + tim13 + tim14

    svd2rust -i $svd uart4 | sed 's/\(pub struct Uart\)4/\1/' > src/uart.rs # uart4 + uart5

    svd2rust -i $svd usart1 | sed 's/\(pub struct Usart\)1/\1/' > src/usart.rs # usart1 + usart2 + usart3 + usart6

    svd2rust -i $svd wwdg > src/wwdg.rs

    rustfmt --write-mode overwrite src/*.rs || :

    xargo build --target thumbv7em-none-eabihf
}

main
