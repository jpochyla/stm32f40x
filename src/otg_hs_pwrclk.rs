# [ doc = "USB on the go high speed" ]
# [ repr ( C ) ]
pub struct OtgHsPwrclk {
    # [ doc = "0x00 - Power and clock gating control register" ]
    pub otg_hs_pcgcr: OtgHsPcgcr,
}

# [ repr ( C ) ]
pub struct OtgHsPcgcr {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsPcgcr {
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
        where for<'w> F: FnOnce(&OtgHsPcgcrR, &'w mut OtgHsPcgcrW) -> &'w mut OtgHsPcgcrW
    {
        let bits = self.register.read();
        let r = OtgHsPcgcrR { bits: bits };
        let mut w = OtgHsPcgcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsPcgcrR {
        OtgHsPcgcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsPcgcrW) -> &mut OtgHsPcgcrW
    {
        let mut w = OtgHsPcgcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsPcgcrR {
    bits: u32,
}

impl OtgHsPcgcrR {
    # [ doc = "Bit 0 - Stop PHY clock" ]
    pub fn stppclk(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Gate HCLK" ]
    pub fn gatehclk(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - PHY suspended" ]
    pub fn physusp(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsPcgcrW {
    bits: u32,
}

impl OtgHsPcgcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsPcgcrW { bits: 0 }
    }
    # [ doc = "Bit 0 - Stop PHY clock" ]
    pub fn stppclk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Gate HCLK" ]
    pub fn gatehclk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - PHY suspended" ]
    pub fn physusp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
