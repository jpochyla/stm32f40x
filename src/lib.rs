// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Author: Jorge Aparicio <japaricious@gmail.com>
// Author: Brandon Edens <brandonedens@gmail.com>
// Author: Jan Pochyla <jpochyla@gmail.com>
// Author: Pavol Rusnak <stick@gk2.sk>
// Date: 2017-02-12

//! Memory map for STM32F40x microcontrollers

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate volatile_register;

#[allow(missing_docs)]
pub mod adc;
#[allow(missing_docs)]
pub mod adc_common;
#[allow(missing_docs)]
pub mod can;
#[allow(missing_docs)]
pub mod crc;
#[allow(missing_docs)]
pub mod dac;
#[allow(missing_docs)]
pub mod dbg;
#[allow(missing_docs)]
pub mod dcmi;
#[allow(missing_docs)]
pub mod dma;
#[allow(missing_docs)]
pub mod ethernet_dma;
#[allow(missing_docs)]
pub mod ethernet_mac;
#[allow(missing_docs)]
pub mod ethernet_mmc;
#[allow(missing_docs)]
pub mod ethernet_ptp;
#[allow(missing_docs)]
pub mod exti;
#[allow(missing_docs)]
pub mod flash;
#[allow(missing_docs)]
pub mod fsmc;
#[allow(missing_docs)]
pub mod gpio;
#[allow(missing_docs)]
pub mod i2c;
#[allow(missing_docs)]
pub mod i2s_ext;
#[allow(missing_docs)]
pub mod iwdg;
#[allow(missing_docs)]
pub mod lib;
#[allow(missing_docs)]
pub mod nvic;
#[allow(missing_docs)]
pub mod otg_fs_device;
#[allow(missing_docs)]
pub mod otg_fs_global;
#[allow(missing_docs)]
pub mod otg_fs_host;
#[allow(missing_docs)]
pub mod otg_fs_pwrclk;
#[allow(missing_docs)]
pub mod otg_hs_device;
#[allow(missing_docs)]
pub mod otg_hs_global;
#[allow(missing_docs)]
pub mod otg_hs_host;
#[allow(missing_docs)]
pub mod otg_hs_pwrclk;
#[allow(missing_docs)]
pub mod pwr;
#[allow(missing_docs)]
pub mod rcc;
#[allow(missing_docs)]
pub mod rng;
#[allow(missing_docs)]
pub mod rtc;
#[allow(missing_docs)]
pub mod sdio;
#[allow(missing_docs)]
pub mod spi;
#[allow(missing_docs)]
pub mod syscfg;
#[allow(missing_docs)]
pub mod timer_advanced;
#[allow(missing_docs)]
pub mod timer_basic;
#[allow(missing_docs)]
pub mod timer_four_channel_16bit;
#[allow(missing_docs)]
pub mod timer_four_channel_32bit;
#[allow(missing_docs)]
pub mod timer_one_channel;
#[allow(missing_docs)]
pub mod timer_two_channel;
#[allow(missing_docs)]
pub mod uart;
#[allow(missing_docs)]
pub mod usart;
#[allow(missing_docs)]
pub mod wwdg;

use adc::Adc;
use adc_common::AdcCommon;
use can::Can;
use crc::Crc;
use dac::Dac;
use dbg::Dbg;
use dcmi::Dcmi;
use dma::Dma;
use ethernet_dma::EthernetDma;
use ethernet_mac::EthernetMac;
use ethernet_mmc::EthernetMmc;
use ethernet_ptp::EthernetPtp;
use exti::Exti;
use flash::Flash;
use fsmc::Fsmc;
use gpio::Gpio;
use i2c::I2c;
use i2s_ext::I2sExt;
use iwdg::Iwdg;
use nvic::Nvic;
use otg_fs_device::OtgFsDevice;
use otg_fs_global::OtgFsGlobal;
use otg_fs_host::OtgFsHost;
use otg_fs_pwrclk::OtgFsPwrclk;
use otg_hs_device::OtgHsDevice;
use otg_hs_global::OtgHsGlobal;
use otg_hs_host::OtgHsHost;
use otg_hs_pwrclk::OtgHsPwrclk;
use pwr::Pwr;
use rcc::Rcc;
use rng::Rng;
use rtc::Rtc;
use sdio::Sdio;
use spi::Spi;
use syscfg::Syscfg;
use timer_advanced::TimerAdvanced;
use timer_basic::TimerBasic;
use timer_four_channel_16bit::TimerFourChannel16bit;
use timer_four_channel_32bit::TimerFourChannel32bit;
use timer_one_channel::TimerOneChannel;
use timer_two_channel::TimerTwoChannel;
use uart::Uart;
use usart::Usart;
use wwdg::Wwdg;

