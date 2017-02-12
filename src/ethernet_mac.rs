# [ doc = "Ethernet: media access control (MAC)" ]
# [ repr ( C ) ]
pub struct EthernetMac {
    # [ doc = "0x00 - Ethernet MAC configuration register" ]
    pub maccr: Maccr,
    # [ doc = "0x04 - Ethernet MAC frame filter register" ]
    pub macffr: Macffr,
    # [ doc = "0x08 - Ethernet MAC hash table high register" ]
    pub machthr: Machthr,
    # [ doc = "0x0c - Ethernet MAC hash table low register" ]
    pub machtlr: Machtlr,
    # [ doc = "0x10 - Ethernet MAC MII address register" ]
    pub macmiiar: Macmiiar,
    # [ doc = "0x14 - Ethernet MAC MII data register" ]
    pub macmiidr: Macmiidr,
    # [ doc = "0x18 - Ethernet MAC flow control register" ]
    pub macfcr: Macfcr,
    # [ doc = "0x1c - Ethernet MAC VLAN tag register" ]
    pub macvlantr: Macvlantr,
    _reserved0: [u8; 12usize],
    # [ doc = "0x2c - Ethernet MAC PMT control and status register" ]
    pub macpmtcsr: Macpmtcsr,
    _reserved1: [u8; 4usize],
    # [ doc = "0x34 - Ethernet MAC debug register" ]
    pub macdbgr: Macdbgr,
    # [ doc = "0x38 - Ethernet MAC interrupt status register" ]
    pub macsr: Macsr,
    # [ doc = "0x3c - Ethernet MAC interrupt mask register" ]
    pub macimr: Macimr,
    # [ doc = "0x40 - Ethernet MAC address 0 high register" ]
    pub maca0hr: Maca0hr,
    # [ doc = "0x44 - Ethernet MAC address 0 low register" ]
    pub maca0lr: Maca0lr,
    # [ doc = "0x48 - Ethernet MAC address 1 high register" ]
    pub maca1hr: Maca1hr,
    # [ doc = "0x4c - Ethernet MAC address1 low register" ]
    pub maca1lr: Maca1lr,
    # [ doc = "0x50 - Ethernet MAC address 2 high register" ]
    pub maca2hr: Maca2hr,
    # [ doc = "0x54 - Ethernet MAC address 2 low register" ]
    pub maca2lr: Maca2lr,
    # [ doc = "0x58 - Ethernet MAC address 3 high register" ]
    pub maca3hr: Maca3hr,
    # [ doc = "0x5c - Ethernet MAC address 3 low register" ]
    pub maca3lr: Maca3lr,
}

# [ repr ( C ) ]
pub struct Maccr {
    register: ::volatile_register::RW<u32>,
}

