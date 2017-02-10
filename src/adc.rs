# [ doc = "Analog-to-digital converter" ]
# [ repr ( C ) ]
pub struct Adc {
    # [ doc = "0x00 - status register" ]
    pub sr: Sr,
    # [ doc = "0x04 - control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x08 - control register 2" ]
    pub cr2: Cr2,
    # [ doc = "0x0c - sample time register 1" ]
    pub smpr1: Smpr1,
    # [ doc = "0x10 - sample time register 2" ]
    pub smpr2: Smpr2,
    # [ doc = "0x14 - injected channel data offset register x" ]
    pub jofr1: Jofr1,
    # [ doc = "0x18 - injected channel data offset register x" ]
    pub jofr2: Jofr2,
    # [ doc = "0x1c - injected channel data offset register x" ]
    pub jofr3: Jofr3,
    # [ doc = "0x20 - injected channel data offset register x" ]
    pub jofr4: Jofr4,
    # [ doc = "0x24 - watchdog higher threshold register" ]
    pub htr: Htr,
    # [ doc = "0x28 - watchdog lower threshold register" ]
    pub ltr: Ltr,
    # [ doc = "0x2c - regular sequence register 1" ]
    pub sqr1: Sqr1,
    # [ doc = "0x30 - regular sequence register 2" ]
    pub sqr2: Sqr2,
    # [ doc = "0x34 - regular sequence register 3" ]
    pub sqr3: Sqr3,
    # [ doc = "0x38 - injected sequence register" ]
    pub jsqr: Jsqr,
    # [ doc = "0x3c - injected data register x" ]
    pub jdr1: Jdr1,
    # [ doc = "0x40 - injected data register x" ]
    pub jdr2: Jdr2,
    # [ doc = "0x44 - injected data register x" ]
    pub jdr3: Jdr3,
    # [ doc = "0x48 - injected data register x" ]
    pub jdr4: Jdr4,
    # [ doc = "0x4c - regular data register" ]
    pub dr: Dr,
}

# [ repr ( C ) ]
pub struct Sr {
    register: ::volatile_register::RW<u32>,
}

