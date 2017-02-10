# [ doc = "Real-time clock" ]
# [ repr ( C ) ]
pub struct Rtc {
    # [ doc = "0x00 - time register" ]
    pub tr: Tr,
    # [ doc = "0x04 - date register" ]
    pub dr: Dr,
    # [ doc = "0x08 - control register" ]
    pub cr: Cr,
    # [ doc = "0x0c - initialization and status register" ]
    pub isr: Isr,
    # [ doc = "0x10 - prescaler register" ]
    pub prer: Prer,
    # [ doc = "0x14 - wakeup timer register" ]
    pub wutr: Wutr,
    # [ doc = "0x18 - calibration register" ]
    pub calibr: Calibr,
    # [ doc = "0x1c - alarm A register" ]
    pub alrmar: Alrmar,
    # [ doc = "0x20 - alarm B register" ]
    pub alrmbr: Alrmbr,
    # [ doc = "0x24 - write protection register" ]
    pub wpr: Wpr,
    # [ doc = "0x28 - sub second register" ]
    pub ssr: Ssr,
    # [ doc = "0x2c - shift control register" ]
    pub shiftr: Shiftr,
    # [ doc = "0x30 - time stamp time register" ]
    pub tstr: Tstr,
    # [ doc = "0x34 - time stamp date register" ]
    pub tsdr: Tsdr,
    # [ doc = "0x38 - timestamp sub second register" ]
    pub tsssr: Tsssr,
    # [ doc = "0x3c - calibration register" ]
    pub calr: Calr,
    # [ doc = "0x40 - tamper and alternate function configuration register" ]
    pub tafcr: Tafcr,
    # [ doc = "0x44 - alarm A sub second register" ]
    pub alrmassr: Alrmassr,
    # [ doc = "0x48 - alarm B sub second register" ]
    pub alrmbssr: Alrmbssr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x50 - backup register" ]
    pub bkp0r: Bkp0r,
    # [ doc = "0x54 - backup register" ]
    pub bkp1r: Bkp1r,
    # [ doc = "0x58 - backup register" ]
    pub bkp2r: Bkp2r,
    # [ doc = "0x5c - backup register" ]
    pub bkp3r: Bkp3r,
    # [ doc = "0x60 - backup register" ]
    pub bkp4r: Bkp4r,
    # [ doc = "0x64 - backup register" ]
    pub bkp5r: Bkp5r,
    # [ doc = "0x68 - backup register" ]
    pub bkp6r: Bkp6r,
    # [ doc = "0x6c - backup register" ]
    pub bkp7r: Bkp7r,
    # [ doc = "0x70 - backup register" ]
    pub bkp8r: Bkp8r,
    # [ doc = "0x74 - backup register" ]
    pub bkp9r: Bkp9r,
    # [ doc = "0x78 - backup register" ]
    pub bkp10r: Bkp10r,
    # [ doc = "0x7c - backup register" ]
    pub bkp11r: Bkp11r,
    # [ doc = "0x80 - backup register" ]
    pub bkp12r: Bkp12r,
    # [ doc = "0x84 - backup register" ]
    pub bkp13r: Bkp13r,
    # [ doc = "0x88 - backup register" ]
    pub bkp14r: Bkp14r,
    # [ doc = "0x8c - backup register" ]
    pub bkp15r: Bkp15r,
    # [ doc = "0x90 - backup register" ]
    pub bkp16r: Bkp16r,
    # [ doc = "0x94 - backup register" ]
    pub bkp17r: Bkp17r,
    # [ doc = "0x98 - backup register" ]
    pub bkp18r: Bkp18r,
    # [ doc = "0x9c - backup register" ]
    pub bkp19r: Bkp19r,
}

# [ repr ( C ) ]
pub struct Tr {
    register: ::volatile_register::RW<u32>,
}

