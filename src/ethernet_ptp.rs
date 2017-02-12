# [ doc = "Ethernet: Precision time protocol" ]
# [ repr ( C ) ]
pub struct EthernetPtp {
    # [ doc = "0x00 - Ethernet PTP time stamp control register" ]
    pub ptptscr: Ptptscr,
    # [ doc = "0x04 - Ethernet PTP subsecond increment register" ]
    pub ptpssir: Ptpssir,
    # [ doc = "0x08 - Ethernet PTP time stamp high register" ]
    pub ptptshr: Ptptshr,
    # [ doc = "0x0c - Ethernet PTP time stamp low register" ]
    pub ptptslr: Ptptslr,
    # [ doc = "0x10 - Ethernet PTP time stamp high update register" ]
    pub ptptshur: Ptptshur,
    # [ doc = "0x14 - Ethernet PTP time stamp low update register" ]
    pub ptptslur: Ptptslur,
    # [ doc = "0x18 - Ethernet PTP time stamp addend register" ]
    pub ptptsar: Ptptsar,
    # [ doc = "0x1c - Ethernet PTP target time high register" ]
    pub ptptthr: Ptptthr,
    # [ doc = "0x20 - Ethernet PTP target time low register" ]
    pub ptpttlr: Ptpttlr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x28 - Ethernet PTP time stamp status register" ]
    pub ptptssr: Ptptssr,
    # [ doc = "0x2c - Ethernet PTP PPS control register" ]
    pub ptpppscr: Ptpppscr,
}

# [ repr ( C ) ]
pub struct Ptptscr {
    register: ::volatile_register::RW<u32>,
}

