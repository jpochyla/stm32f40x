# [ doc = "Nested Vectored Interrupt Controller" ]
# [ repr ( C ) ]
pub struct Nvic {
    _reserved0: [u8; 4usize],
    # [ doc = "0x04 - Interrupt Controller Type Register" ]
    pub ictr: Ictr,
    _reserved1: [u8; 248usize],
    # [ doc = "0x100 - Interrupt Set-Enable Register" ]
    pub iser0: Iser0,
    # [ doc = "0x104 - Interrupt Set-Enable Register" ]
    pub iser1: Iser1,
    # [ doc = "0x108 - Interrupt Set-Enable Register" ]
    pub iser2: Iser2,
    _reserved2: [u8; 116usize],
    # [ doc = "0x180 - Interrupt Clear-Enable Register" ]
    pub icer0: Icer0,
    # [ doc = "0x184 - Interrupt Clear-Enable Register" ]
    pub icer1: Icer1,
    # [ doc = "0x188 - Interrupt Clear-Enable Register" ]
    pub icer2: Icer2,
    _reserved3: [u8; 116usize],
    # [ doc = "0x200 - Interrupt Set-Pending Register" ]
    pub ispr0: Ispr0,
    # [ doc = "0x204 - Interrupt Set-Pending Register" ]
    pub ispr1: Ispr1,
    # [ doc = "0x208 - Interrupt Set-Pending Register" ]
    pub ispr2: Ispr2,
    _reserved4: [u8; 116usize],
    # [ doc = "0x280 - Interrupt Clear-Pending Register" ]
    pub icpr0: Icpr0,
    # [ doc = "0x284 - Interrupt Clear-Pending Register" ]
    pub icpr1: Icpr1,
    # [ doc = "0x288 - Interrupt Clear-Pending Register" ]
    pub icpr2: Icpr2,
    _reserved5: [u8; 116usize],
    # [ doc = "0x300 - Interrupt Active Bit Register" ]
    pub iabr0: Iabr0,
    # [ doc = "0x304 - Interrupt Active Bit Register" ]
    pub iabr1: Iabr1,
    # [ doc = "0x308 - Interrupt Active Bit Register" ]
    pub iabr2: Iabr2,
    _reserved6: [u8; 244usize],
    # [ doc = "0x400 - Interrupt Priority Register" ]
    pub ipr0: Ipr0,
    # [ doc = "0x404 - Interrupt Priority Register" ]
    pub ipr1: Ipr1,
    # [ doc = "0x408 - Interrupt Priority Register" ]
    pub ipr2: Ipr2,
    # [ doc = "0x40c - Interrupt Priority Register" ]
    pub ipr3: Ipr3,
    # [ doc = "0x410 - Interrupt Priority Register" ]
    pub ipr4: Ipr4,
    # [ doc = "0x414 - Interrupt Priority Register" ]
    pub ipr5: Ipr5,
    # [ doc = "0x418 - Interrupt Priority Register" ]
    pub ipr6: Ipr6,
    # [ doc = "0x41c - Interrupt Priority Register" ]
    pub ipr7: Ipr7,
    # [ doc = "0x420 - Interrupt Priority Register" ]
    pub ipr8: Ipr8,
    # [ doc = "0x424 - Interrupt Priority Register" ]
    pub ipr9: Ipr9,
    # [ doc = "0x428 - Interrupt Priority Register" ]
    pub ipr10: Ipr10,
    # [ doc = "0x42c - Interrupt Priority Register" ]
    pub ipr11: Ipr11,
    # [ doc = "0x430 - Interrupt Priority Register" ]
    pub ipr12: Ipr12,
    # [ doc = "0x434 - Interrupt Priority Register" ]
    pub ipr13: Ipr13,
    # [ doc = "0x438 - Interrupt Priority Register" ]
    pub ipr14: Ipr14,
    # [ doc = "0x43c - Interrupt Priority Register" ]
    pub ipr15: Ipr15,
    # [ doc = "0x440 - Interrupt Priority Register" ]
    pub ipr16: Ipr16,
    # [ doc = "0x444 - Interrupt Priority Register" ]
    pub ipr17: Ipr17,
    # [ doc = "0x448 - Interrupt Priority Register" ]
    pub ipr18: Ipr18,
    # [ doc = "0x44c - Interrupt Priority Register" ]
    pub ipr19: Ipr19,
    _reserved7: [u8; 2736usize],
    # [ doc = "0xf00 - Software Triggered Interrupt Register" ]
    pub stir: Stir,
}