const ADC1: usize = 0x40012000;
const ADC2: usize = 0x40012100;
const ADC3: usize = 0x40012200;
const C_ADC: usize = 0x40012300;
const CAN1: usize = 0x40006400;
const CAN2: usize = 0x40006800;
const CRC: usize = 0x40023000;
const DAC: usize = 0x40007400;
const DBG: usize = 0xe0042000;
const DCMI: usize = 0x50050000;
const DMA1: usize = 0x40026000;
const DMA2: usize = 0x40026400;
const Ethernet_DMA: usize = 0x40029000;
const Ethernet_MAC: usize = 0x40028000;
const Ethernet_MMC: usize = 0x40028100;
const Ethernet_PTP: usize = 0x40028700;
const EXTI: usize = 0x40013c00;
const FLASH: usize = 0x40023c00;
const FSMC: usize = 0xa0000000;
const GPIOA: usize = 0x40020000;
const GPIOB: usize = 0x40020400;
const GPIOC: usize = 0x40020800;
const GPIOD: usize = 0x40020c00;
const GPIOE: usize = 0x40021000;
const GPIOF: usize = 0x40021400;
const GPIOG: usize = 0x40021800;
const GPIOH: usize = 0x40021c00;
const GPIOI: usize = 0x40022000;
const I2C1: usize = 0x40005400;
const I2C2: usize = 0x40005800;
const I2C3: usize = 0x40005c00;
const I2S2ext: usize = 0x40003400;
const I2S3ext: usize = 0x40004000;
const IWDG: usize = 0x40003000;
const NVIC: usize = 0xe000e000;
const OTG_FS_DEVICE: usize = 0x50000800;
const OTG_FS_GLOBAL: usize = 0x50000000;
const OTG_FS_HOST: usize = 0x50000400;
const OTG_FS_PWRCLK: usize = 0x50000e00;
const OTG_HS_DEVICE: usize = 0x40040800;
const OTG_HS_GLOBAL: usize = 0x40040000;
const OTG_HS_HOST: usize = 0x40040400;
const OTG_HS_PWRCLK: usize = 0x40040e00;
const PWR: usize = 0x40007000;
const RCC: usize = 0x40023800;
const RNG: usize = 0x50060800;
const RTC: usize = 0x40002800;
const SDIO: usize = 0x40012c00;
const SPI1: usize = 0x40013000;
const SPI2: usize = 0x40003800;
const SPI3: usize = 0x40003c00;
const SYSCFG: usize = 0x40013800;
const TIM10: usize = 0x40014400;
const TIM11: usize = 0x40014800;
const TIM12: usize = 0x40001800;
const TIM13: usize = 0x40001c00;
const TIM14: usize = 0x40002000;
const TIM1: usize = 0x40010000;
const TIM2: usize = 0x40000000;
const TIM3: usize = 0x40000400;
const TIM4: usize = 0x40000800;
const TIM5: usize = 0x40000c00;
const TIM6: usize = 0x40001000;
const TIM7: usize = 0x40001400;
const TIM8: usize = 0x40010400;
const TIM9: usize = 0x40014000;
const UART4: usize = 0x40004c00;
const UART5: usize = 0x40005000;
const USART1: usize = 0x40011000;
const USART2: usize = 0x40004400;
const USART3: usize = 0x40004800;
const USART6: usize = 0x40011400;
const WWDG: usize = 0x40002c00;

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}

/// ADC1 register block (&'static)
pub fn adc1() -> &'static Adc {
    unsafe { deref(ADC1) }
}

/// ADC1 register block (&'static_mut)
pub unsafe fn adc1_mut() -> &'static mut Adc {
    deref_mut(ADC1)
}

/// ADC2 register block (&'static)
pub fn adc2() -> &'static Adc {
    unsafe { deref(ADC2) }
}

/// ADC2 register block (&'static_mut)
pub unsafe fn adc2_mut() -> &'static mut Adc {
    deref_mut(ADC2)
}

