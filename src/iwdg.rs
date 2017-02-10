# [ doc = "Independent watchdog" ]
# [ repr ( C ) ]
pub struct Iwdg {
    # [ doc = "0x00 - Key register" ]
    pub kr: Kr,
    # [ doc = "0x04 - Prescaler register" ]
    pub pr: Pr,
    # [ doc = "0x08 - Reload register" ]
    pub rlr: Rlr,
    # [ doc = "0x0c - Status register" ]
    pub sr: Sr,
}

# [ repr ( C ) ]
pub struct Kr {
    register: ::volatile_register::WO<u32>,
}

impl Kr {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut KrW) -> &mut KrW
    {
        let mut w = KrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct KrW {
    bits: u32,
}

impl KrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        KrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Key value (write only, read 0000h)" ]
    pub fn key(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pr {
    register: ::volatile_register::RW<u32>,
}

impl Pr {
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
        where for<'w> F: FnOnce(&PrR, &'w mut PrW) -> &'w mut PrW
    {
        let bits = self.register.read();
        let r = PrR { bits: bits };
        let mut w = PrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PrR {
        PrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PrW) -> &mut PrW
    {
        let mut w = PrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PrR {
    bits: u32,
}

impl PrR {
    # [ doc = "Bits 0:2 - Prescaler divider" ]
    pub fn pr(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PrW {
    bits: u32,
}

impl PrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PrW { bits: 0 }
    }
    # [ doc = "Bits 0:2 - Prescaler divider" ]
    pub fn pr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Rlr {
    register: ::volatile_register::RW<u32>,
}

impl Rlr {
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
        where for<'w> F: FnOnce(&RlrR, &'w mut RlrW) -> &'w mut RlrW
    {
        let bits = self.register.read();
        let r = RlrR { bits: bits };
        let mut w = RlrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> RlrR {
        RlrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut RlrW) -> &mut RlrW
    {
        let mut w = RlrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RlrR {
    bits: u32,
}

impl RlrR {
    # [ doc = "Bits 0:11 - Watchdog counter reload value" ]
    pub fn rl(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RlrW {
    bits: u32,
}

impl RlrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        RlrW { bits: 4095 }
    }
    # [ doc = "Bits 0:11 - Watchdog counter reload value" ]
    pub fn rl(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sr {
    register: ::volatile_register::RO<u32>,
}

impl Sr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> SrR {
        SrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SrR {
    bits: u32,
}

impl SrR {
    # [ doc = "Bit 1 - Watchdog counter reload value update" ]
    pub fn rvu(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Watchdog prescaler value update" ]
    pub fn pvu(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}