impl Sr {
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
        where for<'w> F: FnOnce(&SrR, &'w mut SrW) -> &'w mut SrW
    {
        let bits = self.register.read();
        let r = SrR { bits: bits };
        let mut w = SrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SrR {
        SrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SrW) -> &mut SrW
    {
        let mut w = SrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SrR {
    bits: u32,
}

impl SrR {
    # [ doc = "Bit 5 - Overrun" ]
    pub fn ovr(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Regular channel start flag" ]
    pub fn strt(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Injected channel start flag" ]
    pub fn jstrt(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected channel end of conversion" ]
    pub fn jeoc(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Regular channel end of conversion" ]
    pub fn eoc(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Analog watchdog flag" ]
    pub fn awd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SrW {
    bits: u32,
}

impl SrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SrW { bits: 0 }
    }
    # [ doc = "Bit 5 - Overrun" ]
    pub fn ovr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Regular channel start flag" ]
    pub fn strt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Injected channel start flag" ]
    pub fn jstrt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Injected channel end of conversion" ]
    pub fn jeoc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Regular channel end of conversion" ]
    pub fn eoc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Analog watchdog flag" ]
    pub fn awd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Cr1 {
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
        where for<'w> F: FnOnce(&Cr1R, &'w mut Cr1W) -> &'w mut Cr1W
    {
        let bits = self.register.read();
        let r = Cr1R { bits: bits };
        let mut w = Cr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr1R {
        Cr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr1W) -> &mut Cr1W
    {
        let mut w = Cr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr1R {
    bits: u32,
}

impl Cr1R {
    # [ doc = "Bit 26 - Overrun interrupt enable" ]
    pub fn ovrie(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 24:25 - Resolution" ]
    pub fn res(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 23 - Analog watchdog enable on regular channels" ]
    pub fn awden(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Analog watchdog enable on injected channels" ]
    pub fn jawden(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:15 - Discontinuous mode channel count" ]
    pub fn discnum(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 12 - Discontinuous mode on injected channels" ]
    pub fn jdiscen(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Discontinuous mode on regular channels" ]
    pub fn discen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Automatic injected group conversion" ]
    pub fn jauto(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Enable the watchdog on a single channel in scan mode" ]
    pub fn awdsgl(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Scan mode" ]
    pub fn scan(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Interrupt enable for injected channels" ]
    pub fn jeocie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Analog watchdog interrupt enable" ]
    pub fn awdie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Interrupt enable for EOC" ]
    pub fn eocie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:4 - Analog watchdog channel select bits" ]
    pub fn awdch(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr1W {
    bits: u32,
}

impl Cr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cr1W { bits: 0 }
    }
    # [ doc = "Bit 26 - Overrun interrupt enable" ]
    pub fn ovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 24:25 - Resolution" ]
    pub fn res(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Analog watchdog enable on regular channels" ]
    pub fn awden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Analog watchdog enable on injected channels" ]
    pub fn jawden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:15 - Discontinuous mode channel count" ]
    pub fn discnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 12 - Discontinuous mode on injected channels" ]
    pub fn jdiscen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Discontinuous mode on regular channels" ]
    pub fn discen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Automatic injected group conversion" ]
    pub fn jauto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Enable the watchdog on a single channel in scan mode" ]
    pub fn awdsgl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Scan mode" ]
    pub fn scan(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Interrupt enable for injected channels" ]
    pub fn jeocie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Analog watchdog interrupt enable" ]
    pub fn awdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Interrupt enable for EOC" ]
    pub fn eocie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:4 - Analog watchdog channel select bits" ]
    pub fn awdch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cr2 {
    register: ::volatile_register::RW<u32>,
}

impl Cr2 {
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
        where for<'w> F: FnOnce(&Cr2R, &'w mut Cr2W) -> &'w mut Cr2W
    {
        let bits = self.register.read();
        let r = Cr2R { bits: bits };
        let mut w = Cr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr2R {
        Cr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr2W) -> &mut Cr2W
    {
        let mut w = Cr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr2R {
    bits: u32,
}

impl Cr2R {
    # [ doc = "Bit 30 - Start conversion of regular channels" ]
    pub fn swstart(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 28:29 - External trigger enable for regular channels" ]
    pub fn exten(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - External event select for regular group" ]
    pub fn extsel(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 22 - Start conversion of injected channels" ]
    pub fn jswstart(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 20:21 - External trigger enable for injected channels" ]
    pub fn jexten(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - External event select for injected group" ]
    pub fn jextsel(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Data alignment" ]
    pub fn align(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - End of conversion selection" ]
    pub fn eocs(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - DMA disable selection (for single ADC mode)" ]
    pub fn dds(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Direct memory access mode (for single ADC mode)" ]
    pub fn dma(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Continuous conversion" ]
    pub fn cont(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - A/D Converter ON / OFF" ]
    pub fn adon(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr2W {
    bits: u32,
}

impl Cr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cr2W { bits: 0 }
    }
    # [ doc = "Bit 30 - Start conversion of regular channels" ]
    pub fn swstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 28:29 - External trigger enable for regular channels" ]
    pub fn exten(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - External event select for regular group" ]
    pub fn extsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 22 - Start conversion of injected channels" ]
    pub fn jswstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 20:21 - External trigger enable for injected channels" ]
    pub fn jexten(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - External event select for injected group" ]
    pub fn jextsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Data alignment" ]
    pub fn align(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - End of conversion selection" ]
    pub fn eocs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - DMA disable selection (for single ADC mode)" ]
    pub fn dds(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Direct memory access mode (for single ADC mode)" ]
    pub fn dma(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Continuous conversion" ]
    pub fn cont(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - A/D Converter ON / OFF" ]
    pub fn adon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Smpr1 {
    register: ::volatile_register::RW<u32>,
}

impl Smpr1 {
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
        where for<'w> F: FnOnce(&Smpr1R, &'w mut Smpr1W) -> &'w mut Smpr1W
    {
        let bits = self.register.read();
        let r = Smpr1R { bits: bits };
        let mut w = Smpr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Smpr1R {
        Smpr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Smpr1W) -> &mut Smpr1W
    {
        let mut w = Smpr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Smpr1R {
    bits: u32,
}

impl Smpr1R {
    # [ doc = "Bits 0:31 - Sample time bits" ]
    pub fn smpx_x(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Smpr1W {
    bits: u32,
}

impl Smpr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Smpr1W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - Sample time bits" ]
    pub fn smpx_x(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Smpr2 {
    register: ::volatile_register::RW<u32>,
}

impl Smpr2 {
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
        where for<'w> F: FnOnce(&Smpr2R, &'w mut Smpr2W) -> &'w mut Smpr2W
    {
        let bits = self.register.read();
        let r = Smpr2R { bits: bits };
        let mut w = Smpr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Smpr2R {
        Smpr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Smpr2W) -> &mut Smpr2W
    {
        let mut w = Smpr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Smpr2R {
    bits: u32,
}

impl Smpr2R {
    # [ doc = "Bits 0:31 - Sample time bits" ]
    pub fn smpx_x(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Smpr2W {
    bits: u32,
}

impl Smpr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Smpr2W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - Sample time bits" ]
    pub fn smpx_x(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jofr1 {
    register: ::volatile_register::RW<u32>,
}

impl Jofr1 {
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
        where for<'w> F: FnOnce(&Jofr1R, &'w mut Jofr1W) -> &'w mut Jofr1W
    {
        let bits = self.register.read();
        let r = Jofr1R { bits: bits };
        let mut w = Jofr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Jofr1R {
        Jofr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Jofr1W) -> &mut Jofr1W
    {
        let mut w = Jofr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jofr1R {
    bits: u32,
}

impl Jofr1R {
    # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
    pub fn joffset1(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jofr1W {
    bits: u32,
}

impl Jofr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Jofr1W { bits: 0 }
    }
    # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
    pub fn joffset1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jofr2 {
    register: ::volatile_register::RW<u32>,
}

impl Jofr2 {
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
        where for<'w> F: FnOnce(&Jofr2R, &'w mut Jofr2W) -> &'w mut Jofr2W
    {
        let bits = self.register.read();
        let r = Jofr2R { bits: bits };
        let mut w = Jofr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Jofr2R {
        Jofr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Jofr2W) -> &mut Jofr2W
    {
        let mut w = Jofr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jofr2R {
    bits: u32,
}

impl Jofr2R {
    # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
    pub fn joffset2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jofr2W {
    bits: u32,
}

impl Jofr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Jofr2W { bits: 0 }
    }
    # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
    pub fn joffset2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jofr3 {
    register: ::volatile_register::RW<u32>,
}

impl Jofr3 {
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
        where for<'w> F: FnOnce(&Jofr3R, &'w mut Jofr3W) -> &'w mut Jofr3W
    {
        let bits = self.register.read();
        let r = Jofr3R { bits: bits };
        let mut w = Jofr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Jofr3R {
        Jofr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Jofr3W) -> &mut Jofr3W
    {
        let mut w = Jofr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jofr3R {
    bits: u32,
}

impl Jofr3R {
    # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
    pub fn joffset3(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jofr3W {
    bits: u32,
}

impl Jofr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Jofr3W { bits: 0 }
    }
    # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
    pub fn joffset3(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jofr4 {
    register: ::volatile_register::RW<u32>,
}

impl Jofr4 {
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
        where for<'w> F: FnOnce(&Jofr4R, &'w mut Jofr4W) -> &'w mut Jofr4W
    {
        let bits = self.register.read();
        let r = Jofr4R { bits: bits };
        let mut w = Jofr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Jofr4R {
        Jofr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Jofr4W) -> &mut Jofr4W
    {
        let mut w = Jofr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jofr4R {
    bits: u32,
}

impl Jofr4R {
    # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
    pub fn joffset4(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jofr4W {
    bits: u32,
}

impl Jofr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Jofr4W { bits: 0 }
    }
    # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
    pub fn joffset4(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Htr {
    register: ::volatile_register::RW<u32>,
}

impl Htr {
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
        where for<'w> F: FnOnce(&HtrR, &'w mut HtrW) -> &'w mut HtrW
    {
        let bits = self.register.read();
        let r = HtrR { bits: bits };
        let mut w = HtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> HtrR {
        HtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut HtrW) -> &mut HtrW
    {
        let mut w = HtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HtrR {
    bits: u32,
}

impl HtrR {
    # [ doc = "Bits 0:11 - Analog watchdog higher threshold" ]
    pub fn ht(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HtrW {
    bits: u32,
}

impl HtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        HtrW { bits: 4095 }
    }
    # [ doc = "Bits 0:11 - Analog watchdog higher threshold" ]
    pub fn ht(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ltr {
    register: ::volatile_register::RW<u32>,
}

impl Ltr {
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
        where for<'w> F: FnOnce(&LtrR, &'w mut LtrW) -> &'w mut LtrW
    {
        let bits = self.register.read();
        let r = LtrR { bits: bits };
        let mut w = LtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> LtrR {
        LtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut LtrW) -> &mut LtrW
    {
        let mut w = LtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct LtrR {
    bits: u32,
}

impl LtrR {
    # [ doc = "Bits 0:11 - Analog watchdog lower threshold" ]
    pub fn lt(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct LtrW {
    bits: u32,
}

impl LtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        LtrW { bits: 0 }
    }
    # [ doc = "Bits 0:11 - Analog watchdog lower threshold" ]
    pub fn lt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sqr1 {
    register: ::volatile_register::RW<u32>,
}

impl Sqr1 {
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
        where for<'w> F: FnOnce(&Sqr1R, &'w mut Sqr1W) -> &'w mut Sqr1W
    {
        let bits = self.register.read();
        let r = Sqr1R { bits: bits };
        let mut w = Sqr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sqr1R {
        Sqr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sqr1W) -> &mut Sqr1W
    {
        let mut w = Sqr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr1R {
    bits: u32,
}

impl Sqr1R {
    # [ doc = "Bits 20:23 - Regular channel sequence length" ]
    pub fn l(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 15:19 - 16th conversion in regular sequence" ]
    pub fn sq16(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:14 - 15th conversion in regular sequence" ]
    pub fn sq15(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 5:9 - 14th conversion in regular sequence" ]
    pub fn sq14(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - 13th conversion in regular sequence" ]
    pub fn sq13(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr1W {
    bits: u32,
}

impl Sqr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sqr1W { bits: 0 }
    }
    # [ doc = "Bits 20:23 - Regular channel sequence length" ]
    pub fn l(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 15:19 - 16th conversion in regular sequence" ]
    pub fn sq16(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 15u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:14 - 15th conversion in regular sequence" ]
    pub fn sq15(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 5:9 - 14th conversion in regular sequence" ]
    pub fn sq14(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - 13th conversion in regular sequence" ]
    pub fn sq13(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sqr2 {
    register: ::volatile_register::RW<u32>,
}

impl Sqr2 {
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
        where for<'w> F: FnOnce(&Sqr2R, &'w mut Sqr2W) -> &'w mut Sqr2W
    {
        let bits = self.register.read();
        let r = Sqr2R { bits: bits };
        let mut w = Sqr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sqr2R {
        Sqr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sqr2W) -> &mut Sqr2W
    {
        let mut w = Sqr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr2R {
    bits: u32,
}

impl Sqr2R {
    # [ doc = "Bits 25:29 - 12th conversion in regular sequence" ]
    pub fn sq12(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:24 - 11th conversion in regular sequence" ]
    pub fn sq11(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 15:19 - 10th conversion in regular sequence" ]
    pub fn sq10(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:14 - 9th conversion in regular sequence" ]
    pub fn sq9(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 5:9 - 8th conversion in regular sequence" ]
    pub fn sq8(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - 7th conversion in regular sequence" ]
    pub fn sq7(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr2W {
    bits: u32,
}

impl Sqr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sqr2W { bits: 0 }
    }
    # [ doc = "Bits 25:29 - 12th conversion in regular sequence" ]
    pub fn sq12(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:24 - 11th conversion in regular sequence" ]
    pub fn sq11(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 15:19 - 10th conversion in regular sequence" ]
    pub fn sq10(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 15u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:14 - 9th conversion in regular sequence" ]
    pub fn sq9(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 5:9 - 8th conversion in regular sequence" ]
    pub fn sq8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - 7th conversion in regular sequence" ]
    pub fn sq7(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sqr3 {
    register: ::volatile_register::RW<u32>,
}

impl Sqr3 {
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
        where for<'w> F: FnOnce(&Sqr3R, &'w mut Sqr3W) -> &'w mut Sqr3W
    {
        let bits = self.register.read();
        let r = Sqr3R { bits: bits };
        let mut w = Sqr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sqr3R {
        Sqr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sqr3W) -> &mut Sqr3W
    {
        let mut w = Sqr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr3R {
    bits: u32,
}

impl Sqr3R {
    # [ doc = "Bits 25:29 - 6th conversion in regular sequence" ]
    pub fn sq6(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:24 - 5th conversion in regular sequence" ]
    pub fn sq5(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 15:19 - 4th conversion in regular sequence" ]
    pub fn sq4(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:14 - 3rd conversion in regular sequence" ]
    pub fn sq3(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 5:9 - 2nd conversion in regular sequence" ]
    pub fn sq2(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - 1st conversion in regular sequence" ]
    pub fn sq1(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr3W {
    bits: u32,
}

impl Sqr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sqr3W { bits: 0 }
    }
    # [ doc = "Bits 25:29 - 6th conversion in regular sequence" ]
    pub fn sq6(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:24 - 5th conversion in regular sequence" ]
    pub fn sq5(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 15:19 - 4th conversion in regular sequence" ]
    pub fn sq4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 15u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:14 - 3rd conversion in regular sequence" ]
    pub fn sq3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 5:9 - 2nd conversion in regular sequence" ]
    pub fn sq2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - 1st conversion in regular sequence" ]
    pub fn sq1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jsqr {
    register: ::volatile_register::RW<u32>,
}

impl Jsqr {
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
        where for<'w> F: FnOnce(&JsqrR, &'w mut JsqrW) -> &'w mut JsqrW
    {
        let bits = self.register.read();
        let r = JsqrR { bits: bits };
        let mut w = JsqrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> JsqrR {
        JsqrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut JsqrW) -> &mut JsqrW
    {
        let mut w = JsqrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct JsqrR {
    bits: u32,
}

impl JsqrR {
    # [ doc = "Bits 20:21 - Injected sequence length" ]
    pub fn jl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 15:19 - 4th conversion in injected sequence" ]
    pub fn jsq4(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:14 - 3rd conversion in injected sequence" ]
    pub fn jsq3(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 5:9 - 2nd conversion in injected sequence" ]
    pub fn jsq2(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - 1st conversion in injected sequence" ]
    pub fn jsq1(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct JsqrW {
    bits: u32,
}

impl JsqrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        JsqrW { bits: 0 }
    }
    # [ doc = "Bits 20:21 - Injected sequence length" ]
    pub fn jl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 15:19 - 4th conversion in injected sequence" ]
    pub fn jsq4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 15u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:14 - 3rd conversion in injected sequence" ]
    pub fn jsq3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 5:9 - 2nd conversion in injected sequence" ]
    pub fn jsq2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - 1st conversion in injected sequence" ]
    pub fn jsq1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jdr1 {
    register: ::volatile_register::RO<u32>,
}

impl Jdr1 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> Jdr1R {
        Jdr1R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr1R {
    bits: u32,
}

impl Jdr1R {
    # [ doc = "Bits 0:15 - Injected data" ]
    pub fn jdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct Jdr2 {
    register: ::volatile_register::RO<u32>,
}

impl Jdr2 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> Jdr2R {
        Jdr2R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr2R {
    bits: u32,
}

impl Jdr2R {
    # [ doc = "Bits 0:15 - Injected data" ]
    pub fn jdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct Jdr3 {
    register: ::volatile_register::RO<u32>,
}

impl Jdr3 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> Jdr3R {
        Jdr3R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr3R {
    bits: u32,
}

impl Jdr3R {
    # [ doc = "Bits 0:15 - Injected data" ]
    pub fn jdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct Jdr4 {
    register: ::volatile_register::RO<u32>,
}

impl Jdr4 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> Jdr4R {
        Jdr4R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr4R {
    bits: u32,
}

impl Jdr4R {
    # [ doc = "Bits 0:15 - Injected data" ]
    pub fn jdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct Dr {
    register: ::volatile_register::RO<u32>,
}

impl Dr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> DrR {
        DrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DrR {
    bits: u32,
}

impl DrR {
    # [ doc = "Bits 0:15 - Regular data" ]
    pub fn data(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}