/// ADC3 register block (&'static)
pub fn adc3() -> &'static Adc {
    unsafe { deref(ADC3) }
}

/// ADC3 register block (&'static_mut)
pub unsafe fn adc3_mut() -> &'static mut Adc {
    deref_mut(ADC3)
}

/// C_ADC register block (&'static)
pub fn c_adc() -> &'static AdcCommon {
    unsafe { deref(C_ADC) }
}

/// C_ADC register block (&'static_mut)
pub unsafe fn c_adc_mut() -> &'static mut AdcCommon {
    deref_mut(C_ADC)
}

/// CAN1 register block (&'static)
pub fn can1() -> &'static Can {
    unsafe { deref(CAN1) }
}

/// CAN1 register block (&'static_mut)
pub unsafe fn can1_mut() -> &'static mut Can {
    deref_mut(CAN1)
}

/// CAN2 register block (&'static)
pub fn can2() -> &'static Can {
    unsafe { deref(CAN2) }
}

/// CAN2 register block (&'static_mut)
pub unsafe fn can2_mut() -> &'static mut Can {
    deref_mut(CAN2)
}

/// CRC register block (&'static)
pub fn crc() -> &'static Crc {
    unsafe { deref(CRC) }
}

/// CRC register block (&'static_mut)
pub unsafe fn crc_mut() -> &'static mut Crc {
    deref_mut(CRC)
}

/// DAC register block (&'static)
pub fn dac() -> &'static Dac {
    unsafe { deref(DAC) }
}

/// DAC register block (&'static_mut)
pub unsafe fn dac_mut() -> &'static mut Dac {
    deref_mut(DAC)
}

/// DBG register block (&'static)
pub fn dbg() -> &'static Dbg {
    unsafe { deref(DBG) }
}

/// DBG register block (&'static_mut)
pub unsafe fn dbg_mut() -> &'static mut Dbg {
    deref_mut(DBG)
}

/// DCMI register block (&'static)
pub fn dcmi() -> &'static Dcmi {
    unsafe { deref(DCMI) }
}

/// DCMI register block (&'static_mut)
pub unsafe fn dcmi_mut() -> &'static mut Dcmi {
    deref_mut(DCMI)
}

/// DMA1 register block (&'static)
pub fn dma1() -> &'static Dma {
    unsafe { deref(DMA1) }
}

/// DMA1 register block (&'static_mut)
pub unsafe fn dma1_mut() -> &'static mut Dma {
    deref_mut(DMA1)
}

/// DMA2 register block (&'static)
pub fn dma2() -> &'static Dma {
    unsafe { deref(DMA2) }
}

/// DMA2 register block (&'static_mut)
pub unsafe fn dma2_mut() -> &'static mut Dma {
    deref_mut(DMA2)
}

/// Ethernet_DMA register block (&'static)
pub fn ethernet_dma() -> &'static EthernetDma {
    unsafe { deref(Ethernet_DMA) }
}

/// Ethernet_DMA register block (&'static_mut)
pub unsafe fn ethernet_dma_mut() -> &'static mut EthernetDma {
    deref_mut(Ethernet_DMA)
}

/// Ethernet_MAC register block (&'static)
pub fn ethernet_mac() -> &'static EthernetMac {
    unsafe { deref(Ethernet_MAC) }
}

/// Ethernet_MAC register block (&'static_mut)
pub unsafe fn ethernet_mac_mut() -> &'static mut EthernetMac {
    deref_mut(Ethernet_MAC)
}

/// Ethernet_MMC register block (&'static)
pub fn ethernet_mmc() -> &'static EthernetMmc {
    unsafe { deref(Ethernet_MMC) }
}

/// Ethernet_MMC register block (&'static_mut)
pub unsafe fn ethernet_mmc_mut() -> &'static mut EthernetMmc {
    deref_mut(Ethernet_MMC)
}

/// Ethernet_PTP register block (&'static)
pub fn ethernet_ptp() -> &'static EthernetPtp {
    unsafe { deref(Ethernet_PTP) }
}

/// Ethernet_PTP register block (&'static_mut)
pub unsafe fn ethernet_ptp_mut() -> &'static mut EthernetPtp {
    deref_mut(Ethernet_PTP)
}

/// EXTI register block (&'static)
pub fn exti() -> &'static Exti {
    unsafe { deref(EXTI) }
}