impl Maccr {
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
        where for<'w> F: FnOnce(&MaccrR, &'w mut MaccrW) -> &'w mut MaccrW
    {
        let bits = self.register.read();
        let r = MaccrR { bits: bits };
        let mut w = MaccrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MaccrR {
        MaccrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MaccrW) -> &mut MaccrW
    {
        let mut w = MaccrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MaccrR {
    bits: u32,
}

impl MaccrR {
    # [ doc = "Bit 2 - RE" ]
    pub fn re(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TE" ]
    pub fn te(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - DC" ]
    pub fn dc(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 5:6 - BL" ]
    pub fn bl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - APCS" ]
    pub fn apcs(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - RD" ]
    pub fn rd(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - IPCO" ]
    pub fn ipco(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - DM" ]
    pub fn dm(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - LM" ]
    pub fn lm(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - ROD" ]
    pub fn rod(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - FES" ]
    pub fn fes(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - CSD" ]
    pub fn csd(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 17:19 - IFG" ]
    pub fn ifg(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 22 - JD" ]
    pub fn jd(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - WD" ]
    pub fn wd(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - CSTF" ]
    pub fn cstf(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MaccrW {
    bits: u32,
}

impl MaccrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MaccrW { bits: 32768 }
    }
    # [ doc = "Bit 2 - RE" ]
    pub fn re(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TE" ]
    pub fn te(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - DC" ]
    pub fn dc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 5:6 - BL" ]
    pub fn bl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - APCS" ]
    pub fn apcs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - RD" ]
    pub fn rd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - IPCO" ]
    pub fn ipco(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - DM" ]
    pub fn dm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - LM" ]
    pub fn lm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - ROD" ]
    pub fn rod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - FES" ]
    pub fn fes(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - CSD" ]
    pub fn csd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 17:19 - IFG" ]
    pub fn ifg(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 17u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 22 - JD" ]
    pub fn jd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - WD" ]
    pub fn wd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - CSTF" ]
    pub fn cstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Macffr {
    register: ::volatile_register::RW<u32>,
}

impl Macffr {
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
        where for<'w> F: FnOnce(&MacffrR, &'w mut MacffrW) -> &'w mut MacffrW
    {
        let bits = self.register.read();
        let r = MacffrR { bits: bits };
        let mut w = MacffrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MacffrR {
        MacffrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MacffrW) -> &mut MacffrW
    {
        let mut w = MacffrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacffrR {
    bits: u32,
}

impl MacffrR {
    # [ doc = "Bit 0 - no description available" ]
    pub fn pm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn hu(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn hm(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn daif(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - no description available" ]
    pub fn ram(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn bfd(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn pcf(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - no description available" ]
    pub fn saif(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - no description available" ]
    pub fn saf(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn hpf(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn ra(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacffrW {
    bits: u32,
}

impl MacffrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MacffrW { bits: 0 }
    }
    # [ doc = "Bit 0 - no description available" ]
    pub fn pm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn hu(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn hm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn daif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - no description available" ]
    pub fn ram(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn bfd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn pcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - no description available" ]
    pub fn saif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - no description available" ]
    pub fn saf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn hpf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn ra(&mut self, value: bool) -> &mut Self {
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
pub struct Machthr {
    register: ::volatile_register::RW<u32>,
}

impl Machthr {
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
        where for<'w> F: FnOnce(&MachthrR, &'w mut MachthrW) -> &'w mut MachthrW
    {
        let bits = self.register.read();
        let r = MachthrR { bits: bits };
        let mut w = MachthrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MachthrR {
        MachthrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MachthrW) -> &mut MachthrW
    {
        let mut w = MachthrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MachthrR {
    bits: u32,
}

impl MachthrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn hth(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MachthrW {
    bits: u32,
}

impl MachthrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MachthrW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn hth(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Machtlr {
    register: ::volatile_register::RW<u32>,
}

impl Machtlr {
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
        where for<'w> F: FnOnce(&MachtlrR, &'w mut MachtlrW) -> &'w mut MachtlrW
    {
        let bits = self.register.read();
        let r = MachtlrR { bits: bits };
        let mut w = MachtlrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MachtlrR {
        MachtlrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MachtlrW) -> &mut MachtlrW
    {
        let mut w = MachtlrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MachtlrR {
    bits: u32,
}

impl MachtlrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn htl(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MachtlrW {
    bits: u32,
}

impl MachtlrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MachtlrW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn htl(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Macmiiar {
    register: ::volatile_register::RW<u32>,
}

impl Macmiiar {
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
        where for<'w> F: FnOnce(&MacmiiarR, &'w mut MacmiiarW) -> &'w mut MacmiiarW
    {
        let bits = self.register.read();
        let r = MacmiiarR { bits: bits };
        let mut w = MacmiiarW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MacmiiarR {
        MacmiiarR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MacmiiarW) -> &mut MacmiiarW
    {
        let mut w = MacmiiarW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacmiiarR {
    bits: u32,
}

impl MacmiiarR {
    # [ doc = "Bit 0 - no description available" ]
    pub fn mb(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn mw(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:4 - no description available" ]
    pub fn cr(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:10 - no description available" ]
    pub fn mr(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:15 - no description available" ]
    pub fn pa(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacmiiarW {
    bits: u32,
}

impl MacmiiarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MacmiiarW { bits: 0 }
    }
    # [ doc = "Bit 0 - no description available" ]
    pub fn mb(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn mw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:4 - no description available" ]
    pub fn cr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:10 - no description available" ]
    pub fn mr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:15 - no description available" ]
    pub fn pa(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Macmiidr {
    register: ::volatile_register::RW<u32>,
}

impl Macmiidr {
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
        where for<'w> F: FnOnce(&MacmiidrR, &'w mut MacmiidrW) -> &'w mut MacmiidrW
    {
        let bits = self.register.read();
        let r = MacmiidrR { bits: bits };
        let mut w = MacmiidrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MacmiidrR {
        MacmiidrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MacmiidrW) -> &mut MacmiidrW
    {
        let mut w = MacmiidrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacmiidrR {
    bits: u32,
}

impl MacmiidrR {
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn td(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacmiidrW {
    bits: u32,
}

impl MacmiidrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MacmiidrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn td(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Macfcr {
    register: ::volatile_register::RW<u32>,
}

impl Macfcr {
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
        where for<'w> F: FnOnce(&MacfcrR, &'w mut MacfcrW) -> &'w mut MacfcrW
    {
        let bits = self.register.read();
        let r = MacfcrR { bits: bits };
        let mut w = MacfcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MacfcrR {
        MacfcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MacfcrW) -> &mut MacfcrW
    {
        let mut w = MacfcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacfcrR {
    bits: u32,
}

impl MacfcrR {
    # [ doc = "Bit 0 - no description available" ]
    pub fn fcb(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn tfce(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn rfce(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn upfd(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:5 - no description available" ]
    pub fn plt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - no description available" ]
    pub fn zqpd(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:31 - no description available" ]
    pub fn pt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacfcrW {
    bits: u32,
}

impl MacfcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MacfcrW { bits: 0 }
    }
    # [ doc = "Bit 0 - no description available" ]
    pub fn fcb(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn tfce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn rfce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn upfd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:5 - no description available" ]
    pub fn plt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - no description available" ]
    pub fn zqpd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:31 - no description available" ]
    pub fn pt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Macvlantr {
    register: ::volatile_register::RW<u32>,
}

impl Macvlantr {
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
        where for<'w> F: FnOnce(&MacvlantrR, &'w mut MacvlantrW) -> &'w mut MacvlantrW
    {
        let bits = self.register.read();
        let r = MacvlantrR { bits: bits };
        let mut w = MacvlantrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MacvlantrR {
        MacvlantrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MacvlantrW) -> &mut MacvlantrW
    {
        let mut w = MacvlantrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacvlantrR {
    bits: u32,
}

impl MacvlantrR {
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn vlanti(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 16 - no description available" ]
    pub fn vlantc(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacvlantrW {
    bits: u32,
}

impl MacvlantrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MacvlantrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn vlanti(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - no description available" ]
    pub fn vlantc(&mut self, value: bool) -> &mut Self {
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
pub struct Macpmtcsr {
    register: ::volatile_register::RW<u32>,
}

impl Macpmtcsr {
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
        where for<'w> F: FnOnce(&MacpmtcsrR, &'w mut MacpmtcsrW) -> &'w mut MacpmtcsrW
    {
        let bits = self.register.read();
        let r = MacpmtcsrR { bits: bits };
        let mut w = MacpmtcsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MacpmtcsrR {
        MacpmtcsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MacpmtcsrW) -> &mut MacpmtcsrW
    {
        let mut w = MacpmtcsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacpmtcsrR {
    bits: u32,
}

impl MacpmtcsrR {
    # [ doc = "Bit 0 - no description available" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn mpe(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn wfe(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn mpr(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn wfr(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn gu(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn wffrpr(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacpmtcsrW {
    bits: u32,
}

impl MacpmtcsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MacpmtcsrW { bits: 0 }
    }
    # [ doc = "Bit 0 - no description available" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn mpe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn wfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn mpr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn wfr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn gu(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn wffrpr(&mut self, value: bool) -> &mut Self {
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
pub struct Macdbgr {
    register: ::volatile_register::RO<u32>,
}

impl Macdbgr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> MacdbgrR {
        MacdbgrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacdbgrR {
    bits: u32,
}

impl MacdbgrR {
    # [ doc = "Bit 0 - CR" ]
    pub fn cr(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - CSR" ]
    pub fn csr(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - ROR" ]
    pub fn ror(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - MCF" ]
    pub fn mcf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - MCP" ]
    pub fn mcp(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - MCFHP" ]
    pub fn mcfhp(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Macsr {
    register: ::volatile_register::RW<u32>,
}

impl Macsr {
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
        where for<'w> F: FnOnce(&MacsrR, &'w mut MacsrW) -> &'w mut MacsrW
    {
        let bits = self.register.read();
        let r = MacsrR { bits: bits };
        let mut w = MacsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MacsrR {
        MacsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MacsrW) -> &mut MacsrW
    {
        let mut w = MacsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacsrR {
    bits: u32,
}

impl MacsrR {
    # [ doc = "Bit 3 - no description available" ]
    pub fn pmts(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - no description available" ]
    pub fn mmcs(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - no description available" ]
    pub fn mmcrs(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn mmcts(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn tsts(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacsrW {
    bits: u32,
}

impl MacsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MacsrW { bits: 0 }
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn tsts(&mut self, value: bool) -> &mut Self {
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
pub struct Macimr {
    register: ::volatile_register::RW<u32>,
}

impl Macimr {
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
        where for<'w> F: FnOnce(&MacimrR, &'w mut MacimrW) -> &'w mut MacimrW
    {
        let bits = self.register.read();
        let r = MacimrR { bits: bits };
        let mut w = MacimrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MacimrR {
        MacimrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MacimrW) -> &mut MacimrW
    {
        let mut w = MacimrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacimrR {
    bits: u32,
}

impl MacimrR {
    # [ doc = "Bit 3 - no description available" ]
    pub fn pmtim(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn tstim(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MacimrW {
    bits: u32,
}

impl MacimrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MacimrW { bits: 0 }
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn pmtim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - no description available" ]
    pub fn tstim(&mut self, value: bool) -> &mut Self {
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
pub struct Maca0hr {
    register: ::volatile_register::RW<u32>,
}

impl Maca0hr {
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
        where for<'w> F: FnOnce(&Maca0hrR, &'w mut Maca0hrW) -> &'w mut Maca0hrW
    {
        let bits = self.register.read();
        let r = Maca0hrR { bits: bits };
        let mut w = Maca0hrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Maca0hrR {
        Maca0hrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Maca0hrW) -> &mut Maca0hrW
    {
        let mut w = Maca0hrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca0hrR {
    bits: u32,
}

impl Maca0hrR {
    # [ doc = "Bits 0:15 - MAC address0 high" ]
    pub fn maca0h(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 31 - Always 1" ]
    pub fn mo(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca0hrW {
    bits: u32,
}

impl Maca0hrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Maca0hrW { bits: 1114111 }
    }
    # [ doc = "Bits 0:15 - MAC address0 high" ]
    pub fn maca0h(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Maca0lr {
    register: ::volatile_register::RW<u32>,
}

impl Maca0lr {
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
        where for<'w> F: FnOnce(&Maca0lrR, &'w mut Maca0lrW) -> &'w mut Maca0lrW
    {
        let bits = self.register.read();
        let r = Maca0lrR { bits: bits };
        let mut w = Maca0lrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Maca0lrR {
        Maca0lrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Maca0lrW) -> &mut Maca0lrW
    {
        let mut w = Maca0lrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca0lrR {
    bits: u32,
}

impl Maca0lrR {
    # [ doc = "Bits 0:31 - 0" ]
    pub fn maca0l(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca0lrW {
    bits: u32,
}

impl Maca0lrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Maca0lrW { bits: 4294967295 }
    }
    # [ doc = "Bits 0:31 - 0" ]
    pub fn maca0l(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Maca1hr {
    register: ::volatile_register::RW<u32>,
}

impl Maca1hr {
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
        where for<'w> F: FnOnce(&Maca1hrR, &'w mut Maca1hrW) -> &'w mut Maca1hrW
    {
        let bits = self.register.read();
        let r = Maca1hrR { bits: bits };
        let mut w = Maca1hrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Maca1hrR {
        Maca1hrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Maca1hrW) -> &mut Maca1hrW
    {
        let mut w = Maca1hrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca1hrR {
    bits: u32,
}

impl Maca1hrR {
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn maca1h(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:29 - no description available" ]
    pub fn mbc(&self) -> u8 {
        const MASK: u32 = 63;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - no description available" ]
    pub fn sa(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn ae(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca1hrW {
    bits: u32,
}

impl Maca1hrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Maca1hrW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn maca1h(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:29 - no description available" ]
    pub fn mbc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 30 - no description available" ]
    pub fn sa(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn ae(&mut self, value: bool) -> &mut Self {
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
pub struct Maca1lr {
    register: ::volatile_register::RW<u32>,
}

impl Maca1lr {
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
        where for<'w> F: FnOnce(&Maca1lrR, &'w mut Maca1lrW) -> &'w mut Maca1lrW
    {
        let bits = self.register.read();
        let r = Maca1lrR { bits: bits };
        let mut w = Maca1lrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Maca1lrR {
        Maca1lrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Maca1lrW) -> &mut Maca1lrW
    {
        let mut w = Maca1lrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca1lrR {
    bits: u32,
}

impl Maca1lrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn maca1lr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca1lrW {
    bits: u32,
}

impl Maca1lrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Maca1lrW { bits: 4294967295 }
    }
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn maca1lr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Maca2hr {
    register: ::volatile_register::RW<u32>,
}

impl Maca2hr {
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
        where for<'w> F: FnOnce(&Maca2hrR, &'w mut Maca2hrW) -> &'w mut Maca2hrW
    {
        let bits = self.register.read();
        let r = Maca2hrR { bits: bits };
        let mut w = Maca2hrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Maca2hrR {
        Maca2hrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Maca2hrW) -> &mut Maca2hrW
    {
        let mut w = Maca2hrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca2hrR {
    bits: u32,
}

impl Maca2hrR {
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn mac2ah(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:29 - no description available" ]
    pub fn mbc(&self) -> u8 {
        const MASK: u32 = 63;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - no description available" ]
    pub fn sa(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn ae(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca2hrW {
    bits: u32,
}

impl Maca2hrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Maca2hrW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn mac2ah(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:29 - no description available" ]
    pub fn mbc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 30 - no description available" ]
    pub fn sa(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn ae(&mut self, value: bool) -> &mut Self {
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
pub struct Maca2lr {
    register: ::volatile_register::RW<u32>,
}

impl Maca2lr {
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
        where for<'w> F: FnOnce(&Maca2lrR, &'w mut Maca2lrW) -> &'w mut Maca2lrW
    {
        let bits = self.register.read();
        let r = Maca2lrR { bits: bits };
        let mut w = Maca2lrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Maca2lrR {
        Maca2lrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Maca2lrW) -> &mut Maca2lrW
    {
        let mut w = Maca2lrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca2lrR {
    bits: u32,
}

impl Maca2lrR {
    # [ doc = "Bits 0:30 - no description available" ]
    pub fn maca2l(&self) -> u32 {
        const MASK: u32 = 2147483647;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca2lrW {
    bits: u32,
}

impl Maca2lrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Maca2lrW { bits: 4294967295 }
    }
    # [ doc = "Bits 0:30 - no description available" ]
    pub fn maca2l(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 2147483647;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Maca3hr {
    register: ::volatile_register::RW<u32>,
}

impl Maca3hr {
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
        where for<'w> F: FnOnce(&Maca3hrR, &'w mut Maca3hrW) -> &'w mut Maca3hrW
    {
        let bits = self.register.read();
        let r = Maca3hrR { bits: bits };
        let mut w = Maca3hrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Maca3hrR {
        Maca3hrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Maca3hrW) -> &mut Maca3hrW
    {
        let mut w = Maca3hrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca3hrR {
    bits: u32,
}

impl Maca3hrR {
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn maca3h(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:29 - no description available" ]
    pub fn mbc(&self) -> u8 {
        const MASK: u32 = 63;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - no description available" ]
    pub fn sa(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn ae(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca3hrW {
    bits: u32,
}

impl Maca3hrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Maca3hrW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - no description available" ]
    pub fn maca3h(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:29 - no description available" ]
    pub fn mbc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 30 - no description available" ]
    pub fn sa(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - no description available" ]
    pub fn ae(&mut self, value: bool) -> &mut Self {
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
pub struct Maca3lr {
    register: ::volatile_register::RW<u32>,
}

impl Maca3lr {
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
        where for<'w> F: FnOnce(&Maca3lrR, &'w mut Maca3lrW) -> &'w mut Maca3lrW
    {
        let bits = self.register.read();
        let r = Maca3lrR { bits: bits };
        let mut w = Maca3lrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Maca3lrR {
        Maca3lrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Maca3lrW) -> &mut Maca3lrW
    {
        let mut w = Maca3lrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca3lrR {
    bits: u32,
}

impl Maca3lrR {
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn mbca3l(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Maca3lrW {
    bits: u32,
}

impl Maca3lrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Maca3lrW { bits: 4294967295 }
    }
    # [ doc = "Bits 0:31 - no description available" ]
    pub fn mbca3l(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
