# [ doc = "Digital camera interface" ]
# [ repr ( C ) ]
pub struct Dcmi {
    # [ doc = "0x00 - control register 1" ]
    pub cr: Cr,
    # [ doc = "0x04 - status register" ]
    pub sr: Sr,
    # [ doc = "0x08 - raw interrupt status register" ]
    pub ris: Ris,
    # [ doc = "0x0c - interrupt enable register" ]
    pub ier: Ier,
    # [ doc = "0x10 - masked interrupt status register" ]
    pub mis: Mis,
    # [ doc = "0x14 - interrupt clear register" ]
    pub icr: Icr,
    # [ doc = "0x18 - embedded synchronization code register" ]
    pub escr: Escr,
    # [ doc = "0x1c - embedded synchronization unmask register" ]
    pub esur: Esur,
    # [ doc = "0x20 - crop window start" ]
    pub cwstrt: Cwstrt,
    # [ doc = "0x24 - crop window size" ]
    pub cwsize: Cwsize,
    # [ doc = "0x28 - data register" ]
    pub dr: Dr,
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
    # [ doc = "Bit 14 - DCMI enable" ]
    pub fn enable(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 10:11 - Extended data mode" ]
    pub fn edm(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Frame capture rate control" ]
    pub fn fcrc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Vertical synchronization polarity" ]
    pub fn vspol(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Horizontal synchronization polarity" ]
    pub fn hspol(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Pixel clock polarity" ]
    pub fn pckpol(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Embedded synchronization select" ]
    pub fn ess(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - JPEG format" ]
    pub fn jpeg(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Crop feature" ]
    pub fn crop(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Capture mode" ]
    pub fn cm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Capture enable" ]
    pub fn capture(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
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
    # [ doc = "Bit 14 - DCMI enable" ]
    pub fn enable(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 10:11 - Extended data mode" ]
    pub fn edm(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Frame capture rate control" ]
    pub fn fcrc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Vertical synchronization polarity" ]
    pub fn vspol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Horizontal synchronization polarity" ]
    pub fn hspol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Pixel clock polarity" ]
    pub fn pckpol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Embedded synchronization select" ]
    pub fn ess(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - JPEG format" ]
    pub fn jpeg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Crop feature" ]
    pub fn crop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Capture mode" ]
    pub fn cm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Capture enable" ]
    pub fn capture(&mut self, value: bool) -> &mut Self {
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
    # [ doc = "Bit 2 - FIFO not empty" ]
    pub fn fne(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - VSYNC" ]
    pub fn vsync(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - HSYNC" ]
    pub fn hsync(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Ris {
    register: ::volatile_register::RO<u32>,
}

impl Ris {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> RisR {
        RisR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RisR {
    bits: u32,
}

impl RisR {
    # [ doc = "Bit 4 - Line raw interrupt status" ]
    pub fn line_ris(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - VSYNC raw interrupt status" ]
    pub fn vsync_ris(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Synchronization error raw interrupt status" ]
    pub fn err_ris(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Overrun raw interrupt status" ]
    pub fn ovr_ris(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Capture complete raw interrupt status" ]
    pub fn frame_ris(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Ier {
    register: ::volatile_register::RW<u32>,
}

impl Ier {
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
        where for<'w> F: FnOnce(&IerR, &'w mut IerW) -> &'w mut IerW
    {
        let bits = self.register.read();
        let r = IerR { bits: bits };
        let mut w = IerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IerR {
        IerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IerW) -> &mut IerW
    {
        let mut w = IerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IerR {
    bits: u32,
}

impl IerR {
    # [ doc = "Bit 4 - Line interrupt enable" ]
    pub fn line_ie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - VSYNC interrupt enable" ]
    pub fn vsync_ie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Synchronization error interrupt enable" ]
    pub fn err_ie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Overrun interrupt enable" ]
    pub fn ovr_ie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Capture complete interrupt enable" ]
    pub fn frame_ie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IerW {
    bits: u32,
}

impl IerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IerW { bits: 0 }
    }
    # [ doc = "Bit 4 - Line interrupt enable" ]
    pub fn line_ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - VSYNC interrupt enable" ]
    pub fn vsync_ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Synchronization error interrupt enable" ]
    pub fn err_ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Overrun interrupt enable" ]
    pub fn ovr_ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Capture complete interrupt enable" ]
    pub fn frame_ie(&mut self, value: bool) -> &mut Self {
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
pub struct Mis {
    register: ::volatile_register::RO<u32>,
}

impl Mis {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> MisR {
        MisR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MisR {
    bits: u32,
}

impl MisR {
    # [ doc = "Bit 4 - Line masked interrupt status" ]
    pub fn line_mis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - VSYNC masked interrupt status" ]
    pub fn vsync_mis(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Synchronization error masked interrupt status" ]
    pub fn err_mis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Overrun masked interrupt status" ]
    pub fn ovr_mis(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Capture complete masked interrupt status" ]
    pub fn frame_mis(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Icr {
    register: ::volatile_register::WO<u32>,
}

impl Icr {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut IcrW) -> &mut IcrW
    {
        let mut w = IcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IcrW {
    bits: u32,
}

impl IcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IcrW { bits: 0 }
    }
    # [ doc = "Bit 4 - line interrupt status clear" ]
    pub fn line_isc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Vertical synch interrupt status clear" ]
    pub fn vsync_isc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Synchronization error interrupt status clear" ]
    pub fn err_isc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Overrun interrupt status clear" ]
    pub fn ovr_isc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Capture complete interrupt status clear" ]
    pub fn frame_isc(&mut self, value: bool) -> &mut Self {
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
pub struct Escr {
    register: ::volatile_register::RW<u32>,
}

impl Escr {
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
        where for<'w> F: FnOnce(&EscrR, &'w mut EscrW) -> &'w mut EscrW
    {
        let bits = self.register.read();
        let r = EscrR { bits: bits };
        let mut w = EscrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> EscrR {
        EscrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut EscrW) -> &mut EscrW
    {
        let mut w = EscrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EscrR {
    bits: u32,
}

impl EscrR {
    # [ doc = "Bits 24:31 - Frame end delimiter code" ]
    pub fn fec(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Line end delimiter code" ]
    pub fn lec(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - Line start delimiter code" ]
    pub fn lsc(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Frame start delimiter code" ]
    pub fn fsc(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EscrW {
    bits: u32,
}

impl EscrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        EscrW { bits: 0 }
    }
    # [ doc = "Bits 24:31 - Frame end delimiter code" ]
    pub fn fec(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Line end delimiter code" ]
    pub fn lec(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - Line start delimiter code" ]
    pub fn lsc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Frame start delimiter code" ]
    pub fn fsc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Esur {
    register: ::volatile_register::RW<u32>,
}

impl Esur {
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
        where for<'w> F: FnOnce(&EsurR, &'w mut EsurW) -> &'w mut EsurW
    {
        let bits = self.register.read();
        let r = EsurR { bits: bits };
        let mut w = EsurW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> EsurR {
        EsurR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut EsurW) -> &mut EsurW
    {
        let mut w = EsurW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EsurR {
    bits: u32,
}

impl EsurR {
    # [ doc = "Bits 24:31 - Frame end delimiter unmask" ]
    pub fn feu(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Line end delimiter unmask" ]
    pub fn leu(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - Line start delimiter unmask" ]
    pub fn lsu(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Frame start delimiter unmask" ]
    pub fn fsu(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EsurW {
    bits: u32,
}

impl EsurW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        EsurW { bits: 0 }
    }
    # [ doc = "Bits 24:31 - Frame end delimiter unmask" ]
    pub fn feu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Line end delimiter unmask" ]
    pub fn leu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - Line start delimiter unmask" ]
    pub fn lsu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Frame start delimiter unmask" ]
    pub fn fsu(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cwstrt {
    register: ::volatile_register::RW<u32>,
}

impl Cwstrt {
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
        where for<'w> F: FnOnce(&CwstrtR, &'w mut CwstrtW) -> &'w mut CwstrtW
    {
        let bits = self.register.read();
        let r = CwstrtR { bits: bits };
        let mut w = CwstrtW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CwstrtR {
        CwstrtR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CwstrtW) -> &mut CwstrtW
    {
        let mut w = CwstrtW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CwstrtR {
    bits: u32,
}

impl CwstrtR {
    # [ doc = "Bits 16:28 - Vertical start line count" ]
    pub fn vst(&self) -> u16 {
        const MASK: u32 = 8191;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:13 - Horizontal offset count" ]
    pub fn hoffcnt(&self) -> u16 {
        const MASK: u32 = 16383;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CwstrtW {
    bits: u32,
}

impl CwstrtW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CwstrtW { bits: 0 }
    }
    # [ doc = "Bits 16:28 - Vertical start line count" ]
    pub fn vst(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 8191;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:13 - Horizontal offset count" ]
    pub fn hoffcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 16383;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cwsize {
    register: ::volatile_register::RW<u32>,
}

impl Cwsize {
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
        where for<'w> F: FnOnce(&CwsizeR, &'w mut CwsizeW) -> &'w mut CwsizeW
    {
        let bits = self.register.read();
        let r = CwsizeR { bits: bits };
        let mut w = CwsizeW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CwsizeR {
        CwsizeR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CwsizeW) -> &mut CwsizeW
    {
        let mut w = CwsizeW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CwsizeR {
    bits: u32,
}

impl CwsizeR {
    # [ doc = "Bits 16:29 - Vertical line count" ]
    pub fn vline(&self) -> u16 {
        const MASK: u32 = 16383;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:13 - Capture count" ]
    pub fn capcnt(&self) -> u16 {
        const MASK: u32 = 16383;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CwsizeW {
    bits: u32,
}

impl CwsizeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CwsizeW { bits: 0 }
    }
    # [ doc = "Bits 16:29 - Vertical line count" ]
    pub fn vline(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 16383;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:13 - Capture count" ]
    pub fn capcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 16383;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
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
    # [ doc = "Bits 24:31 - Data byte 3" ]
    pub fn byte3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Data byte 2" ]
    pub fn byte2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - Data byte 1" ]
    pub fn byte1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Data byte 0" ]
    pub fn byte0(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}