/// EXTI register block (&'static_mut)
pub unsafe fn exti_mut() -> &'static mut Exti {
    deref_mut(EXTI)
}

/// FLASH register block (&'static)
pub fn flash() -> &'static Flash {
    unsafe { deref(FLASH) }
}

/// FLASH register block (&'static_mut)
pub unsafe fn flash_mut() -> &'static mut Flash {
    deref_mut(FLASH)
}

/// FSMC register block (&'static)
pub fn fsmc() -> &'static Fsmc {
    unsafe { deref(FSMC) }
}

/// FSMC register block (&'static_mut)
pub unsafe fn fsmc_mut() -> &'static mut Fsmc {
    deref_mut(FSMC)
}

/// GPIOA register block (&'static)
pub fn gpioa() -> &'static Gpio {
    unsafe { deref(GPIOA) }
}

/// GPIOA register block (&'static_mut)
pub unsafe fn gpioa_mut() -> &'static mut Gpio {
    deref_mut(GPIOA)
}

/// GPIOB register block (&'static)
pub fn gpiob() -> &'static Gpio {
    unsafe { deref(GPIOB) }
}

/// GPIOB register block (&'static_mut)
pub unsafe fn gpiob_mut() -> &'static mut Gpio {
    deref_mut(GPIOB)
}

/// GPIOC register block (&'static)
pub fn gpioc() -> &'static Gpio {
    unsafe { deref(GPIOC) }
}

/// GPIOC register block (&'static_mut)
pub unsafe fn gpioc_mut() -> &'static mut Gpio {
    deref_mut(GPIOC)
}

/// GPIOD register block (&'static)
pub fn gpiod() -> &'static Gpio {
    unsafe { deref(GPIOD) }
}

/// GPIOD register block (&'static_mut)
pub unsafe fn gpiod_mut() -> &'static mut Gpio {
    deref_mut(GPIOD)
}

/// GPIOE register block (&'static)
pub fn gpioe() -> &'static Gpio {
    unsafe { deref(GPIOE) }
}

/// GPIOE register block (&'static_mut)
pub unsafe fn gpioe_mut() -> &'static mut Gpio {
    deref_mut(GPIOE)
}

/// GPIOF register block (&'static)
pub fn gpiof() -> &'static Gpio {
    unsafe { deref(GPIOF) }
}

/// GPIOF register block (&'static_mut)
pub unsafe fn gpiof_mut() -> &'static mut Gpio {
    deref_mut(GPIOF)
}

/// GPIOG register block (&'static)
pub fn gpiog() -> &'static Gpio {
    unsafe { deref(GPIOG) }
}

/// GPIOG register block (&'static_mut)
pub unsafe fn gpiog_mut() -> &'static mut Gpio {
    deref_mut(GPIOG)
}

/// GPIOH register block (&'static)
pub fn gpioh() -> &'static Gpio {
    unsafe { deref(GPIOH) }
}

/// GPIOH register block (&'static_mut)
pub unsafe fn gpioh_mut() -> &'static mut Gpio {
    deref_mut(GPIOH)
}

/// GPIOI register block (&'static)
pub fn gpioi() -> &'static Gpio {
    unsafe { deref(GPIOI) }
}

/// GPIOI register block (&'static_mut)
pub unsafe fn gpioi_mut() -> &'static mut Gpio {
    deref_mut(GPIOI)
}

/// I2C1 register block (&'static)
pub fn i2c1() -> &'static I2c {
    unsafe { deref(I2C1) }
}

/// I2C1 register block (&'static_mut)
pub unsafe fn i2c1_mut() -> &'static mut I2c {
    deref_mut(I2C1)
}

/// I2C2 register block (&'static)
pub fn i2c2() -> &'static I2c {
    unsafe { deref(I2C2) }
}

/// I2C2 register block (&'static_mut)
pub unsafe fn i2c2_mut() -> &'static mut I2c {
    deref_mut(I2C2)
}

/// I2C3 register block (&'static)
pub fn i2c3() -> &'static I2c {
    unsafe { deref(I2C3) }
}

/// I2C3 register block (&'static_mut)
pub unsafe fn i2c3_mut() -> &'static mut I2c {
    deref_mut(I2C3)
}

/// I2S2ext register block (&'static)
pub fn i2s2ext() -> &'static I2sExt {
    unsafe { deref(I2S2ext) }
}