impl Tr {
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
        where for<'w> F: FnOnce(&TrR, &'w mut TrW) -> &'w mut TrW
    {
        let bits = self.register.read();
        let r = TrR { bits: bits };
        let mut w = TrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> TrR {
        TrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut TrW) -> &mut TrW
    {
        let mut w = TrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TrR {
    bits: u32,
}

impl TrR {
    # [ doc = "Bit 22 - AM/PM notation" ]
    pub fn pm(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 20:21 - Hour tens in BCD format" ]
    pub fn ht(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Hour units in BCD format" ]
    pub fn hu(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:14 - Minute tens in BCD format" ]
    pub fn mnt(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Minute units in BCD format" ]
    pub fn mnu(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:6 - Second tens in BCD format" ]
    pub fn st(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - Second units in BCD format" ]
    pub fn su(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TrW {
    bits: u32,
}

impl TrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        TrW { bits: 0 }
    }
    # [ doc = "Bit 22 - AM/PM notation" ]
    pub fn pm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 20:21 - Hour tens in BCD format" ]
    pub fn ht(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Hour units in BCD format" ]
    pub fn hu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:14 - Minute tens in BCD format" ]
    pub fn mnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Minute units in BCD format" ]
    pub fn mnu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:6 - Second tens in BCD format" ]
    pub fn st(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Second units in BCD format" ]
    pub fn su(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dr {
    register: ::volatile_register::RW<u32>,
}

impl Dr {
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
        where for<'w> F: FnOnce(&DrR, &'w mut DrW) -> &'w mut DrW
    {
        let bits = self.register.read();
        let r = DrR { bits: bits };
        let mut w = DrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DrR {
        DrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DrW) -> &mut DrW
    {
        let mut w = DrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DrR {
    bits: u32,
}

impl DrR {
    # [ doc = "Bits 20:23 - Year tens in BCD format" ]
    pub fn yt(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Year units in BCD format" ]
    pub fn yu(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 13:15 - Week day units" ]
    pub fn wdu(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 12 - Month tens in BCD format" ]
    pub fn mt(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:11 - Month units in BCD format" ]
    pub fn mu(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:5 - Date tens in BCD format" ]
    pub fn dt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - Date units in BCD format" ]
    pub fn du(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DrW {
    bits: u32,
}

impl DrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DrW { bits: 8449 }
    }
    # [ doc = "Bits 20:23 - Year tens in BCD format" ]
    pub fn yt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Year units in BCD format" ]
    pub fn yu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 13:15 - Week day units" ]
    pub fn wdu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 12 - Month tens in BCD format" ]
    pub fn mt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:11 - Month units in BCD format" ]
    pub fn mu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:5 - Date tens in BCD format" ]
    pub fn dt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Date units in BCD format" ]
    pub fn du(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cr {
    register: ::volatile_register::RW<u32>,
}

impl Cr {
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
        where for<'w> F: FnOnce(&CrR, &'w mut CrW) -> &'w mut CrW
    {
        let bits = self.register.read();
        let r = CrR { bits: bits };
        let mut w = CrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CrR {
        CrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CrW) -> &mut CrW
    {
        let mut w = CrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrR {
    bits: u32,
}

impl CrR {
    # [ doc = "Bit 23 - Calibration output enable" ]
    pub fn coe(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 21:22 - Output selection" ]
    pub fn osel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - Output polarity" ]
    pub fn pol(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Backup" ]
    pub fn bkp(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Subtract 1 hour (winter time change)" ]
    pub fn sub1h(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Add 1 hour (summer time change)" ]
    pub fn add1h(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Time-stamp interrupt enable" ]
    pub fn tsie(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Wakeup timer interrupt enable" ]
    pub fn wutie(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Alarm B interrupt enable" ]
    pub fn alrbie(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Alarm A interrupt enable" ]
    pub fn alraie(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Time stamp enable" ]
    pub fn tse(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Wakeup timer enable" ]
    pub fn wute(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Alarm B enable" ]
    pub fn alrbe(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Alarm A enable" ]
    pub fn alrae(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Coarse digital calibration enable" ]
    pub fn dce(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Hour format" ]
    pub fn fmt(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)" ]
    pub fn refckon(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Time-stamp event active edge" ]
    pub fn tsedge(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:2 - Wakeup clock selection" ]
    pub fn wcksel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrW {
    bits: u32,
}

impl CrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CrW { bits: 0 }
    }
    # [ doc = "Bit 23 - Calibration output enable" ]
    pub fn coe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 21:22 - Output selection" ]
    pub fn osel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - Output polarity" ]
    pub fn pol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Backup" ]
    pub fn bkp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Subtract 1 hour (winter time change)" ]
    pub fn sub1h(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Add 1 hour (summer time change)" ]
    pub fn add1h(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Time-stamp interrupt enable" ]
    pub fn tsie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Wakeup timer interrupt enable" ]
    pub fn wutie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Alarm B interrupt enable" ]
    pub fn alrbie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Alarm A interrupt enable" ]
    pub fn alraie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Time stamp enable" ]
    pub fn tse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Wakeup timer enable" ]
    pub fn wute(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Alarm B enable" ]
    pub fn alrbe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Alarm A enable" ]
    pub fn alrae(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Coarse digital calibration enable" ]
    pub fn dce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Hour format" ]
    pub fn fmt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)" ]
    pub fn refckon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Time-stamp event active edge" ]
    pub fn tsedge(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:2 - Wakeup clock selection" ]
    pub fn wcksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Isr {
    register: ::volatile_register::RW<u32>,
}

impl Isr {
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
        where for<'w> F: FnOnce(&IsrR, &'w mut IsrW) -> &'w mut IsrW
    {
        let bits = self.register.read();
        let r = IsrR { bits: bits };
        let mut w = IsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IsrR {
        IsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IsrW) -> &mut IsrW
    {
        let mut w = IsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IsrR {
    bits: u32,
}

impl IsrR {
    # [ doc = "Bit 0 - Alarm A write flag" ]
    pub fn alrawf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Alarm B write flag" ]
    pub fn alrbwf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wakeup timer write flag" ]
    pub fn wutwf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Shift operation pending" ]
    pub fn shpf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Initialization status flag" ]
    pub fn inits(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Registers synchronization flag" ]
    pub fn rsf(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Initialization flag" ]
    pub fn initf(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Initialization mode" ]
    pub fn init(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Alarm A flag" ]
    pub fn alraf(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Alarm B flag" ]
    pub fn alrbf(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Wakeup timer flag" ]
    pub fn wutf(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Time-stamp flag" ]
    pub fn tsf(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Time-stamp overflow flag" ]
    pub fn tsovf(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Tamper detection flag" ]
    pub fn tamp1f(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - TAMPER2 detection flag" ]
    pub fn tamp2f(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Recalibration pending Flag" ]
    pub fn recalpf(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IsrW {
    bits: u32,
}

impl IsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IsrW { bits: 7 }
    }
    # [ doc = "Bit 3 - Shift operation pending" ]
    pub fn shpf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Registers synchronization flag" ]
    pub fn rsf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Initialization mode" ]
    pub fn init(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Alarm A flag" ]
    pub fn alraf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Alarm B flag" ]
    pub fn alrbf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Wakeup timer flag" ]
    pub fn wutf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Time-stamp flag" ]
    pub fn tsf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Time-stamp overflow flag" ]
    pub fn tsovf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Tamper detection flag" ]
    pub fn tamp1f(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - TAMPER2 detection flag" ]
    pub fn tamp2f(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Prer {
    register: ::volatile_register::RW<u32>,
}

impl Prer {
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
        where for<'w> F: FnOnce(&PrerR, &'w mut PrerW) -> &'w mut PrerW
    {
        let bits = self.register.read();
        let r = PrerR { bits: bits };
        let mut w = PrerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PrerR {
        PrerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PrerW) -> &mut PrerW
    {
        let mut w = PrerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PrerR {
    bits: u32,
}

impl PrerR {
    # [ doc = "Bits 16:22 - Asynchronous prescaler factor" ]
    pub fn prediv_a(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:14 - Synchronous prescaler factor" ]
    pub fn prediv_s(&self) -> u16 {
        const MASK: u32 = 32767;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PrerW {
    bits: u32,
}

impl PrerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PrerW { bits: 8323327 }
    }
    # [ doc = "Bits 16:22 - Asynchronous prescaler factor" ]
    pub fn prediv_a(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:14 - Synchronous prescaler factor" ]
    pub fn prediv_s(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 32767;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Wutr {
    register: ::volatile_register::RW<u32>,
}

impl Wutr {
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
        where for<'w> F: FnOnce(&WutrR, &'w mut WutrW) -> &'w mut WutrW
    {
        let bits = self.register.read();
        let r = WutrR { bits: bits };
        let mut w = WutrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> WutrR {
        WutrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut WutrW) -> &mut WutrW
    {
        let mut w = WutrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct WutrR {
    bits: u32,
}

impl WutrR {
    # [ doc = "Bits 0:15 - Wakeup auto-reload value bits" ]
    pub fn wut(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct WutrW {
    bits: u32,
}

impl WutrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        WutrW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - Wakeup auto-reload value bits" ]
    pub fn wut(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Calibr {
    register: ::volatile_register::RW<u32>,
}

impl Calibr {
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
        where for<'w> F: FnOnce(&CalibrR, &'w mut CalibrW) -> &'w mut CalibrW
    {
        let bits = self.register.read();
        let r = CalibrR { bits: bits };
        let mut w = CalibrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CalibrR {
        CalibrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CalibrW) -> &mut CalibrW
    {
        let mut w = CalibrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CalibrR {
    bits: u32,
}

impl CalibrR {
    # [ doc = "Bit 7 - Digital calibration sign" ]
    pub fn dcs(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:4 - Digital calibration" ]
    pub fn dc(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CalibrW {
    bits: u32,
}

impl CalibrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CalibrW { bits: 0 }
    }
    # [ doc = "Bit 7 - Digital calibration sign" ]
    pub fn dcs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:4 - Digital calibration" ]
    pub fn dc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Alrmar {
    register: ::volatile_register::RW<u32>,
}

impl Alrmar {
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
        where for<'w> F: FnOnce(&AlrmarR, &'w mut AlrmarW) -> &'w mut AlrmarW
    {
        let bits = self.register.read();
        let r = AlrmarR { bits: bits };
        let mut w = AlrmarW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AlrmarR {
        AlrmarR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AlrmarW) -> &mut AlrmarW
    {
        let mut w = AlrmarW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AlrmarR {
    bits: u32,
}

impl AlrmarR {
    # [ doc = "Bit 31 - Alarm A date mask" ]
    pub fn msk4(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Week day selection" ]
    pub fn wdsel(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 28:29 - Date tens in BCD format" ]
    pub fn dt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Date units or day in BCD format" ]
    pub fn du(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 23 - Alarm A hours mask" ]
    pub fn msk3(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - AM/PM notation" ]
    pub fn pm(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 20:21 - Hour tens in BCD format" ]
    pub fn ht(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Hour units in BCD format" ]
    pub fn hu(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Alarm A minutes mask" ]
    pub fn msk2(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:14 - Minute tens in BCD format" ]
    pub fn mnt(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Minute units in BCD format" ]
    pub fn mnu(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Alarm A seconds mask" ]
    pub fn msk1(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Second tens in BCD format" ]
    pub fn st(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - Second units in BCD format" ]
    pub fn su(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AlrmarW {
    bits: u32,
}

impl AlrmarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AlrmarW { bits: 0 }
    }
    # [ doc = "Bit 31 - Alarm A date mask" ]
    pub fn msk4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Week day selection" ]
    pub fn wdsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 28:29 - Date tens in BCD format" ]
    pub fn dt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Date units or day in BCD format" ]
    pub fn du(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Alarm A hours mask" ]
    pub fn msk3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - AM/PM notation" ]
    pub fn pm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 20:21 - Hour tens in BCD format" ]
    pub fn ht(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Hour units in BCD format" ]
    pub fn hu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Alarm A minutes mask" ]
    pub fn msk2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:14 - Minute tens in BCD format" ]
    pub fn mnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Minute units in BCD format" ]
    pub fn mnu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Alarm A seconds mask" ]
    pub fn msk1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Second tens in BCD format" ]
    pub fn st(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Second units in BCD format" ]
    pub fn su(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Alrmbr {
    register: ::volatile_register::RW<u32>,
}

impl Alrmbr {
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
        where for<'w> F: FnOnce(&AlrmbrR, &'w mut AlrmbrW) -> &'w mut AlrmbrW
    {
        let bits = self.register.read();
        let r = AlrmbrR { bits: bits };
        let mut w = AlrmbrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AlrmbrR {
        AlrmbrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AlrmbrW) -> &mut AlrmbrW
    {
        let mut w = AlrmbrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AlrmbrR {
    bits: u32,
}

impl AlrmbrR {
    # [ doc = "Bit 31 - Alarm B date mask" ]
    pub fn msk4(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Week day selection" ]
    pub fn wdsel(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 28:29 - Date tens in BCD format" ]
    pub fn dt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Date units or day in BCD format" ]
    pub fn du(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 23 - Alarm B hours mask" ]
    pub fn msk3(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - AM/PM notation" ]
    pub fn pm(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 20:21 - Hour tens in BCD format" ]
    pub fn ht(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Hour units in BCD format" ]
    pub fn hu(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Alarm B minutes mask" ]
    pub fn msk2(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:14 - Minute tens in BCD format" ]
    pub fn mnt(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Minute units in BCD format" ]
    pub fn mnu(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Alarm B seconds mask" ]
    pub fn msk1(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Second tens in BCD format" ]
    pub fn st(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - Second units in BCD format" ]
    pub fn su(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AlrmbrW {
    bits: u32,
}

impl AlrmbrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AlrmbrW { bits: 0 }
    }
    # [ doc = "Bit 31 - Alarm B date mask" ]
    pub fn msk4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Week day selection" ]
    pub fn wdsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 28:29 - Date tens in BCD format" ]
    pub fn dt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Date units or day in BCD format" ]
    pub fn du(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Alarm B hours mask" ]
    pub fn msk3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - AM/PM notation" ]
    pub fn pm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 20:21 - Hour tens in BCD format" ]
    pub fn ht(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Hour units in BCD format" ]
    pub fn hu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Alarm B minutes mask" ]
    pub fn msk2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:14 - Minute tens in BCD format" ]
    pub fn mnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Minute units in BCD format" ]
    pub fn mnu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Alarm B seconds mask" ]
    pub fn msk1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Second tens in BCD format" ]
    pub fn st(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Second units in BCD format" ]
    pub fn su(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Wpr {
    register: ::volatile_register::WO<u32>,
}

impl Wpr {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut WprW) -> &mut WprW
    {
        let mut w = WprW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct WprW {
    bits: u32,
}

impl WprW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        WprW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Write protection key" ]
    pub fn key(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ssr {
    register: ::volatile_register::RO<u32>,
}

impl Ssr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> SsrR {
        SsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SsrR {
    bits: u32,
}

impl SsrR {
    # [ doc = "Bits 0:15 - Sub second value" ]
    pub fn ss(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct Shiftr {
    register: ::volatile_register::WO<u32>,
}

impl Shiftr {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut ShiftrW) -> &mut ShiftrW
    {
        let mut w = ShiftrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ShiftrW {
    bits: u32,
}

impl ShiftrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ShiftrW { bits: 0 }
    }
    # [ doc = "Bit 31 - Add one second" ]
    pub fn add1s(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:14 - Subtract a fraction of a second" ]
    pub fn subfs(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 32767;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tstr {
    register: ::volatile_register::RO<u32>,
}

impl Tstr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> TstrR {
        TstrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TstrR {
    bits: u32,
}

impl TstrR {
    # [ doc = "Bit 18 - AFO_ALARM output type" ]
    pub fn alarmouttype(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIMESTAMP mapping" ]
    pub fn tsinsel(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TAMPER1 mapping" ]
    pub fn tamp1insel(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Tamper interrupt enable" ]
    pub fn tampie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Active level for tamper 1" ]
    pub fn tamp1trg(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Tamper 1 detection enable" ]
    pub fn tamp1e(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Tsdr {
    register: ::volatile_register::RO<u32>,
}

impl Tsdr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> TsdrR {
        TsdrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TsdrR {
    bits: u32,
}

impl TsdrR {
    # [ doc = "Bits 13:15 - Week day units" ]
    pub fn wdu(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 12 - Month tens in BCD format" ]
    pub fn mt(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:11 - Month units in BCD format" ]
    pub fn mu(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:5 - Date tens in BCD format" ]
    pub fn dt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - Date units in BCD format" ]
    pub fn du(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Tsssr {
    register: ::volatile_register::RO<u32>,
}

impl Tsssr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> TsssrR {
        TsssrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TsssrR {
    bits: u32,
}

impl TsssrR {
    # [ doc = "Bits 0:15 - Sub second value" ]
    pub fn ss(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct Calr {
    register: ::volatile_register::RW<u32>,
}

impl Calr {
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
        where for<'w> F: FnOnce(&CalrR, &'w mut CalrW) -> &'w mut CalrW
    {
        let bits = self.register.read();
        let r = CalrR { bits: bits };
        let mut w = CalrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CalrR {
        CalrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CalrW) -> &mut CalrW
    {
        let mut w = CalrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CalrR {
    bits: u32,
}

impl CalrR {
    # [ doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm" ]
    pub fn calp(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Use an 8-second calibration cycle period" ]
    pub fn calw8(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Use a 16-second calibration cycle period" ]
    pub fn calw16(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:8 - Calibration minus" ]
    pub fn calm(&self) -> u16 {
        const MASK: u32 = 511;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CalrW {
    bits: u32,
}

impl CalrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CalrW { bits: 0 }
    }
    # [ doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm" ]
    pub fn calp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Use an 8-second calibration cycle period" ]
    pub fn calw8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Use a 16-second calibration cycle period" ]
    pub fn calw16(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:8 - Calibration minus" ]
    pub fn calm(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 511;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tafcr {
    register: ::volatile_register::RW<u32>,
}

impl Tafcr {
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
        where for<'w> F: FnOnce(&TafcrR, &'w mut TafcrW) -> &'w mut TafcrW
    {
        let bits = self.register.read();
        let r = TafcrR { bits: bits };
        let mut w = TafcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> TafcrR {
        TafcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut TafcrW) -> &mut TafcrW
    {
        let mut w = TafcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TafcrR {
    bits: u32,
}

impl TafcrR {
    # [ doc = "Bit 18 - AFO_ALARM output type" ]
    pub fn alarmouttype(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIMESTAMP mapping" ]
    pub fn tsinsel(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TAMPER1 mapping" ]
    pub fn tamp1insel(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - TAMPER pull-up disable" ]
    pub fn tamppudis(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Tamper precharge duration" ]
    pub fn tampprch(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Tamper filter count" ]
    pub fn tampflt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - Tamper sampling frequency" ]
    pub fn tampfreq(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Activate timestamp on tamper detection event" ]
    pub fn tampts(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Active level for tamper 2" ]
    pub fn tamp2trg(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Tamper 2 detection enable" ]
    pub fn tamp2e(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Tamper interrupt enable" ]
    pub fn tampie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Active level for tamper 1" ]
    pub fn tamp1trg(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Tamper 1 detection enable" ]
    pub fn tamp1e(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TafcrW {
    bits: u32,
}

impl TafcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        TafcrW { bits: 0 }
    }
    # [ doc = "Bit 18 - AFO_ALARM output type" ]
    pub fn alarmouttype(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - TIMESTAMP mapping" ]
    pub fn tsinsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - TAMPER1 mapping" ]
    pub fn tamp1insel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - TAMPER pull-up disable" ]
    pub fn tamppudis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Tamper precharge duration" ]
    pub fn tampprch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Tamper filter count" ]
    pub fn tampflt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - Tamper sampling frequency" ]
    pub fn tampfreq(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Activate timestamp on tamper detection event" ]
    pub fn tampts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Active level for tamper 2" ]
    pub fn tamp2trg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Tamper 2 detection enable" ]
    pub fn tamp2e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Tamper interrupt enable" ]
    pub fn tampie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Active level for tamper 1" ]
    pub fn tamp1trg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Tamper 1 detection enable" ]
    pub fn tamp1e(&mut self, value: bool) -> &mut Self {
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
pub struct Alrmassr {
    register: ::volatile_register::RW<u32>,
}

impl Alrmassr {
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
        where for<'w> F: FnOnce(&AlrmassrR, &'w mut AlrmassrW) -> &'w mut AlrmassrW
    {
        let bits = self.register.read();
        let r = AlrmassrR { bits: bits };
        let mut w = AlrmassrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AlrmassrR {
        AlrmassrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AlrmassrW) -> &mut AlrmassrW
    {
        let mut w = AlrmassrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AlrmassrR {
    bits: u32,
}

impl AlrmassrR {
    # [ doc = "Bits 24:27 - Mask the most-significant bits starting at this bit" ]
    pub fn maskss(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:14 - Sub seconds value" ]
    pub fn ss(&self) -> u16 {
        const MASK: u32 = 32767;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AlrmassrW {
    bits: u32,
}

impl AlrmassrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AlrmassrW { bits: 0 }
    }
    # [ doc = "Bits 24:27 - Mask the most-significant bits starting at this bit" ]
    pub fn maskss(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:14 - Sub seconds value" ]
    pub fn ss(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 32767;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Alrmbssr {
    register: ::volatile_register::RW<u32>,
}

impl Alrmbssr {
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
        where for<'w> F: FnOnce(&AlrmbssrR, &'w mut AlrmbssrW) -> &'w mut AlrmbssrW
    {
        let bits = self.register.read();
        let r = AlrmbssrR { bits: bits };
        let mut w = AlrmbssrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AlrmbssrR {
        AlrmbssrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AlrmbssrW) -> &mut AlrmbssrW
    {
        let mut w = AlrmbssrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AlrmbssrR {
    bits: u32,
}

impl AlrmbssrR {
    # [ doc = "Bits 24:27 - Mask the most-significant bits starting at this bit" ]
    pub fn maskss(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:14 - Sub seconds value" ]
    pub fn ss(&self) -> u16 {
        const MASK: u32 = 32767;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AlrmbssrW {
    bits: u32,
}

impl AlrmbssrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AlrmbssrW { bits: 0 }
    }
    # [ doc = "Bits 24:27 - Mask the most-significant bits starting at this bit" ]
    pub fn maskss(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:14 - Sub seconds value" ]
    pub fn ss(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 32767;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp0r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp0r {
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
        where for<'w> F: FnOnce(&Bkp0rR, &'w mut Bkp0rW) -> &'w mut Bkp0rW
    {
        let bits = self.register.read();
        let r = Bkp0rR { bits: bits };
        let mut w = Bkp0rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp0rR {
        Bkp0rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp0rW) -> &mut Bkp0rW
    {
        let mut w = Bkp0rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp0rR {
    bits: u32,
}

impl Bkp0rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp0rW {
    bits: u32,
}

impl Bkp0rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp0rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp1r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp1r {
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
        where for<'w> F: FnOnce(&Bkp1rR, &'w mut Bkp1rW) -> &'w mut Bkp1rW
    {
        let bits = self.register.read();
        let r = Bkp1rR { bits: bits };
        let mut w = Bkp1rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp1rR {
        Bkp1rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp1rW) -> &mut Bkp1rW
    {
        let mut w = Bkp1rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp1rR {
    bits: u32,
}

impl Bkp1rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp1rW {
    bits: u32,
}

impl Bkp1rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp1rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp2r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp2r {
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
        where for<'w> F: FnOnce(&Bkp2rR, &'w mut Bkp2rW) -> &'w mut Bkp2rW
    {
        let bits = self.register.read();
        let r = Bkp2rR { bits: bits };
        let mut w = Bkp2rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp2rR {
        Bkp2rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp2rW) -> &mut Bkp2rW
    {
        let mut w = Bkp2rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp2rR {
    bits: u32,
}

impl Bkp2rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp2rW {
    bits: u32,
}

impl Bkp2rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp2rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp3r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp3r {
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
        where for<'w> F: FnOnce(&Bkp3rR, &'w mut Bkp3rW) -> &'w mut Bkp3rW
    {
        let bits = self.register.read();
        let r = Bkp3rR { bits: bits };
        let mut w = Bkp3rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp3rR {
        Bkp3rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp3rW) -> &mut Bkp3rW
    {
        let mut w = Bkp3rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp3rR {
    bits: u32,
}

impl Bkp3rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp3rW {
    bits: u32,
}

impl Bkp3rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp3rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp4r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp4r {
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
        where for<'w> F: FnOnce(&Bkp4rR, &'w mut Bkp4rW) -> &'w mut Bkp4rW
    {
        let bits = self.register.read();
        let r = Bkp4rR { bits: bits };
        let mut w = Bkp4rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp4rR {
        Bkp4rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp4rW) -> &mut Bkp4rW
    {
        let mut w = Bkp4rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp4rR {
    bits: u32,
}

impl Bkp4rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp4rW {
    bits: u32,
}

impl Bkp4rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp4rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp5r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp5r {
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
        where for<'w> F: FnOnce(&Bkp5rR, &'w mut Bkp5rW) -> &'w mut Bkp5rW
    {
        let bits = self.register.read();
        let r = Bkp5rR { bits: bits };
        let mut w = Bkp5rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp5rR {
        Bkp5rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp5rW) -> &mut Bkp5rW
    {
        let mut w = Bkp5rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp5rR {
    bits: u32,
}

impl Bkp5rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp5rW {
    bits: u32,
}

impl Bkp5rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp5rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp6r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp6r {
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
        where for<'w> F: FnOnce(&Bkp6rR, &'w mut Bkp6rW) -> &'w mut Bkp6rW
    {
        let bits = self.register.read();
        let r = Bkp6rR { bits: bits };
        let mut w = Bkp6rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp6rR {
        Bkp6rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp6rW) -> &mut Bkp6rW
    {
        let mut w = Bkp6rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp6rR {
    bits: u32,
}

impl Bkp6rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp6rW {
    bits: u32,
}

impl Bkp6rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp6rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp7r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp7r {
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
        where for<'w> F: FnOnce(&Bkp7rR, &'w mut Bkp7rW) -> &'w mut Bkp7rW
    {
        let bits = self.register.read();
        let r = Bkp7rR { bits: bits };
        let mut w = Bkp7rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp7rR {
        Bkp7rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp7rW) -> &mut Bkp7rW
    {
        let mut w = Bkp7rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp7rR {
    bits: u32,
}

impl Bkp7rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp7rW {
    bits: u32,
}

impl Bkp7rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp7rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp8r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp8r {
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
        where for<'w> F: FnOnce(&Bkp8rR, &'w mut Bkp8rW) -> &'w mut Bkp8rW
    {
        let bits = self.register.read();
        let r = Bkp8rR { bits: bits };
        let mut w = Bkp8rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp8rR {
        Bkp8rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp8rW) -> &mut Bkp8rW
    {
        let mut w = Bkp8rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp8rR {
    bits: u32,
}

impl Bkp8rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp8rW {
    bits: u32,
}

impl Bkp8rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp8rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp9r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp9r {
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
        where for<'w> F: FnOnce(&Bkp9rR, &'w mut Bkp9rW) -> &'w mut Bkp9rW
    {
        let bits = self.register.read();
        let r = Bkp9rR { bits: bits };
        let mut w = Bkp9rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp9rR {
        Bkp9rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp9rW) -> &mut Bkp9rW
    {
        let mut w = Bkp9rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp9rR {
    bits: u32,
}

impl Bkp9rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp9rW {
    bits: u32,
}

impl Bkp9rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp9rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp10r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp10r {
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
        where for<'w> F: FnOnce(&Bkp10rR, &'w mut Bkp10rW) -> &'w mut Bkp10rW
    {
        let bits = self.register.read();
        let r = Bkp10rR { bits: bits };
        let mut w = Bkp10rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp10rR {
        Bkp10rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp10rW) -> &mut Bkp10rW
    {
        let mut w = Bkp10rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp10rR {
    bits: u32,
}

impl Bkp10rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp10rW {
    bits: u32,
}

impl Bkp10rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp10rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp11r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp11r {
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
        where for<'w> F: FnOnce(&Bkp11rR, &'w mut Bkp11rW) -> &'w mut Bkp11rW
    {
        let bits = self.register.read();
        let r = Bkp11rR { bits: bits };
        let mut w = Bkp11rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp11rR {
        Bkp11rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp11rW) -> &mut Bkp11rW
    {
        let mut w = Bkp11rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp11rR {
    bits: u32,
}

impl Bkp11rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp11rW {
    bits: u32,
}

impl Bkp11rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp11rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp12r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp12r {
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
        where for<'w> F: FnOnce(&Bkp12rR, &'w mut Bkp12rW) -> &'w mut Bkp12rW
    {
        let bits = self.register.read();
        let r = Bkp12rR { bits: bits };
        let mut w = Bkp12rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp12rR {
        Bkp12rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp12rW) -> &mut Bkp12rW
    {
        let mut w = Bkp12rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp12rR {
    bits: u32,
}

impl Bkp12rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp12rW {
    bits: u32,
}

impl Bkp12rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp12rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp13r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp13r {
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
        where for<'w> F: FnOnce(&Bkp13rR, &'w mut Bkp13rW) -> &'w mut Bkp13rW
    {
        let bits = self.register.read();
        let r = Bkp13rR { bits: bits };
        let mut w = Bkp13rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp13rR {
        Bkp13rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp13rW) -> &mut Bkp13rW
    {
        let mut w = Bkp13rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp13rR {
    bits: u32,
}

impl Bkp13rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp13rW {
    bits: u32,
}

impl Bkp13rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp13rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp14r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp14r {
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
        where for<'w> F: FnOnce(&Bkp14rR, &'w mut Bkp14rW) -> &'w mut Bkp14rW
    {
        let bits = self.register.read();
        let r = Bkp14rR { bits: bits };
        let mut w = Bkp14rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp14rR {
        Bkp14rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp14rW) -> &mut Bkp14rW
    {
        let mut w = Bkp14rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp14rR {
    bits: u32,
}

impl Bkp14rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp14rW {
    bits: u32,
}

impl Bkp14rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp14rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp15r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp15r {
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
        where for<'w> F: FnOnce(&Bkp15rR, &'w mut Bkp15rW) -> &'w mut Bkp15rW
    {
        let bits = self.register.read();
        let r = Bkp15rR { bits: bits };
        let mut w = Bkp15rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp15rR {
        Bkp15rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp15rW) -> &mut Bkp15rW
    {
        let mut w = Bkp15rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp15rR {
    bits: u32,
}

impl Bkp15rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp15rW {
    bits: u32,
}

impl Bkp15rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp15rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp16r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp16r {
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
        where for<'w> F: FnOnce(&Bkp16rR, &'w mut Bkp16rW) -> &'w mut Bkp16rW
    {
        let bits = self.register.read();
        let r = Bkp16rR { bits: bits };
        let mut w = Bkp16rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp16rR {
        Bkp16rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp16rW) -> &mut Bkp16rW
    {
        let mut w = Bkp16rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp16rR {
    bits: u32,
}

impl Bkp16rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp16rW {
    bits: u32,
}

impl Bkp16rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp16rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp17r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp17r {
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
        where for<'w> F: FnOnce(&Bkp17rR, &'w mut Bkp17rW) -> &'w mut Bkp17rW
    {
        let bits = self.register.read();
        let r = Bkp17rR { bits: bits };
        let mut w = Bkp17rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp17rR {
        Bkp17rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp17rW) -> &mut Bkp17rW
    {
        let mut w = Bkp17rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp17rR {
    bits: u32,
}

impl Bkp17rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp17rW {
    bits: u32,
}

impl Bkp17rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp17rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp18r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp18r {
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
        where for<'w> F: FnOnce(&Bkp18rR, &'w mut Bkp18rW) -> &'w mut Bkp18rW
    {
        let bits = self.register.read();
        let r = Bkp18rR { bits: bits };
        let mut w = Bkp18rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp18rR {
        Bkp18rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp18rW) -> &mut Bkp18rW
    {
        let mut w = Bkp18rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp18rR {
    bits: u32,
}

impl Bkp18rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp18rW {
    bits: u32,
}

impl Bkp18rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp18rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bkp19r {
    register: ::volatile_register::RW<u32>,
}

impl Bkp19r {
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
        where for<'w> F: FnOnce(&Bkp19rR, &'w mut Bkp19rW) -> &'w mut Bkp19rW
    {
        let bits = self.register.read();
        let r = Bkp19rR { bits: bits };
        let mut w = Bkp19rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bkp19rR {
        Bkp19rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bkp19rW) -> &mut Bkp19rW
    {
        let mut w = Bkp19rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp19rR {
    bits: u32,
}

impl Bkp19rR {
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bkp19rW {
    bits: u32,
}

impl Bkp19rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bkp19rW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - BKP" ]
    pub fn bkp(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
