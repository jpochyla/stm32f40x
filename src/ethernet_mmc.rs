# [ doc = "Ethernet: MAC management counters" ]
# [ repr ( C ) ]
pub struct EthernetMmc {
    # [ doc = "0x00 - Ethernet MMC control register" ]
    pub mmccr: Mmccr,
    # [ doc = "0x04 - Ethernet MMC receive interrupt register" ]
    pub mmcrir: Mmcrir,
    # [ doc = "0x08 - Ethernet MMC transmit interrupt register" ]
    pub mmctir: Mmctir,
    # [ doc = "0x0c - Ethernet MMC receive interrupt mask register" ]
    pub mmcrimr: Mmcrimr,
    # [ doc = "0x10 - Ethernet MMC transmit interrupt mask register" ]
    pub mmctimr: Mmctimr,
    _reserved0: [u8; 56usize],
    # [ doc = "0x4c - Ethernet MMC transmitted good frames after a single collision counter" ]
    pub mmctgfsccr: Mmctgfsccr,
    # [ doc = "0x50 - Ethernet MMC transmitted good frames after more than a single collision" ]
    pub mmctgfmsccr: Mmctgfmsccr,
    _reserved1: [u8; 20usize],
    # [ doc = "0x68 - Ethernet MMC transmitted good frames counter register" ]
    pub mmctgfcr: Mmctgfcr,
    _reserved2: [u8; 40usize],
    # [ doc = "0x94 - Ethernet MMC received frames with CRC error counter register" ]
    pub mmcrfcecr: Mmcrfcecr,
    # [ doc = "0x98 - Ethernet MMC received frames with alignment error counter register" ]
    pub mmcrfaecr: Mmcrfaecr,
    _reserved3: [u8; 40usize],
    # [ doc = "0xc4 - MMC received good unicast frames counter register" ]
    pub mmcrgufcr: Mmcrgufcr,
}

# [ repr ( C ) ]
pub struct Mmccr {
    register: ::volatile_register::RW<u32>,
}

impl Mmccr {
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
        where for<'w> F: FnOnce(&MmccrR, &'w mut MmccrW) -> &'w mut MmccrW
    {
        let bits = self.register.read();
        let r = MmccrR { bits: bits };
        let mut w = MmccrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MmccrR {
        MmccrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MmccrW) -> &mut MmccrW
    {
        let mut w = MmccrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmccrR {
    bits: u32,
}

impl MmccrR {
    # [ doc = "Bit 0 - no description available" ]
    pub fn cr(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn csr(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn ror(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn mcf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - no description available" ]
    pub fn mcp(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn mcfhp(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmccrW {
    bits: u32,
}

impl MmccrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MmccrW { bits: 0 }
    }
    # [ doc = "Bit 0 - no description available" ]
    pub fn cr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn csr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn ror(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn mcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - no description available" ]
    pub fn mcp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn mcfhp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Mmcrir {
    register: ::volatile_register::RW<u32>,
}

impl Mmcrir {
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
        where for<'w> F: FnOnce(&MmcrirR, &'w mut MmcrirW) -> &'w mut MmcrirW
    {
        let bits = self.register.read();
        let r = MmcrirR { bits: bits };
        let mut w = MmcrirW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MmcrirR {
        MmcrirR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MmcrirW) -> &mut MmcrirW
    {
        let mut w = MmcrirW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmcrirR {
    bits: u32,
}

impl MmcrirR {
    # [ doc = "Bit 5 - no description available" ]
    pub fn rfces(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn rfaes(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - no description available" ]
    pub fn rgufs(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmcrirW {
    bits: u32,
}

impl MmcrirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MmcrirW { bits: 0 }
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn rfces(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn rfaes(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - no description available" ]
    pub fn rgufs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Mmctir {
    register: ::volatile_register::RO<u32>,
}

impl Mmctir {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> MmctirR {
        MmctirR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmctirR {
    bits: u32,
}

impl MmctirR {
    # [ doc = "Bit 14 - no description available" ]
    pub fn tgfscs(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - no description available" ]
    pub fn tgfmscs(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - no description available" ]
    pub fn tgfs(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Mmcrimr {
    register: ::volatile_register::RW<u32>,
}

impl Mmcrimr {
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
        where for<'w> F: FnOnce(&MmcrimrR, &'w mut MmcrimrW) -> &'w mut MmcrimrW
    {
        let bits = self.register.read();
        let r = MmcrimrR { bits: bits };
        let mut w = MmcrimrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MmcrimrR {
        MmcrimrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MmcrimrW) -> &mut MmcrimrW
    {
        let mut w = MmcrimrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmcrimrR {
    bits: u32,
}

impl MmcrimrR {
    # [ doc = "Bit 5 - no description available" ]
    pub fn rfcem(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn rfaem(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - no description available" ]
    pub fn rgufm(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmcrimrW {
    bits: u32,
}

impl MmcrimrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MmcrimrW { bits: 0 }
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn rfcem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn rfaem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - no description available" ]
    pub fn rgufm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Mmctimr {
    register: ::volatile_register::RW<u32>,
}

impl Mmctimr {
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
        where for<'w> F: FnOnce(&MmctimrR, &'w mut MmctimrW) -> &'w mut MmctimrW
    {
        let bits = self.register.read();
        let r = MmctimrR { bits: bits };
        let mut w = MmctimrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MmctimrR {
        MmctimrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MmctimrW) -> &mut MmctimrW
    {
        let mut w = MmctimrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmctimrR {
    bits: u32,
}

impl MmctimrR {
    # [ doc = "Bit 14 - no description available" ]
    pub fn tgfscm(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - no description available" ]
    pub fn tgfmscm(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - no description available" ]
    pub fn tgfm(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmctimrW {
    bits: u32,
}

impl MmctimrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MmctimrW { bits: 0 }
    }
    # [ doc = "Bit 14 - no description available" ]
    pub fn tgfscm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - no description available" ]
    pub fn tgfmscm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - no description available" ]
    pub fn tgfm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Mmctgfsccr {
    register: ::volatile_register::RO<u32>,
}

impl Mmctgfsccr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> MmctgfsccrR {
        MmctgfsccrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmctgfsccrR {
    bits: u32,
}

impl MmctgfsccrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn tgfscc(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Mmctgfmsccr {
    register: ::volatile_register::RO<u32>,
}

impl Mmctgfmsccr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> MmctgfmsccrR {
        MmctgfmsccrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmctgfmsccrR {
    bits: u32,
}

impl MmctgfmsccrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn tgfmscc(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Mmctgfcr {
    register: ::volatile_register::RO<u32>,
}

impl Mmctgfcr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> MmctgfcrR {
        MmctgfcrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmctgfcrR {
    bits: u32,
}

impl MmctgfcrR {
    # [ doc = "Bits 0:31 - HTL" ]
    pub fn tgfc(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Mmcrfcecr {
    register: ::volatile_register::RO<u32>,
}

impl Mmcrfcecr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> MmcrfcecrR {
        MmcrfcecrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmcrfcecrR {
    bits: u32,
}

impl MmcrfcecrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn rfcfc(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Mmcrfaecr {
    register: ::volatile_register::RO<u32>,
}

impl Mmcrfaecr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> MmcrfaecrR {
        MmcrfaecrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmcrfaecrR {
    bits: u32,
}

impl MmcrfaecrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn rfaec(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Mmcrgufcr {
    register: ::volatile_register::RO<u32>,
}

impl Mmcrgufcr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> MmcrgufcrR {
        MmcrgufcrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MmcrgufcrR {
    bits: u32,
}

impl MmcrgufcrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn rgufc(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}