/// I2S2ext register block (&'static_mut)
pub unsafe fn i2s2ext_mut() -> &'static mut I2sExt {
    deref_mut(I2S2ext)
}

/// I2S3ext register block (&'static)
pub fn i2s3ext() -> &'static I2sExt {
    unsafe { deref(I2S3ext) }
}

/// I2S3ext register block (&'static_mut)
pub unsafe fn i2s3ext_mut() -> &'static mut I2sExt {
    deref_mut(I2S3ext)
}

/// IWDG register block (&'static)
pub fn iwdg() -> &'static Iwdg {
    unsafe { deref(IWDG) }
}

/// IWDG register block (&'static_mut)
pub unsafe fn iwdg_mut() -> &'static mut Iwdg {
    deref_mut(IWDG)
}

/// NVIC register block (&'static)
pub fn nvic() -> &'static Nvic {
    unsafe { deref(NVIC) }
}

/// NVIC register block (&'static_mut)
pub unsafe fn nvic_mut() -> &'static mut Nvic {
    deref_mut(NVIC)
}

/// OTG_FS_DEVICE register block (&'static)
pub fn otg_fs_device() -> &'static OtgFsDevice {
    unsafe { deref(OTG_FS_DEVICE) }
}

/// OTG_FS_DEVICE register block (&'static_mut)
pub unsafe fn otg_fs_device_mut() -> &'static mut OtgFsDevice {
    deref_mut(OTG_FS_DEVICE)
}

/// OTG_FS_GLOBAL register block (&'static)
pub fn otg_fs_global() -> &'static OtgFsGlobal {
    unsafe { deref(OTG_FS_GLOBAL) }
}

/// OTG_FS_GLOBAL register block (&'static_mut)
pub unsafe fn otg_fs_global_mut() -> &'static mut OtgFsGlobal {
    deref_mut(OTG_FS_GLOBAL)
}

/// OTG_FS_HOST register block (&'static)
pub fn otg_fs_host() -> &'static OtgFsHost {
    unsafe { deref(OTG_FS_HOST) }
}

/// OTG_FS_HOST register block (&'static_mut)
pub unsafe fn otg_fs_host_mut() -> &'static mut OtgFsHost {
    deref_mut(OTG_FS_HOST)
}

/// OTG_FS_PWRCLK register block (&'static)
pub fn otg_fs_pwrclk() -> &'static OtgFsPwrclk {
    unsafe { deref(OTG_FS_PWRCLK) }
}

/// OTG_FS_PWRCLK register block (&'static_mut)
pub unsafe fn otg_fs_pwrclk_mut() -> &'static mut OtgFsPwrclk {
    deref_mut(OTG_FS_PWRCLK)
}

/// OTG_HS_DEVICE register block (&'static)
pub fn otg_hs_device() -> &'static OtgHsDevice {
    unsafe { deref(OTG_HS_DEVICE) }
}

/// OTG_HS_DEVICE register block (&'static_mut)
pub unsafe fn otg_hs_device_mut() -> &'static mut OtgHsDevice {
    deref_mut(OTG_HS_DEVICE)
}

/// OTG_HS_GLOBAL register block (&'static)
pub fn otg_hs_global() -> &'static OtgHsGlobal {
    unsafe { deref(OTG_HS_GLOBAL) }
}

/// OTG_HS_GLOBAL register block (&'static_mut)
pub unsafe fn otg_hs_global_mut() -> &'static mut OtgHsGlobal {
    deref_mut(OTG_HS_GLOBAL)
}

/// OTG_HS_HOST register block (&'static)
pub fn otg_hs_host() -> &'static OtgHsHost {
    unsafe { deref(OTG_HS_HOST) }
}

/// OTG_HS_HOST register block (&'static_mut)
pub unsafe fn otg_hs_host_mut() -> &'static mut OtgHsHost {
    deref_mut(OTG_HS_HOST)
}

/// OTG_HS_PWRCLK register block (&'static)
pub fn otg_hs_pwrclk() -> &'static OtgHsPwrclk {
    unsafe { deref(OTG_HS_PWRCLK) }
}

/// OTG_HS_PWRCLK register block (&'static_mut)
pub unsafe fn otg_hs_pwrclk_mut() -> &'static mut OtgHsPwrclk {
    deref_mut(OTG_HS_PWRCLK)
}

