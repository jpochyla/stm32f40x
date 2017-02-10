# [ doc = "System configuration controller" ]
# [ repr ( C ) ]
pub struct Syscfg {
    # [ doc = "0x00 - memory remap register" ]
    pub memrm: Memrm,
    # [ doc = "0x04 - peripheral mode configuration register" ]
    pub pmc: Pmc,
    # [ doc = "0x08 - external interrupt configuration register 1" ]
    pub exticr1: Exticr1,
    # [ doc = "0x0c - external interrupt configuration register 2" ]
    pub exticr2: Exticr2,
    # [ doc = "0x10 - external interrupt configuration register 3" ]
    pub exticr3: Exticr3,
    # [ doc = "0x14 - external interrupt configuration register 4" ]
    pub exticr4: Exticr4,
    _reserved0: [u8; 8usize],
    # [ doc = "0x20 - Compensation cell control register" ]
    pub cmpcr: Cmpcr,
}

# [ repr ( C ) ]
pub struct Memrm {
    register: ::volatile_register::RW<u32>,
}

impl Memrm {
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
        where for<'w> F: FnOnce(&MemrmR, &'w mut MemrmW) -> &'w mut MemrmW
    {
        let bits = self.register.read();
        let r = MemrmR { bits: bits };
        let mut w = MemrmW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MemrmR {
        MemrmR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MemrmW) -> &mut MemrmW
    {
        let mut w = MemrmW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MemrmR {
    bits: u32,
}

impl MemrmR {
    # [ doc = "Bits 0:1 - MEM_MODE" ]
    pub fn mem_mode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MemrmW {
    bits: u32,
}

impl MemrmW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MemrmW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - MEM_MODE" ]
    pub fn mem_mode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pmc {
    register: ::volatile_register::RW<u32>,
}

impl Pmc {
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
        where for<'w> F: FnOnce(&PmcR, &'w mut PmcW) -> &'w mut PmcW
    {
        let bits = self.register.read();
        let r = PmcR { bits: bits };
        let mut w = PmcW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PmcR {
        PmcR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PmcW) -> &mut PmcW
    {
        let mut w = PmcW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PmcR {
    bits: u32,
}

impl PmcR {
    # [ doc = "Bit 23 - Ethernet PHY interface selection" ]
    pub fn mii_rmii_sel(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PmcW {
    bits: u32,
}

impl PmcW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PmcW { bits: 0 }
    }
    # [ doc = "Bit 23 - Ethernet PHY interface selection" ]
    pub fn mii_rmii_sel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Exticr1 {
    register: ::volatile_register::RW<u32>,
}

impl Exticr1 {
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
        where for<'w> F: FnOnce(&Exticr1R, &'w mut Exticr1W) -> &'w mut Exticr1W
    {
        let bits = self.register.read();
        let r = Exticr1R { bits: bits };
        let mut w = Exticr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Exticr1R {
        Exticr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Exticr1W) -> &mut Exticr1W
    {
        let mut w = Exticr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Exticr1R {
    bits: u32,
}

impl Exticr1R {
    # [ doc = "Bits 12:15 - EXTI x configuration (x = 0 to 3)" ]
    pub fn exti3(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - EXTI x configuration (x = 0 to 3)" ]
    pub fn exti2(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - EXTI x configuration (x = 0 to 3)" ]
    pub fn exti1(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - EXTI x configuration (x = 0 to 3)" ]
    pub fn exti0(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Exticr1W {
    bits: u32,
}

impl Exticr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Exticr1W { bits: 0 }
    }
    # [ doc = "Bits 12:15 - EXTI x configuration (x = 0 to 3)" ]
    pub fn exti3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - EXTI x configuration (x = 0 to 3)" ]
    pub fn exti2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - EXTI x configuration (x = 0 to 3)" ]
    pub fn exti1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - EXTI x configuration (x = 0 to 3)" ]
    pub fn exti0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Exticr2 {
    register: ::volatile_register::RW<u32>,
}

impl Exticr2 {
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
        where for<'w> F: FnOnce(&Exticr2R, &'w mut Exticr2W) -> &'w mut Exticr2W
    {
        let bits = self.register.read();
        let r = Exticr2R { bits: bits };
        let mut w = Exticr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Exticr2R {
        Exticr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Exticr2W) -> &mut Exticr2W
    {
        let mut w = Exticr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Exticr2R {
    bits: u32,
}

impl Exticr2R {
    # [ doc = "Bits 12:15 - EXTI x configuration (x = 4 to 7)" ]
    pub fn exti7(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - EXTI x configuration (x = 4 to 7)" ]
    pub fn exti6(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - EXTI x configuration (x = 4 to 7)" ]
    pub fn exti5(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - EXTI x configuration (x = 4 to 7)" ]
    pub fn exti4(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Exticr2W {
    bits: u32,
}

impl Exticr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Exticr2W { bits: 0 }
    }
    # [ doc = "Bits 12:15 - EXTI x configuration (x = 4 to 7)" ]
    pub fn exti7(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - EXTI x configuration (x = 4 to 7)" ]
    pub fn exti6(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - EXTI x configuration (x = 4 to 7)" ]
    pub fn exti5(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - EXTI x configuration (x = 4 to 7)" ]
    pub fn exti4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Exticr3 {
    register: ::volatile_register::RW<u32>,
}

impl Exticr3 {
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
        where for<'w> F: FnOnce(&Exticr3R, &'w mut Exticr3W) -> &'w mut Exticr3W
    {
        let bits = self.register.read();
        let r = Exticr3R { bits: bits };
        let mut w = Exticr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Exticr3R {
        Exticr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Exticr3W) -> &mut Exticr3W
    {
        let mut w = Exticr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Exticr3R {
    bits: u32,
}

impl Exticr3R {
    # [ doc = "Bits 12:15 - EXTI x configuration (x = 8 to 11)" ]
    pub fn exti11(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - EXTI10" ]
    pub fn exti10(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - EXTI x configuration (x = 8 to 11)" ]
    pub fn exti9(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - EXTI x configuration (x = 8 to 11)" ]
    pub fn exti8(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Exticr3W {
    bits: u32,
}

impl Exticr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Exticr3W { bits: 0 }
    }
    # [ doc = "Bits 12:15 - EXTI x configuration (x = 8 to 11)" ]
    pub fn exti11(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - EXTI10" ]
    pub fn exti10(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - EXTI x configuration (x = 8 to 11)" ]
    pub fn exti9(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - EXTI x configuration (x = 8 to 11)" ]
    pub fn exti8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Exticr4 {
    register: ::volatile_register::RW<u32>,
}

impl Exticr4 {
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
        where for<'w> F: FnOnce(&Exticr4R, &'w mut Exticr4W) -> &'w mut Exticr4W
    {
        let bits = self.register.read();
        let r = Exticr4R { bits: bits };
        let mut w = Exticr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Exticr4R {
        Exticr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Exticr4W) -> &mut Exticr4W
    {
        let mut w = Exticr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Exticr4R {
    bits: u32,
}

impl Exticr4R {
    # [ doc = "Bits 12:15 - EXTI x configuration (x = 12 to 15)" ]
    pub fn exti15(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - EXTI x configuration (x = 12 to 15)" ]
    pub fn exti14(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - EXTI x configuration (x = 12 to 15)" ]
    pub fn exti13(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - EXTI x configuration (x = 12 to 15)" ]
    pub fn exti12(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Exticr4W {
    bits: u32,
}

impl Exticr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Exticr4W { bits: 0 }
    }
    # [ doc = "Bits 12:15 - EXTI x configuration (x = 12 to 15)" ]
    pub fn exti15(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - EXTI x configuration (x = 12 to 15)" ]
    pub fn exti14(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - EXTI x configuration (x = 12 to 15)" ]
    pub fn exti13(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - EXTI x configuration (x = 12 to 15)" ]
    pub fn exti12(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cmpcr {
    register: ::volatile_register::RO<u32>,
}

impl Cmpcr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> CmpcrR {
        CmpcrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CmpcrR {
    bits: u32,
}

impl CmpcrR {
    # [ doc = "Bit 8 - READY" ]
    pub fn ready(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Compensation cell power-down" ]
    pub fn cmp_pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}