impl Ptptscr {
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
        where for<'w> F: FnOnce(&PtptscrR, &'w mut PtptscrW) -> &'w mut PtptscrW
    {
        let bits = self.register.read();
        let r = PtptscrR { bits: bits };
        let mut w = PtptscrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PtptscrR {
        PtptscrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PtptscrW) -> &mut PtptscrW
    {
        let mut w = PtptscrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptscrR {
    bits: u32,
}

impl PtptscrR {
    # [ doc = "Bit 0 - no description available" ]
    pub fn tse(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn tsfcu(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - no description available" ]
    pub fn tsptppsv2e(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - no description available" ]
    pub fn tssptpoefe(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - no description available" ]
    pub fn tssipv6fe(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - no description available" ]
    pub fn tssipv4fe(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - no description available" ]
    pub fn tsseme(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - no description available" ]
    pub fn tssmrme(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - no description available" ]
    pub fn tscnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 18 - no description available" ]
    pub fn tspffmae(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn tssti(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn tsstu(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - no description available" ]
    pub fn tsite(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn ttsaru(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - no description available" ]
    pub fn tssarfe(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn tsssr(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptscrW {
    bits: u32,
}

impl PtptscrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PtptscrW { bits: 8192 }
    }
    # [ doc = "Bit 0 - no description available" ]
    pub fn tse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn tsfcu(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - no description available" ]
    pub fn tsptppsv2e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - no description available" ]
    pub fn tssptpoefe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - no description available" ]
    pub fn tssipv6fe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - no description available" ]
    pub fn tssipv4fe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - no description available" ]
    pub fn tsseme(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - no description available" ]
    pub fn tssmrme(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - no description available" ]
    pub fn tscnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 18 - no description available" ]
    pub fn tspffmae(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn tssti(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn tsstu(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - no description available" ]
    pub fn tsite(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn ttsaru(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - no description available" ]
    pub fn tssarfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn tsssr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ptpssir {
    register: ::volatile_register::RW<u32>,
}

impl Ptpssir {
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
        where for<'w> F: FnOnce(&PtpssirR, &'w mut PtpssirW) -> &'w mut PtpssirW
    {
        let bits = self.register.read();
        let r = PtpssirR { bits: bits };
        let mut w = PtpssirW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PtpssirR {
        PtpssirR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PtpssirW) -> &mut PtpssirW
    {
        let mut w = PtpssirW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtpssirR {
    bits: u32,
}

impl PtpssirR {
    # [ doc = "Bits 0:7 - no description available" ]
    pub fn stssi(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtpssirW {
    bits: u32,
}

impl PtpssirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PtpssirW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - no description available" ]
    pub fn stssi(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ptptshr {
    register: ::volatile_register::RO<u32>,
}

impl Ptptshr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> PtptshrR {
        PtptshrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptshrR {
    bits: u32,
}

impl PtptshrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn sts(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Ptptslr {
    register: ::volatile_register::RO<u32>,
}

impl Ptptslr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> PtptslrR {
        PtptslrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptslrR {
    bits: u32,
}

impl PtptslrR {
    # [ doc = "Bits 0:30 - no description available" ]
    pub fn stss(&self) -> u32 {
        const MASK: u32 = 2147483647;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn stpns(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Ptptshur {
    register: ::volatile_register::RW<u32>,
}

impl Ptptshur {
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
        where for<'w> F: FnOnce(&PtptshurR, &'w mut PtptshurW) -> &'w mut PtptshurW
    {
        let bits = self.register.read();
        let r = PtptshurR { bits: bits };
        let mut w = PtptshurW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PtptshurR {
        PtptshurR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PtptshurW) -> &mut PtptshurW
    {
        let mut w = PtptshurW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptshurR {
    bits: u32,
}

impl PtptshurR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn tsus(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptshurW {
    bits: u32,
}

impl PtptshurW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PtptshurW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn tsus(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ptptslur {
    register: ::volatile_register::RW<u32>,
}

impl Ptptslur {
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
        where for<'w> F: FnOnce(&PtptslurR, &'w mut PtptslurW) -> &'w mut PtptslurW
    {
        let bits = self.register.read();
        let r = PtptslurR { bits: bits };
        let mut w = PtptslurW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PtptslurR {
        PtptslurR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PtptslurW) -> &mut PtptslurW
    {
        let mut w = PtptslurW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptslurR {
    bits: u32,
}

impl PtptslurR {
    # [ doc = "Bits 0:30 - no description available" ]
    pub fn tsuss(&self) -> u32 {
        const MASK: u32 = 2147483647;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn tsupns(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptslurW {
    bits: u32,
}

impl PtptslurW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PtptslurW { bits: 0 }
    }
    # [ doc = "Bits 0:30 - no description available" ]
    pub fn tsuss(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 2147483647;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn tsupns(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ptptsar {
    register: ::volatile_register::RW<u32>,
}

impl Ptptsar {
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
        where for<'w> F: FnOnce(&PtptsarR, &'w mut PtptsarW) -> &'w mut PtptsarW
    {
        let bits = self.register.read();
        let r = PtptsarR { bits: bits };
        let mut w = PtptsarW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PtptsarR {
        PtptsarR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PtptsarW) -> &mut PtptsarW
    {
        let mut w = PtptsarW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptsarR {
    bits: u32,
}

impl PtptsarR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn tsa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptsarW {
    bits: u32,
}

impl PtptsarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PtptsarW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn tsa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ptptthr {
    register: ::volatile_register::RW<u32>,
}

impl Ptptthr {
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
        where for<'w> F: FnOnce(&PtptthrR, &'w mut PtptthrW) -> &'w mut PtptthrW
    {
        let bits = self.register.read();
        let r = PtptthrR { bits: bits };
        let mut w = PtptthrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PtptthrR {
        PtptthrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PtptthrW) -> &mut PtptthrW
    {
        let mut w = PtptthrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptthrR {
    bits: u32,
}

impl PtptthrR {
    # [ doc = "Bits 0:31 - 0" ]
    pub fn ttsh(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptthrW {
    bits: u32,
}

impl PtptthrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PtptthrW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - 0" ]
    pub fn ttsh(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ptpttlr {
    register: ::volatile_register::RW<u32>,
}

impl Ptpttlr {
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
        where for<'w> F: FnOnce(&PtpttlrR, &'w mut PtpttlrW) -> &'w mut PtpttlrW
    {
        let bits = self.register.read();
        let r = PtpttlrR { bits: bits };
        let mut w = PtpttlrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PtpttlrR {
        PtpttlrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PtpttlrW) -> &mut PtpttlrW
    {
        let mut w = PtpttlrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtpttlrR {
    bits: u32,
}

impl PtpttlrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn ttsl(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtpttlrW {
    bits: u32,
}

impl PtpttlrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PtpttlrW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn ttsl(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ptptssr {
    register: ::volatile_register::RO<u32>,
}

impl Ptptssr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> PtptssrR {
        PtptssrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtptssrR {
    bits: u32,
}

impl PtptssrR {
    # [ doc = "Bit 0 - no description available" ]
    pub fn tsso(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn tsttr(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Ptpppscr {
    register: ::volatile_register::RO<u32>,
}

impl Ptpppscr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> PtpppscrR {
        PtpppscrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtpppscrR {
    bits: u32,
}

impl PtpppscrR {
    # [ doc = "Bit 0 - TSSO" ]
    pub fn tsso(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - TSTTR" ]
    pub fn tsttr(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
}