/// PWR register block (&'static)
pub fn pwr() -> &'static Pwr {
    unsafe { deref(PWR) }
}

/// PWR register block (&'static_mut)
pub unsafe fn pwr_mut() -> &'static mut Pwr {
    deref_mut(PWR)
}

/// RCC register block (&'static)
pub fn rcc() -> &'static Rcc {
    unsafe { deref(RCC) }
}

/// RCC register block (&'static_mut)
pub unsafe fn rcc_mut() -> &'static mut Rcc {
    deref_mut(RCC)
}

/// RNG register block (&'static)
pub fn rng() -> &'static Rng {
    unsafe { deref(RNG) }
}

/// RNG register block (&'static_mut)
pub unsafe fn rng_mut() -> &'static mut Rng {
    deref_mut(RNG)
}

/// RTC register block (&'static)
pub fn rtc() -> &'static Rtc {
    unsafe { deref(RTC) }
}

/// RTC register block (&'static_mut)
pub unsafe fn rtc_mut() -> &'static mut Rtc {
    deref_mut(RTC)
}

/// SDIO register block (&'static)
pub fn sdio() -> &'static Sdio {
    unsafe { deref(SDIO) }
}

/// SDIO register block (&'static_mut)
pub unsafe fn sdio_mut() -> &'static mut Sdio {
    deref_mut(SDIO)
}

/// SPI1 register block (&'static)
pub fn spi1() -> &'static Spi {
    unsafe { deref(SPI1) }
}

/// SPI1 register block (&'static_mut)
pub unsafe fn spi1_mut() -> &'static mut Spi {
    deref_mut(SPI1)
}

/// SPI2 register block (&'static)
pub fn spi2() -> &'static Spi {
    unsafe { deref(SPI2) }
}

/// SPI2 register block (&'static_mut)
pub unsafe fn spi2_mut() -> &'static mut Spi {
    deref_mut(SPI2)
}

/// SPI3 register block (&'static)
pub fn spi3() -> &'static Spi {
    unsafe { deref(SPI3) }
}

/// SPI3 register block (&'static_mut)
pub unsafe fn spi3_mut() -> &'static mut Spi {
    deref_mut(SPI3)
}

/// SYSCFG register block (&'static)
pub fn syscfg() -> &'static SysCfg {
    unsafe { deref(SYSCFG) }
}

/// SYSCFG register block (&'static_mut)
pub unsafe fn syscfg_mut() -> &'static mut SysCfg {
    deref_mut(SYSCFG)
}

/// TIM1 register block (&'static)
pub fn tim1() -> &'static TimerAdvanced {
    unsafe { deref(TIM1) }
}

/// TIM1 register block (&'static_mut)
pub unsafe fn tim1_mut() -> &'static mut TimerAdvanced {
    deref_mut(TIM1)
}

/// TIM2 register block (&'static)
pub fn tim2() -> &'static TimerFourChannel32bit {
    unsafe { deref(TIM2) }
}

/// TIM2 register block (&'static_mut)
pub unsafe fn tim2_mut() -> &'static mut TimerFourChannel32bit {
    deref_mut(TIM2)
}

/// TIM3 register block (&'static)
pub fn tim3() -> &'static TimerFourChannel16bit {
    unsafe { deref(TIM3) }
}

/// TIM3 register block (&'static_mut)
pub unsafe fn tim3_mut() -> &'static mut TimerFourChannel16bit {
    deref_mut(TIM3)
}

/// TIM4 register block (&'static)
pub fn tim4() -> &'static TimerFourChannel16bit {
    unsafe { deref(TIM4) }
}

/// TIM4 register block (&'static_mut)
pub unsafe fn tim4_mut() -> &'static mut TimerFourChannel16bit {
    deref_mut(TIM4)
}

/// TIM5 register block (&'static)
pub fn tim5() -> &'static TimerFourChannel32bit {
    unsafe { deref(TIM5) }
}

/// TIM5 register block (&'static_mut)
pub unsafe fn tim5_mut() -> &'static mut TimerFourChannel32bit {
    deref_mut(TIM5)
}

/// TIM6 register block (&'static)
pub fn tim6() -> &'static TimerBasic {
    unsafe { deref(TIM6) }
}

/// TIM6 register block (&'static_mut)
pub unsafe fn tim6_mut() -> &'static mut TimerBasic {
    deref_mut(TIM6)
}