# [ repr ( C ) ]
pub struct Ictr {
    register: ::volatile_register::RO<u32>,
}

impl Ictr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> IctrR {
        IctrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IctrR {
    bits: u32,
}

impl IctrR {
    # [ doc = "Bits 0:3 - Total number of interrupt lines in groups" ]
    pub fn intlinesnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Stir {
    register: ::volatile_register::WO<u32>,
}

impl Stir {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut StirW) -> &mut StirW
    {
        let mut w = StirW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct StirW {
    bits: u32,
}

impl StirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        StirW { bits: 0 }
    }
    # [ doc = "Bits 0:8 - interrupt to be triggered" ]
    pub fn intid(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 511;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Iser0 {
    register: ::volatile_register::RW<u32>,
}

impl Iser0 {
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
        where for<'w> F: FnOnce(&Iser0R, &'w mut Iser0W) -> &'w mut Iser0W
    {
        let bits = self.register.read();
        let r = Iser0R { bits: bits };
        let mut w = Iser0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Iser0R {
        Iser0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Iser0W) -> &mut Iser0W
    {
        let mut w = Iser0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Iser0R {
    bits: u32,
}

impl Iser0R {
    # [ doc = "Bits 0:31 - SETENA" ]
    pub fn setena(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Iser0W {
    bits: u32,
}

impl Iser0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Iser0W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - SETENA" ]
    pub fn setena(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Iser1 {
    register: ::volatile_register::RW<u32>,
}

impl Iser1 {
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
        where for<'w> F: FnOnce(&Iser1R, &'w mut Iser1W) -> &'w mut Iser1W
    {
        let bits = self.register.read();
        let r = Iser1R { bits: bits };
        let mut w = Iser1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Iser1R {
        Iser1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Iser1W) -> &mut Iser1W
    {
        let mut w = Iser1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Iser1R {
    bits: u32,
}

impl Iser1R {
    # [ doc = "Bits 0:31 - SETENA" ]
    pub fn setena(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Iser1W {
    bits: u32,
}

impl Iser1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Iser1W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - SETENA" ]
    pub fn setena(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Iser2 {
    register: ::volatile_register::RW<u32>,
}

impl Iser2 {
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
        where for<'w> F: FnOnce(&Iser2R, &'w mut Iser2W) -> &'w mut Iser2W
    {
        let bits = self.register.read();
        let r = Iser2R { bits: bits };
        let mut w = Iser2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Iser2R {
        Iser2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Iser2W) -> &mut Iser2W
    {
        let mut w = Iser2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Iser2R {
    bits: u32,
}

impl Iser2R {
    # [ doc = "Bits 0:31 - SETENA" ]
    pub fn setena(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Iser2W {
    bits: u32,
}

impl Iser2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Iser2W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - SETENA" ]
    pub fn setena(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Icer0 {
    register: ::volatile_register::RW<u32>,
}

impl Icer0 {
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
        where for<'w> F: FnOnce(&Icer0R, &'w mut Icer0W) -> &'w mut Icer0W
    {
        let bits = self.register.read();
        let r = Icer0R { bits: bits };
        let mut w = Icer0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Icer0R {
        Icer0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Icer0W) -> &mut Icer0W
    {
        let mut w = Icer0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icer0R {
    bits: u32,
}

impl Icer0R {
    # [ doc = "Bits 0:31 - CLRENA" ]
    pub fn clrena(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icer0W {
    bits: u32,
}

impl Icer0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Icer0W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - CLRENA" ]
    pub fn clrena(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Icer1 {
    register: ::volatile_register::RW<u32>,
}

impl Icer1 {
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
        where for<'w> F: FnOnce(&Icer1R, &'w mut Icer1W) -> &'w mut Icer1W
    {
        let bits = self.register.read();
        let r = Icer1R { bits: bits };
        let mut w = Icer1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Icer1R {
        Icer1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Icer1W) -> &mut Icer1W
    {
        let mut w = Icer1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icer1R {
    bits: u32,
}

impl Icer1R {
    # [ doc = "Bits 0:31 - CLRENA" ]
    pub fn clrena(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icer1W {
    bits: u32,
}

impl Icer1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Icer1W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - CLRENA" ]
    pub fn clrena(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Icer2 {
    register: ::volatile_register::RW<u32>,
}

impl Icer2 {
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
        where for<'w> F: FnOnce(&Icer2R, &'w mut Icer2W) -> &'w mut Icer2W
    {
        let bits = self.register.read();
        let r = Icer2R { bits: bits };
        let mut w = Icer2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Icer2R {
        Icer2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Icer2W) -> &mut Icer2W
    {
        let mut w = Icer2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icer2R {
    bits: u32,
}

impl Icer2R {
    # [ doc = "Bits 0:31 - CLRENA" ]
    pub fn clrena(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icer2W {
    bits: u32,
}

impl Icer2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Icer2W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - CLRENA" ]
    pub fn clrena(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ispr0 {
    register: ::volatile_register::RW<u32>,
}

impl Ispr0 {
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
        where for<'w> F: FnOnce(&Ispr0R, &'w mut Ispr0W) -> &'w mut Ispr0W
    {
        let bits = self.register.read();
        let r = Ispr0R { bits: bits };
        let mut w = Ispr0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ispr0R {
        Ispr0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ispr0W) -> &mut Ispr0W
    {
        let mut w = Ispr0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ispr0R {
    bits: u32,
}

impl Ispr0R {
    # [ doc = "Bits 0:31 - SETPEND" ]
    pub fn setpend(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ispr0W {
    bits: u32,
}

impl Ispr0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ispr0W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - SETPEND" ]
    pub fn setpend(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ispr1 {
    register: ::volatile_register::RW<u32>,
}

impl Ispr1 {
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
        where for<'w> F: FnOnce(&Ispr1R, &'w mut Ispr1W) -> &'w mut Ispr1W
    {
        let bits = self.register.read();
        let r = Ispr1R { bits: bits };
        let mut w = Ispr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ispr1R {
        Ispr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ispr1W) -> &mut Ispr1W
    {
        let mut w = Ispr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ispr1R {
    bits: u32,
}

impl Ispr1R {
    # [ doc = "Bits 0:31 - SETPEND" ]
    pub fn setpend(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ispr1W {
    bits: u32,
}

impl Ispr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ispr1W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - SETPEND" ]
    pub fn setpend(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ispr2 {
    register: ::volatile_register::RW<u32>,
}

impl Ispr2 {
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
        where for<'w> F: FnOnce(&Ispr2R, &'w mut Ispr2W) -> &'w mut Ispr2W
    {
        let bits = self.register.read();
        let r = Ispr2R { bits: bits };
        let mut w = Ispr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ispr2R {
        Ispr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ispr2W) -> &mut Ispr2W
    {
        let mut w = Ispr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ispr2R {
    bits: u32,
}

impl Ispr2R {
    # [ doc = "Bits 0:31 - SETPEND" ]
    pub fn setpend(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ispr2W {
    bits: u32,
}

impl Ispr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ispr2W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - SETPEND" ]
    pub fn setpend(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Icpr0 {
    register: ::volatile_register::RW<u32>,
}

impl Icpr0 {
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
        where for<'w> F: FnOnce(&Icpr0R, &'w mut Icpr0W) -> &'w mut Icpr0W
    {
        let bits = self.register.read();
        let r = Icpr0R { bits: bits };
        let mut w = Icpr0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Icpr0R {
        Icpr0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Icpr0W) -> &mut Icpr0W
    {
        let mut w = Icpr0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icpr0R {
    bits: u32,
}

impl Icpr0R {
    # [ doc = "Bits 0:31 - CLRPEND" ]
    pub fn clrpend(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icpr0W {
    bits: u32,
}

impl Icpr0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Icpr0W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - CLRPEND" ]
    pub fn clrpend(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Icpr1 {
    register: ::volatile_register::RW<u32>,
}

impl Icpr1 {
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
        where for<'w> F: FnOnce(&Icpr1R, &'w mut Icpr1W) -> &'w mut Icpr1W
    {
        let bits = self.register.read();
        let r = Icpr1R { bits: bits };
        let mut w = Icpr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Icpr1R {
        Icpr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Icpr1W) -> &mut Icpr1W
    {
        let mut w = Icpr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icpr1R {
    bits: u32,
}

impl Icpr1R {
    # [ doc = "Bits 0:31 - CLRPEND" ]
    pub fn clrpend(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icpr1W {
    bits: u32,
}

impl Icpr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Icpr1W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - CLRPEND" ]
    pub fn clrpend(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Icpr2 {
    register: ::volatile_register::RW<u32>,
}

impl Icpr2 {
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
        where for<'w> F: FnOnce(&Icpr2R, &'w mut Icpr2W) -> &'w mut Icpr2W
    {
        let bits = self.register.read();
        let r = Icpr2R { bits: bits };
        let mut w = Icpr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Icpr2R {
        Icpr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Icpr2W) -> &mut Icpr2W
    {
        let mut w = Icpr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icpr2R {
    bits: u32,
}

impl Icpr2R {
    # [ doc = "Bits 0:31 - CLRPEND" ]
    pub fn clrpend(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icpr2W {
    bits: u32,
}

impl Icpr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Icpr2W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - CLRPEND" ]
    pub fn clrpend(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Iabr0 {
    register: ::volatile_register::RO<u32>,
}

impl Iabr0 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> Iabr0R {
        Iabr0R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Iabr0R {
    bits: u32,
}

impl Iabr0R {
    # [ doc = "Bits 0:31 - ACTIVE" ]
    pub fn active(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Iabr1 {
    register: ::volatile_register::RO<u32>,
}

impl Iabr1 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> Iabr1R {
        Iabr1R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Iabr1R {
    bits: u32,
}

impl Iabr1R {
    # [ doc = "Bits 0:31 - ACTIVE" ]
    pub fn active(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Iabr2 {
    register: ::volatile_register::RO<u32>,
}

impl Iabr2 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> Iabr2R {
        Iabr2R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Iabr2R {
    bits: u32,
}

impl Iabr2R {
    # [ doc = "Bits 0:31 - ACTIVE" ]
    pub fn active(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Ipr0 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr0 {
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
        where for<'w> F: FnOnce(&Ipr0R, &'w mut Ipr0W) -> &'w mut Ipr0W
    {
        let bits = self.register.read();
        let r = Ipr0R { bits: bits };
        let mut w = Ipr0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr0R {
        Ipr0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr0W) -> &mut Ipr0W
    {
        let mut w = Ipr0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr0R {
    bits: u32,
}

impl Ipr0R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr0W {
    bits: u32,
}

impl Ipr0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr0W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr1 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr1 {
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
        where for<'w> F: FnOnce(&Ipr1R, &'w mut Ipr1W) -> &'w mut Ipr1W
    {
        let bits = self.register.read();
        let r = Ipr1R { bits: bits };
        let mut w = Ipr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr1R {
        Ipr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr1W) -> &mut Ipr1W
    {
        let mut w = Ipr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr1R {
    bits: u32,
}

impl Ipr1R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr1W {
    bits: u32,
}

impl Ipr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr1W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr2 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr2 {
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
        where for<'w> F: FnOnce(&Ipr2R, &'w mut Ipr2W) -> &'w mut Ipr2W
    {
        let bits = self.register.read();
        let r = Ipr2R { bits: bits };
        let mut w = Ipr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr2R {
        Ipr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr2W) -> &mut Ipr2W
    {
        let mut w = Ipr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr2R {
    bits: u32,
}

impl Ipr2R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr2W {
    bits: u32,
}

impl Ipr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr2W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr3 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr3 {
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
        where for<'w> F: FnOnce(&Ipr3R, &'w mut Ipr3W) -> &'w mut Ipr3W
    {
        let bits = self.register.read();
        let r = Ipr3R { bits: bits };
        let mut w = Ipr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr3R {
        Ipr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr3W) -> &mut Ipr3W
    {
        let mut w = Ipr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr3R {
    bits: u32,
}

impl Ipr3R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr3W {
    bits: u32,
}

impl Ipr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr3W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr4 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr4 {
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
        where for<'w> F: FnOnce(&Ipr4R, &'w mut Ipr4W) -> &'w mut Ipr4W
    {
        let bits = self.register.read();
        let r = Ipr4R { bits: bits };
        let mut w = Ipr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr4R {
        Ipr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr4W) -> &mut Ipr4W
    {
        let mut w = Ipr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr4R {
    bits: u32,
}

impl Ipr4R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr4W {
    bits: u32,
}

impl Ipr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr4W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr5 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr5 {
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
        where for<'w> F: FnOnce(&Ipr5R, &'w mut Ipr5W) -> &'w mut Ipr5W
    {
        let bits = self.register.read();
        let r = Ipr5R { bits: bits };
        let mut w = Ipr5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr5R {
        Ipr5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr5W) -> &mut Ipr5W
    {
        let mut w = Ipr5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr5R {
    bits: u32,
}

impl Ipr5R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr5W {
    bits: u32,
}

impl Ipr5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr5W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr6 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr6 {
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
        where for<'w> F: FnOnce(&Ipr6R, &'w mut Ipr6W) -> &'w mut Ipr6W
    {
        let bits = self.register.read();
        let r = Ipr6R { bits: bits };
        let mut w = Ipr6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr6R {
        Ipr6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr6W) -> &mut Ipr6W
    {
        let mut w = Ipr6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr6R {
    bits: u32,
}

impl Ipr6R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr6W {
    bits: u32,
}

impl Ipr6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr6W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr7 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr7 {
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
        where for<'w> F: FnOnce(&Ipr7R, &'w mut Ipr7W) -> &'w mut Ipr7W
    {
        let bits = self.register.read();
        let r = Ipr7R { bits: bits };
        let mut w = Ipr7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr7R {
        Ipr7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr7W) -> &mut Ipr7W
    {
        let mut w = Ipr7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr7R {
    bits: u32,
}

impl Ipr7R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr7W {
    bits: u32,
}

impl Ipr7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr7W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr8 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr8 {
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
        where for<'w> F: FnOnce(&Ipr8R, &'w mut Ipr8W) -> &'w mut Ipr8W
    {
        let bits = self.register.read();
        let r = Ipr8R { bits: bits };
        let mut w = Ipr8W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr8R {
        Ipr8R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr8W) -> &mut Ipr8W
    {
        let mut w = Ipr8W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr8R {
    bits: u32,
}

impl Ipr8R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr8W {
    bits: u32,
}

impl Ipr8W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr8W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr9 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr9 {
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
        where for<'w> F: FnOnce(&Ipr9R, &'w mut Ipr9W) -> &'w mut Ipr9W
    {
        let bits = self.register.read();
        let r = Ipr9R { bits: bits };
        let mut w = Ipr9W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr9R {
        Ipr9R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr9W) -> &mut Ipr9W
    {
        let mut w = Ipr9W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr9R {
    bits: u32,
}

impl Ipr9R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr9W {
    bits: u32,
}

impl Ipr9W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr9W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr10 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr10 {
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
        where for<'w> F: FnOnce(&Ipr10R, &'w mut Ipr10W) -> &'w mut Ipr10W
    {
        let bits = self.register.read();
        let r = Ipr10R { bits: bits };
        let mut w = Ipr10W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr10R {
        Ipr10R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr10W) -> &mut Ipr10W
    {
        let mut w = Ipr10W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr10R {
    bits: u32,
}

impl Ipr10R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr10W {
    bits: u32,
}

impl Ipr10W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr10W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr11 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr11 {
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
        where for<'w> F: FnOnce(&Ipr11R, &'w mut Ipr11W) -> &'w mut Ipr11W
    {
        let bits = self.register.read();
        let r = Ipr11R { bits: bits };
        let mut w = Ipr11W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr11R {
        Ipr11R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr11W) -> &mut Ipr11W
    {
        let mut w = Ipr11W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr11R {
    bits: u32,
}

impl Ipr11R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr11W {
    bits: u32,
}

impl Ipr11W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr11W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr12 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr12 {
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
        where for<'w> F: FnOnce(&Ipr12R, &'w mut Ipr12W) -> &'w mut Ipr12W
    {
        let bits = self.register.read();
        let r = Ipr12R { bits: bits };
        let mut w = Ipr12W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr12R {
        Ipr12R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr12W) -> &mut Ipr12W
    {
        let mut w = Ipr12W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr12R {
    bits: u32,
}

impl Ipr12R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr12W {
    bits: u32,
}

impl Ipr12W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr12W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr13 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr13 {
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
        where for<'w> F: FnOnce(&Ipr13R, &'w mut Ipr13W) -> &'w mut Ipr13W
    {
        let bits = self.register.read();
        let r = Ipr13R { bits: bits };
        let mut w = Ipr13W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr13R {
        Ipr13R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr13W) -> &mut Ipr13W
    {
        let mut w = Ipr13W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr13R {
    bits: u32,
}

impl Ipr13R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr13W {
    bits: u32,
}

impl Ipr13W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr13W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr14 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr14 {
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
        where for<'w> F: FnOnce(&Ipr14R, &'w mut Ipr14W) -> &'w mut Ipr14W
    {
        let bits = self.register.read();
        let r = Ipr14R { bits: bits };
        let mut w = Ipr14W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr14R {
        Ipr14R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr14W) -> &mut Ipr14W
    {
        let mut w = Ipr14W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr14R {
    bits: u32,
}

impl Ipr14R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr14W {
    bits: u32,
}

impl Ipr14W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr14W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr15 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr15 {
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
        where for<'w> F: FnOnce(&Ipr15R, &'w mut Ipr15W) -> &'w mut Ipr15W
    {
        let bits = self.register.read();
        let r = Ipr15R { bits: bits };
        let mut w = Ipr15W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr15R {
        Ipr15R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr15W) -> &mut Ipr15W
    {
        let mut w = Ipr15W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr15R {
    bits: u32,
}

impl Ipr15R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr15W {
    bits: u32,
}

impl Ipr15W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr15W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr16 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr16 {
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
        where for<'w> F: FnOnce(&Ipr16R, &'w mut Ipr16W) -> &'w mut Ipr16W
    {
        let bits = self.register.read();
        let r = Ipr16R { bits: bits };
        let mut w = Ipr16W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr16R {
        Ipr16R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr16W) -> &mut Ipr16W
    {
        let mut w = Ipr16W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr16R {
    bits: u32,
}

impl Ipr16R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr16W {
    bits: u32,
}

impl Ipr16W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr16W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr17 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr17 {
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
        where for<'w> F: FnOnce(&Ipr17R, &'w mut Ipr17W) -> &'w mut Ipr17W
    {
        let bits = self.register.read();
        let r = Ipr17R { bits: bits };
        let mut w = Ipr17W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr17R {
        Ipr17R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr17W) -> &mut Ipr17W
    {
        let mut w = Ipr17W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr17R {
    bits: u32,
}

impl Ipr17R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr17W {
    bits: u32,
}

impl Ipr17W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr17W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr18 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr18 {
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
        where for<'w> F: FnOnce(&Ipr18R, &'w mut Ipr18W) -> &'w mut Ipr18W
    {
        let bits = self.register.read();
        let r = Ipr18R { bits: bits };
        let mut w = Ipr18W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr18R {
        Ipr18R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr18W) -> &mut Ipr18W
    {
        let mut w = Ipr18W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr18R {
    bits: u32,
}

impl Ipr18R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr18W {
    bits: u32,
}

impl Ipr18W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr18W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ipr19 {
    register: ::volatile_register::RW<u32>,
}

impl Ipr19 {
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
        where for<'w> F: FnOnce(&Ipr19R, &'w mut Ipr19W) -> &'w mut Ipr19W
    {
        let bits = self.register.read();
        let r = Ipr19R { bits: bits };
        let mut w = Ipr19W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ipr19R {
        Ipr19R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ipr19W) -> &mut Ipr19W
    {
        let mut w = Ipr19W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr19R {
    bits: u32,
}

impl Ipr19R {
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ipr19W {
    bits: u32,
}

impl Ipr19W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ipr19W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - IPR_N0" ]
    pub fn ipr_n0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - IPR_N1" ]
    pub fn ipr_n1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - IPR_N2" ]
    pub fn ipr_n2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:31 - IPR_N3" ]
    pub fn ipr_n3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
