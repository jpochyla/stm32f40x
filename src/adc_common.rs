# [ doc = "Common ADC registers" ]
# [ repr ( C ) ]
pub struct AdcCommon {
    # [ doc = "0x00 - ADC Common status register" ]
    pub csr: Csr,
    # [ doc = "0x04 - ADC common control register" ]
    pub ccr: Ccr,
    # [ doc = "0x08 - ADC common regular data register for dual and triple modes" ]
    pub cdr: Cdr,
}

# [ repr ( C ) ]
pub struct Csr {
    register: ::volatile_register::RO<u32>,
}

impl Csr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> CsrR {
        CsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CsrR {
    bits: u32,
}

impl CsrR {
    # [ doc = "Bit 21 - Overrun flag of ADC3" ]
    pub fn ovr3(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Regular channel Start flag of ADC 3" ]
    pub fn strt3(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Injected channel Start flag of ADC 3" ]
    pub fn jstrt3(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Injected channel end of conversion of ADC 3" ]
    pub fn jeoc3(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - End of conversion of ADC 3" ]
    pub fn eoc3(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Analog watchdog flag of ADC 3" ]
    pub fn awd3(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Overrun flag of ADC 2" ]
    pub fn ovr2(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Regular channel Start flag of ADC 2" ]
    pub fn strt2(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Injected channel Start flag of ADC 2" ]
    pub fn jstrt2(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Injected channel end of conversion of ADC 2" ]
    pub fn jeoc2(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - End of conversion of ADC 2" ]
    pub fn eoc2(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Analog watchdog flag of ADC 2" ]
    pub fn awd2(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Overrun flag of ADC 1" ]
    pub fn ovr1(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Regular channel Start flag of ADC 1" ]
    pub fn strt1(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Injected channel Start flag of ADC 1" ]
    pub fn jstrt1(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected channel end of conversion of ADC 1" ]
    pub fn jeoc1(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - End of conversion of ADC 1" ]
    pub fn eoc1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Analog watchdog flag of ADC 1" ]
    pub fn awd1(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Ccr {
    register: ::volatile_register::RW<u32>,
}

impl Ccr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CcrR, &'w mut CcrW) -> &'w mut CcrW
    {
        let bits = self.register.read();
        let r = CcrR { bits: bits };
        let mut w = CcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CcrR {
        CcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CcrW) -> &mut CcrW
    {
        let mut w = CcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CcrR {
    bits: u32,
}

impl CcrR {
    # [ doc = "Bit 23 - Temperature sensor and VREFINT enable" ]
    pub fn tsvrefe(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - VBAT enable" ]
    pub fn vbate(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - ADC prescaler" ]
    pub fn adcpre(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - Direct memory access mode for multi ADC mode" ]
    pub fn dma(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 13 - DMA disable selection for multi-ADC mode" ]
    pub fn dds(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:11 - Delay between 2 sampling phases" ]
    pub fn delay(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - Multi ADC mode selection" ]
    pub fn mult(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CcrW {
    bits: u32,
}

impl CcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CcrW { bits: 0 }
    }
    # [ doc = "Bit 23 - Temperature sensor and VREFINT enable" ]
    pub fn tsvrefe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - VBAT enable" ]
    pub fn vbate(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - ADC prescaler" ]
    pub fn adcpre(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - Direct memory access mode for multi ADC mode" ]
    pub fn dma(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 13 - DMA disable selection for multi-ADC mode" ]
    pub fn dds(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:11 - Delay between 2 sampling phases" ]
    pub fn delay(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - Multi ADC mode selection" ]
    pub fn mult(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cdr {
    register: ::volatile_register::RO<u32>,
}

impl Cdr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> CdrR {
        CdrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CdrR {
    bits: u32,
}

impl CdrR {
    # [ doc = "Bits 16:31 - 2nd data item of a pair of regular conversions" ]
    pub fn data2(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - 1st data item of a pair of regular conversions" ]
    pub fn data1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}