/// TIM7 register block (&'static)
pub fn tim7() -> &'static TimerBasic {
    unsafe { deref(TIM7) }
}

/// TIM7 register block (&'static_mut)
pub unsafe fn tim7_mut() -> &'static mut TimerBasic {
    deref_mut(TIM7)
}

/// TIM8 register block (&'static)
pub fn tim8() -> &'static TimerAdvanced {
    unsafe { deref(TIM8) }
}

/// TIM8 register block (&'static_mut)
pub unsafe fn tim8_mut() -> &'static mut TimerAdvanced {
    deref_mut(TIM8)
}

/// TIM9 register block (&'static)
pub fn tim9() -> &'static TimerTwoChannel {
    unsafe { deref(TIM9) }
}

/// TIM9 register block (&'static_mut)
pub unsafe fn tim9_mut() -> &'static mut TimerTwoChannel {
    deref_mut(TIM9)
}

/// TIM10 register block (&'static)
pub fn tim10() -> &'static TimerOneChannel {
    unsafe { deref(TIM10) }
}

/// TIM10 register block (&'static_mut)
pub unsafe fn tim10_mut() -> &'static mut TimerOneChannel {
    deref_mut(TIM10)
}

/// TIM11 register block (&'static)
pub fn tim11() -> &'static TimerOneChannel {
    unsafe { deref(TIM11) }
}

/// TIM11 register block (&'static_mut)
pub unsafe fn tim11_mut() -> &'static mut TimerOneChannel {
    deref_mut(TIM11)
}

/// TIM12 register block (&'static)
pub fn tim12() -> &'static TimerTwoChannel {
    unsafe { deref(TIM12) }
}

/// TIM12 register block (&'static_mut)
pub unsafe fn tim12_mut() -> &'static mut TimerTwoChannel {
    deref_mut(TIM12)
}

/// TIM13 register block (&'static)
pub fn tim13() -> &'static TimerOneChannel {
    unsafe { deref(TIM13) }
}

/// TIM13 register block (&'static_mut)
pub unsafe fn tim13_mut() -> &'static mut TimerOneChannel {
    deref_mut(TIM13)
}

/// TIM14 register block (&'static)
pub fn tim14() -> &'static TimerOneChannel {
    unsafe { deref(TIM14) }
}

/// TIM14 register block (&'static_mut)
pub unsafe fn tim14_mut() -> &'static mut TimerOneChannel {
    deref_mut(TIM14)
}

/// UART4 register block (&'static)
pub fn uart4() -> &'static Uart {
    unsafe { deref(UART4) }
}

/// UART4 register block (&'static_mut)
pub unsafe fn uart4_mut() -> &'static mut Uart {
    deref_mut(UART4)
}

/// UART5 register block (&'static)
pub fn uart5() -> &'static Uart {
    unsafe { deref(UART5) }
}

/// UART5 register block (&'static_mut)
pub unsafe fn uart5_mut() -> &'static mut Uart {
    deref_mut(UART5)
}

/// USART1 register block (&'static)
pub fn usart1() -> &'static Usart {
    unsafe { deref(USART1) }
}

/// USART1 register block (&'static_mut)
pub unsafe fn usart1_mut() -> &'static mut Usart {
    deref_mut(USART1)
}

/// USART2 register block (&'static)
pub fn usart2() -> &'static Usart {
    unsafe { deref(USART2) }
}

/// USART2 register block (&'static_mut)
pub unsafe fn usart2_mut() -> &'static mut Usart {
    deref_mut(USART2)
}

/// USART3 register block (&'static)
pub fn usart3() -> &'static Usart {
    unsafe { deref(USART3) }
}

/// USART3 register block (&'static_mut)
pub unsafe fn usart3_mut() -> &'static mut Usart {
    deref_mut(USART3)
}

/// USART6 register block (&'static)
pub fn usart6() -> &'static Usart {
    unsafe { deref(USART6) }
}

/// USART6 register block (&'static_mut)
pub unsafe fn usart6_mut() -> &'static mut Usart {
    deref_mut(USART6)
}

/// WWDG register block (&'static)
pub fn wwdg() -> &'static Wwdg {
    unsafe { deref(WWDG) }
}

/// WWDG register block (&'static_mut)
pub unsafe fn wwdg_mut() -> &'static mut Wwdg {
    deref_mut(WWDG)
}
