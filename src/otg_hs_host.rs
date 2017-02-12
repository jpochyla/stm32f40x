# [ doc = "USB on the go high speed" ]
# [ repr ( C ) ]
pub struct OtgHsHost {
    # [ doc = "0x00 - OTG_HS host configuration register" ]
    pub otg_hs_hcfg: OtgHsHcfg,
    # [ doc = "0x04 - OTG_HS Host frame interval register" ]
    pub otg_hs_hfir: OtgHsHfir,
    # [ doc = "0x08 - OTG_HS host frame number/frame time remaining register" ]
    pub otg_hs_hfnum: OtgHsHfnum,
    _reserved0: [u8; 4usize],
    # [ doc = "0x10 - OTG_HS_Host periodic transmit FIFO/queue status register" ]
    pub otg_hs_hptxsts: OtgHsHptxsts,
    # [ doc = "0x14 - OTG_HS Host all channels interrupt register" ]
    pub otg_hs_haint: OtgHsHaint,
    # [ doc = "0x18 - OTG_HS host all channels interrupt mask register" ]
    pub otg_hs_haintmsk: OtgHsHaintmsk,
    _reserved1: [u8; 36usize],
    # [ doc = "0x40 - OTG_HS host port control and status register" ]
    pub otg_hs_hprt: OtgHsHprt,
    _reserved2: [u8; 188usize],
    # [ doc = "0x100 - OTG_HS host channel-0 characteristics register" ]
    pub otg_hs_hcchar0: OtgHsHcchar0,
    # [ doc = "0x104 - OTG_HS host channel-0 split control register" ]
    pub otg_hs_hcsplt0: OtgHsHcsplt0,
    # [ doc = "0x108 - OTG_HS host channel-11 interrupt register" ]
    pub otg_hs_hcint0: OtgHsHcint0,
    # [ doc = "0x10c - OTG_HS host channel-11 interrupt mask register" ]
    pub otg_hs_hcintmsk0: OtgHsHcintmsk0,
    # [ doc = "0x110 - OTG_HS host channel-11 transfer size register" ]
    pub otg_hs_hctsiz0: OtgHsHctsiz0,
    # [ doc = "0x114 - OTG_HS host channel-0 DMA address register" ]
    pub otg_hs_hcdma0: OtgHsHcdma0,
    _reserved3: [u8; 8usize],
    # [ doc = "0x120 - OTG_HS host channel-1 characteristics register" ]
    pub otg_hs_hcchar1: OtgHsHcchar1,
    # [ doc = "0x124 - OTG_HS host channel-1 split control register" ]
    pub otg_hs_hcsplt1: OtgHsHcsplt1,
    # [ doc = "0x128 - OTG_HS host channel-1 interrupt register" ]
    pub otg_hs_hcint1: OtgHsHcint1,
    # [ doc = "0x12c - OTG_HS host channel-1 interrupt mask register" ]
    pub otg_hs_hcintmsk1: OtgHsHcintmsk1,
    # [ doc = "0x130 - OTG_HS host channel-1 transfer size register" ]
    pub otg_hs_hctsiz1: OtgHsHctsiz1,
    # [ doc = "0x134 - OTG_HS host channel-1 DMA address register" ]
    pub otg_hs_hcdma1: OtgHsHcdma1,
    _reserved4: [u8; 8usize],
    # [ doc = "0x140 - OTG_HS host channel-2 characteristics register" ]
    pub otg_hs_hcchar2: OtgHsHcchar2,
    # [ doc = "0x144 - OTG_HS host channel-2 split control register" ]
    pub otg_hs_hcsplt2: OtgHsHcsplt2,
    # [ doc = "0x148 - OTG_HS host channel-2 interrupt register" ]
    pub otg_hs_hcint2: OtgHsHcint2,
    # [ doc = "0x14c - OTG_HS host channel-2 interrupt mask register" ]
    pub otg_hs_hcintmsk2: OtgHsHcintmsk2,
    # [ doc = "0x150 - OTG_HS host channel-2 transfer size register" ]
    pub otg_hs_hctsiz2: OtgHsHctsiz2,
    # [ doc = "0x154 - OTG_HS host channel-2 DMA address register" ]
    pub otg_hs_hcdma2: OtgHsHcdma2,
    _reserved5: [u8; 8usize],
    # [ doc = "0x160 - OTG_HS host channel-3 characteristics register" ]
    pub otg_hs_hcchar3: OtgHsHcchar3,
    # [ doc = "0x164 - OTG_HS host channel-3 split control register" ]
    pub otg_hs_hcsplt3: OtgHsHcsplt3,
    # [ doc = "0x168 - OTG_HS host channel-3 interrupt register" ]
    pub otg_hs_hcint3: OtgHsHcint3,
    # [ doc = "0x16c - OTG_HS host channel-3 interrupt mask register" ]
    pub otg_hs_hcintmsk3: OtgHsHcintmsk3,
    # [ doc = "0x170 - OTG_HS host channel-3 transfer size register" ]
    pub otg_hs_hctsiz3: OtgHsHctsiz3,
    # [ doc = "0x174 - OTG_HS host channel-3 DMA address register" ]
    pub otg_hs_hcdma3: OtgHsHcdma3,
    _reserved6: [u8; 8usize],
    # [ doc = "0x180 - OTG_HS host channel-4 characteristics register" ]
    pub otg_hs_hcchar4: OtgHsHcchar4,
    # [ doc = "0x184 - OTG_HS host channel-4 split control register" ]
    pub otg_hs_hcsplt4: OtgHsHcsplt4,
    # [ doc = "0x188 - OTG_HS host channel-4 interrupt register" ]
    pub otg_hs_hcint4: OtgHsHcint4,
    # [ doc = "0x18c - OTG_HS host channel-4 interrupt mask register" ]
    pub otg_hs_hcintmsk4: OtgHsHcintmsk4,
    # [ doc = "0x190 - OTG_HS host channel-4 transfer size register" ]
    pub otg_hs_hctsiz4: OtgHsHctsiz4,
    # [ doc = "0x194 - OTG_HS host channel-4 DMA address register" ]
    pub otg_hs_hcdma4: OtgHsHcdma4,
    _reserved7: [u8; 8usize],
    # [ doc = "0x1a0 - OTG_HS host channel-5 characteristics register" ]
    pub otg_hs_hcchar5: OtgHsHcchar5,
    # [ doc = "0x1a4 - OTG_HS host channel-5 split control register" ]
    pub otg_hs_hcsplt5: OtgHsHcsplt5,
    # [ doc = "0x1a8 - OTG_HS host channel-5 interrupt register" ]
    pub otg_hs_hcint5: OtgHsHcint5,
    # [ doc = "0x1ac - OTG_HS host channel-5 interrupt mask register" ]
    pub otg_hs_hcintmsk5: OtgHsHcintmsk5,
    # [ doc = "0x1b0 - OTG_HS host channel-5 transfer size register" ]
    pub otg_hs_hctsiz5: OtgHsHctsiz5,
    # [ doc = "0x1b4 - OTG_HS host channel-5 DMA address register" ]
    pub otg_hs_hcdma5: OtgHsHcdma5,
    _reserved8: [u8; 8usize],
    # [ doc = "0x1c0 - OTG_HS host channel-6 characteristics register" ]
    pub otg_hs_hcchar6: OtgHsHcchar6,
    # [ doc = "0x1c4 - OTG_HS host channel-6 split control register" ]
    pub otg_hs_hcsplt6: OtgHsHcsplt6,
    # [ doc = "0x1c8 - OTG_HS host channel-6 interrupt register" ]
    pub otg_hs_hcint6: OtgHsHcint6,
    # [ doc = "0x1cc - OTG_HS host channel-6 interrupt mask register" ]
    pub otg_hs_hcintmsk6: OtgHsHcintmsk6,
    # [ doc = "0x1d0 - OTG_HS host channel-6 transfer size register" ]
    pub otg_hs_hctsiz6: OtgHsHctsiz6,
    # [ doc = "0x1d4 - OTG_HS host channel-6 DMA address register" ]
    pub otg_hs_hcdma6: OtgHsHcdma6,
    _reserved9: [u8; 8usize],
    # [ doc = "0x1e0 - OTG_HS host channel-7 characteristics register" ]
    pub otg_hs_hcchar7: OtgHsHcchar7,
    # [ doc = "0x1e4 - OTG_HS host channel-7 split control register" ]
    pub otg_hs_hcsplt7: OtgHsHcsplt7,
    # [ doc = "0x1e8 - OTG_HS host channel-7 interrupt register" ]
    pub otg_hs_hcint7: OtgHsHcint7,
    # [ doc = "0x1ec - OTG_HS host channel-7 interrupt mask register" ]
    pub otg_hs_hcintmsk7: OtgHsHcintmsk7,
    # [ doc = "0x1f0 - OTG_HS host channel-7 transfer size register" ]
    pub otg_hs_hctsiz7: OtgHsHctsiz7,
    # [ doc = "0x1f4 - OTG_HS host channel-7 DMA address register" ]
    pub otg_hs_hcdma7: OtgHsHcdma7,
    _reserved10: [u8; 8usize],
    # [ doc = "0x200 - OTG_HS host channel-8 characteristics register" ]
    pub otg_hs_hcchar8: OtgHsHcchar8,
    # [ doc = "0x204 - OTG_HS host channel-8 split control register" ]
    pub otg_hs_hcsplt8: OtgHsHcsplt8,
    # [ doc = "0x208 - OTG_HS host channel-8 interrupt register" ]
    pub otg_hs_hcint8: OtgHsHcint8,
    # [ doc = "0x20c - OTG_HS host channel-8 interrupt mask register" ]
    pub otg_hs_hcintmsk8: OtgHsHcintmsk8,
    # [ doc = "0x210 - OTG_HS host channel-8 transfer size register" ]
    pub otg_hs_hctsiz8: OtgHsHctsiz8,
    # [ doc = "0x214 - OTG_HS host channel-8 DMA address register" ]
    pub otg_hs_hcdma8: OtgHsHcdma8,
    _reserved11: [u8; 8usize],
    # [ doc = "0x220 - OTG_HS host channel-9 characteristics register" ]
    pub otg_hs_hcchar9: OtgHsHcchar9,
    # [ doc = "0x224 - OTG_HS host channel-9 split control register" ]
    pub otg_hs_hcsplt9: OtgHsHcsplt9,
    # [ doc = "0x228 - OTG_HS host channel-9 interrupt register" ]
    pub otg_hs_hcint9: OtgHsHcint9,
    # [ doc = "0x22c - OTG_HS host channel-9 interrupt mask register" ]
    pub otg_hs_hcintmsk9: OtgHsHcintmsk9,
    # [ doc = "0x230 - OTG_HS host channel-9 transfer size register" ]
    pub otg_hs_hctsiz9: OtgHsHctsiz9,
    # [ doc = "0x234 - OTG_HS host channel-9 DMA address register" ]
    pub otg_hs_hcdma9: OtgHsHcdma9,
    _reserved12: [u8; 8usize],
    # [ doc = "0x240 - OTG_HS host channel-10 characteristics register" ]
    pub otg_hs_hcchar10: OtgHsHcchar10,
    # [ doc = "0x244 - OTG_HS host channel-10 split control register" ]
    pub otg_hs_hcsplt10: OtgHsHcsplt10,
    # [ doc = "0x248 - OTG_HS host channel-10 interrupt register" ]
    pub otg_hs_hcint10: OtgHsHcint10,
    # [ doc = "0x24c - OTG_HS host channel-10 interrupt mask register" ]
    pub otg_hs_hcintmsk10: OtgHsHcintmsk10,
    # [ doc = "0x250 - OTG_HS host channel-10 transfer size register" ]
    pub otg_hs_hctsiz10: OtgHsHctsiz10,
    # [ doc = "0x254 - OTG_HS host channel-10 DMA address register" ]
    pub otg_hs_hcdma10: OtgHsHcdma10,
    _reserved13: [u8; 8usize],
    # [ doc = "0x260 - OTG_HS host channel-11 characteristics register" ]
    pub otg_hs_hcchar11: OtgHsHcchar11,
    # [ doc = "0x264 - OTG_HS host channel-11 split control register" ]
    pub otg_hs_hcsplt11: OtgHsHcsplt11,
    # [ doc = "0x268 - OTG_HS host channel-11 interrupt register" ]
    pub otg_hs_hcint11: OtgHsHcint11,
    # [ doc = "0x26c - OTG_HS host channel-11 interrupt mask register" ]
    pub otg_hs_hcintmsk11: OtgHsHcintmsk11,
    # [ doc = "0x270 - OTG_HS host channel-11 transfer size register" ]
    pub otg_hs_hctsiz11: OtgHsHctsiz11,
    # [ doc = "0x274 - OTG_HS host channel-11 DMA address register" ]
    pub otg_hs_hcdma11: OtgHsHcdma11,
}

# [ repr ( C ) ]
pub struct OtgHsHcfg {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcfg {
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
        where for<'w> F: FnOnce(&OtgHsHcfgR, &'w mut OtgHsHcfgW) -> &'w mut OtgHsHcfgW
    {
        let bits = self.register.read();
        let r = OtgHsHcfgR { bits: bits };
        let mut w = OtgHsHcfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcfgR {
        OtgHsHcfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcfgW) -> &mut OtgHsHcfgW
    {
        let mut w = OtgHsHcfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcfgR {
    bits: u32,
}

impl OtgHsHcfgR {
    # [ doc = "Bits 0:1 - FS/LS PHY clock select" ]
    pub fn fslspcs(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - FS- and LS-only support" ]
    pub fn fslss(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcfgW {
    bits: u32,
}

impl OtgHsHcfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcfgW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - FS/LS PHY clock select" ]
    pub fn fslspcs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHfir {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHfir {
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
        where for<'w> F: FnOnce(&OtgHsHfirR, &'w mut OtgHsHfirW) -> &'w mut OtgHsHfirW
    {
        let bits = self.register.read();
        let r = OtgHsHfirR { bits: bits };
        let mut w = OtgHsHfirW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHfirR {
        OtgHsHfirR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHfirW) -> &mut OtgHsHfirW
    {
        let mut w = OtgHsHfirW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHfirR {
    bits: u32,
}

impl OtgHsHfirR {
    # [ doc = "Bits 0:15 - Frame interval" ]
    pub fn frivl(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHfirW {
    bits: u32,
}

impl OtgHsHfirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHfirW { bits: 60000 }
    }
    # [ doc = "Bits 0:15 - Frame interval" ]
    pub fn frivl(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHfnum {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsHfnum {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsHfnumR {
        OtgHsHfnumR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHfnumR {
    bits: u32,
}

impl OtgHsHfnumR {
    # [ doc = "Bits 0:15 - Frame number" ]
    pub fn frnum(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Frame time remaining" ]
    pub fn ftrem(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsHptxsts {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHptxsts {
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
        where for<'w> F: FnOnce(&OtgHsHptxstsR, &'w mut OtgHsHptxstsW) -> &'w mut OtgHsHptxstsW
    {
        let bits = self.register.read();
        let r = OtgHsHptxstsR { bits: bits };
        let mut w = OtgHsHptxstsW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHptxstsR {
        OtgHsHptxstsR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHptxstsW) -> &mut OtgHsHptxstsW
    {
        let mut w = OtgHsHptxstsW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHptxstsR {
    bits: u32,
}

impl OtgHsHptxstsR {
    # [ doc = "Bits 0:15 - Periodic transmit data FIFO space available" ]
    pub fn ptxfsavl(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:23 - Periodic transmit request queue space available" ]
    pub fn ptxqsav(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - Top of the periodic transmit request queue" ]
    pub fn ptxqtop(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHptxstsW {
    bits: u32,
}

impl OtgHsHptxstsW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHptxstsW { bits: 524544 }
    }
    # [ doc = "Bits 0:15 - Periodic transmit data FIFO space available" ]
    pub fn ptxfsavl(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHaint {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsHaint {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsHaintR {
        OtgHsHaintR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHaintR {
    bits: u32,
}

impl OtgHsHaintR {
    # [ doc = "Bits 0:15 - Channel interrupts" ]
    pub fn haint(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsHaintmsk {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHaintmsk {
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
        where for<'w> F: FnOnce(&OtgHsHaintmskR, &'w mut OtgHsHaintmskW) -> &'w mut OtgHsHaintmskW
    {
        let bits = self.register.read();
        let r = OtgHsHaintmskR { bits: bits };
        let mut w = OtgHsHaintmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHaintmskR {
        OtgHsHaintmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHaintmskW) -> &mut OtgHsHaintmskW
    {
        let mut w = OtgHsHaintmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHaintmskR {
    bits: u32,
}

impl OtgHsHaintmskR {
    # [ doc = "Bits 0:15 - Channel interrupt mask" ]
    pub fn haintm(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHaintmskW {
    bits: u32,
}

impl OtgHsHaintmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHaintmskW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Channel interrupt mask" ]
    pub fn haintm(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHprt {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHprt {
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
        where for<'w> F: FnOnce(&OtgHsHprtR, &'w mut OtgHsHprtW) -> &'w mut OtgHsHprtW
    {
        let bits = self.register.read();
        let r = OtgHsHprtR { bits: bits };
        let mut w = OtgHsHprtW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHprtR {
        OtgHsHprtR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHprtW) -> &mut OtgHsHprtW
    {
        let mut w = OtgHsHprtW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHprtR {
    bits: u32,
}

impl OtgHsHprtR {
    # [ doc = "Bit 0 - Port connect status" ]
    pub fn pcsts(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port connect detected" ]
    pub fn pcdet(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port enable" ]
    pub fn pena(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port enable/disable change" ]
    pub fn penchng(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port overcurrent active" ]
    pub fn poca(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port overcurrent change" ]
    pub fn pocchng(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port resume" ]
    pub fn pres(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port suspend" ]
    pub fn psusp(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port reset" ]
    pub fn prst(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 10:11 - Port line status" ]
    pub fn plsts(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 12 - Port power" ]
    pub fn ppwr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:16 - Port test control" ]
    pub fn ptctl(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 17:18 - Port speed" ]
    pub fn pspd(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHprtW {
    bits: u32,
}

impl OtgHsHprtW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHprtW { bits: 0 }
    }
    # [ doc = "Bit 1 - Port connect detected" ]
    pub fn pcdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port enable" ]
    pub fn pena(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port enable/disable change" ]
    pub fn penchng(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port overcurrent change" ]
    pub fn pocchng(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port resume" ]
    pub fn pres(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port suspend" ]
    pub fn psusp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port reset" ]
    pub fn prst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port power" ]
    pub fn ppwr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:16 - Port test control" ]
    pub fn ptctl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcchar0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar0 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar0R, &'w mut OtgHsHcchar0W) -> &'w mut OtgHsHcchar0W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar0R { bits: bits };
        let mut w = OtgHsHcchar0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar0R {
        OtgHsHcchar0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar0W) -> &mut OtgHsHcchar0W
    {
        let mut w = OtgHsHcchar0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar0R {
    bits: u32,
}

impl OtgHsHcchar0R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar0W {
    bits: u32,
}

impl OtgHsHcchar0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar0W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar1 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar1R, &'w mut OtgHsHcchar1W) -> &'w mut OtgHsHcchar1W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar1R { bits: bits };
        let mut w = OtgHsHcchar1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar1R {
        OtgHsHcchar1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar1W) -> &mut OtgHsHcchar1W
    {
        let mut w = OtgHsHcchar1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar1R {
    bits: u32,
}

impl OtgHsHcchar1R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar1W {
    bits: u32,
}

impl OtgHsHcchar1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar1W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar2 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar2R, &'w mut OtgHsHcchar2W) -> &'w mut OtgHsHcchar2W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar2R { bits: bits };
        let mut w = OtgHsHcchar2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar2R {
        OtgHsHcchar2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar2W) -> &mut OtgHsHcchar2W
    {
        let mut w = OtgHsHcchar2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar2R {
    bits: u32,
}

impl OtgHsHcchar2R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar2W {
    bits: u32,
}

impl OtgHsHcchar2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar2W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar3 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar3R, &'w mut OtgHsHcchar3W) -> &'w mut OtgHsHcchar3W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar3R { bits: bits };
        let mut w = OtgHsHcchar3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar3R {
        OtgHsHcchar3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar3W) -> &mut OtgHsHcchar3W
    {
        let mut w = OtgHsHcchar3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar3R {
    bits: u32,
}

impl OtgHsHcchar3R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar3W {
    bits: u32,
}

impl OtgHsHcchar3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar3W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar4 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar4R, &'w mut OtgHsHcchar4W) -> &'w mut OtgHsHcchar4W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar4R { bits: bits };
        let mut w = OtgHsHcchar4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar4R {
        OtgHsHcchar4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar4W) -> &mut OtgHsHcchar4W
    {
        let mut w = OtgHsHcchar4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar4R {
    bits: u32,
}

impl OtgHsHcchar4R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar4W {
    bits: u32,
}

impl OtgHsHcchar4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar4W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar5 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar5R, &'w mut OtgHsHcchar5W) -> &'w mut OtgHsHcchar5W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar5R { bits: bits };
        let mut w = OtgHsHcchar5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar5R {
        OtgHsHcchar5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar5W) -> &mut OtgHsHcchar5W
    {
        let mut w = OtgHsHcchar5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar5R {
    bits: u32,
}

impl OtgHsHcchar5R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar5W {
    bits: u32,
}

impl OtgHsHcchar5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar5W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar6 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar6R, &'w mut OtgHsHcchar6W) -> &'w mut OtgHsHcchar6W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar6R { bits: bits };
        let mut w = OtgHsHcchar6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar6R {
        OtgHsHcchar6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar6W) -> &mut OtgHsHcchar6W
    {
        let mut w = OtgHsHcchar6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar6R {
    bits: u32,
}

impl OtgHsHcchar6R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar6W {
    bits: u32,
}

impl OtgHsHcchar6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar6W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar7 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar7R, &'w mut OtgHsHcchar7W) -> &'w mut OtgHsHcchar7W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar7R { bits: bits };
        let mut w = OtgHsHcchar7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar7R {
        OtgHsHcchar7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar7W) -> &mut OtgHsHcchar7W
    {
        let mut w = OtgHsHcchar7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar7R {
    bits: u32,
}

impl OtgHsHcchar7R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar7W {
    bits: u32,
}

impl OtgHsHcchar7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar7W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar8 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar8 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar8R, &'w mut OtgHsHcchar8W) -> &'w mut OtgHsHcchar8W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar8R { bits: bits };
        let mut w = OtgHsHcchar8W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar8R {
        OtgHsHcchar8R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar8W) -> &mut OtgHsHcchar8W
    {
        let mut w = OtgHsHcchar8W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar8R {
    bits: u32,
}

impl OtgHsHcchar8R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar8W {
    bits: u32,
}

impl OtgHsHcchar8W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar8W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar9 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar9 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar9R, &'w mut OtgHsHcchar9W) -> &'w mut OtgHsHcchar9W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar9R { bits: bits };
        let mut w = OtgHsHcchar9W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar9R {
        OtgHsHcchar9R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar9W) -> &mut OtgHsHcchar9W
    {
        let mut w = OtgHsHcchar9W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar9R {
    bits: u32,
}

impl OtgHsHcchar9R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar9W {
    bits: u32,
}

impl OtgHsHcchar9W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar9W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar10 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar10 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar10R, &'w mut OtgHsHcchar10W) -> &'w mut OtgHsHcchar10W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar10R { bits: bits };
        let mut w = OtgHsHcchar10W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar10R {
        OtgHsHcchar10R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar10W) -> &mut OtgHsHcchar10W
    {
        let mut w = OtgHsHcchar10W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar10R {
    bits: u32,
}

impl OtgHsHcchar10R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar10W {
    bits: u32,
}

impl OtgHsHcchar10W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar10W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcchar11 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcchar11 {
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
        where for<'w> F: FnOnce(&OtgHsHcchar11R, &'w mut OtgHsHcchar11W) -> &'w mut OtgHsHcchar11W
    {
        let bits = self.register.read();
        let r = OtgHsHcchar11R { bits: bits };
        let mut w = OtgHsHcchar11W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcchar11R {
        OtgHsHcchar11R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcchar11W) -> &mut OtgHsHcchar11W
    {
        let mut w = OtgHsHcchar11W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar11R {
    bits: u32,
}

impl OtgHsHcchar11R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcchar11W {
    bits: u32,
}

impl OtgHsHcchar11W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcchar11W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)" ]
    pub fn mc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt0 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt0R, &'w mut OtgHsHcsplt0W) -> &'w mut OtgHsHcsplt0W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt0R { bits: bits };
        let mut w = OtgHsHcsplt0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt0R {
        OtgHsHcsplt0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt0W) -> &mut OtgHsHcsplt0W
    {
        let mut w = OtgHsHcsplt0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt0R {
    bits: u32,
}

impl OtgHsHcsplt0R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt0W {
    bits: u32,
}

impl OtgHsHcsplt0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt0W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt1 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt1R, &'w mut OtgHsHcsplt1W) -> &'w mut OtgHsHcsplt1W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt1R { bits: bits };
        let mut w = OtgHsHcsplt1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt1R {
        OtgHsHcsplt1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt1W) -> &mut OtgHsHcsplt1W
    {
        let mut w = OtgHsHcsplt1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt1R {
    bits: u32,
}

impl OtgHsHcsplt1R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt1W {
    bits: u32,
}

impl OtgHsHcsplt1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt1W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt2 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt2R, &'w mut OtgHsHcsplt2W) -> &'w mut OtgHsHcsplt2W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt2R { bits: bits };
        let mut w = OtgHsHcsplt2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt2R {
        OtgHsHcsplt2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt2W) -> &mut OtgHsHcsplt2W
    {
        let mut w = OtgHsHcsplt2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt2R {
    bits: u32,
}

impl OtgHsHcsplt2R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt2W {
    bits: u32,
}

impl OtgHsHcsplt2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt2W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt3 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt3R, &'w mut OtgHsHcsplt3W) -> &'w mut OtgHsHcsplt3W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt3R { bits: bits };
        let mut w = OtgHsHcsplt3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt3R {
        OtgHsHcsplt3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt3W) -> &mut OtgHsHcsplt3W
    {
        let mut w = OtgHsHcsplt3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt3R {
    bits: u32,
}

impl OtgHsHcsplt3R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt3W {
    bits: u32,
}

impl OtgHsHcsplt3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt3W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt4 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt4R, &'w mut OtgHsHcsplt4W) -> &'w mut OtgHsHcsplt4W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt4R { bits: bits };
        let mut w = OtgHsHcsplt4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt4R {
        OtgHsHcsplt4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt4W) -> &mut OtgHsHcsplt4W
    {
        let mut w = OtgHsHcsplt4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt4R {
    bits: u32,
}

impl OtgHsHcsplt4R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt4W {
    bits: u32,
}

impl OtgHsHcsplt4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt4W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt5 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt5R, &'w mut OtgHsHcsplt5W) -> &'w mut OtgHsHcsplt5W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt5R { bits: bits };
        let mut w = OtgHsHcsplt5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt5R {
        OtgHsHcsplt5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt5W) -> &mut OtgHsHcsplt5W
    {
        let mut w = OtgHsHcsplt5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt5R {
    bits: u32,
}

impl OtgHsHcsplt5R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt5W {
    bits: u32,
}

impl OtgHsHcsplt5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt5W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt6 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt6R, &'w mut OtgHsHcsplt6W) -> &'w mut OtgHsHcsplt6W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt6R { bits: bits };
        let mut w = OtgHsHcsplt6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt6R {
        OtgHsHcsplt6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt6W) -> &mut OtgHsHcsplt6W
    {
        let mut w = OtgHsHcsplt6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt6R {
    bits: u32,
}

impl OtgHsHcsplt6R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt6W {
    bits: u32,
}

impl OtgHsHcsplt6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt6W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt7 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt7R, &'w mut OtgHsHcsplt7W) -> &'w mut OtgHsHcsplt7W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt7R { bits: bits };
        let mut w = OtgHsHcsplt7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt7R {
        OtgHsHcsplt7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt7W) -> &mut OtgHsHcsplt7W
    {
        let mut w = OtgHsHcsplt7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt7R {
    bits: u32,
}

impl OtgHsHcsplt7R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt7W {
    bits: u32,
}

impl OtgHsHcsplt7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt7W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt8 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt8 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt8R, &'w mut OtgHsHcsplt8W) -> &'w mut OtgHsHcsplt8W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt8R { bits: bits };
        let mut w = OtgHsHcsplt8W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt8R {
        OtgHsHcsplt8R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt8W) -> &mut OtgHsHcsplt8W
    {
        let mut w = OtgHsHcsplt8W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt8R {
    bits: u32,
}

impl OtgHsHcsplt8R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt8W {
    bits: u32,
}

impl OtgHsHcsplt8W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt8W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt9 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt9 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt9R, &'w mut OtgHsHcsplt9W) -> &'w mut OtgHsHcsplt9W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt9R { bits: bits };
        let mut w = OtgHsHcsplt9W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt9R {
        OtgHsHcsplt9R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt9W) -> &mut OtgHsHcsplt9W
    {
        let mut w = OtgHsHcsplt9W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt9R {
    bits: u32,
}

impl OtgHsHcsplt9R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt9W {
    bits: u32,
}

impl OtgHsHcsplt9W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt9W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt10 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt10 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt10R, &'w mut OtgHsHcsplt10W) -> &'w mut OtgHsHcsplt10W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt10R { bits: bits };
        let mut w = OtgHsHcsplt10W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt10R {
        OtgHsHcsplt10R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt10W) -> &mut OtgHsHcsplt10W
    {
        let mut w = OtgHsHcsplt10W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt10R {
    bits: u32,
}

impl OtgHsHcsplt10R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt10W {
    bits: u32,
}

impl OtgHsHcsplt10W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt10W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcsplt11 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcsplt11 {
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
        where for<'w> F: FnOnce(&OtgHsHcsplt11R, &'w mut OtgHsHcsplt11W) -> &'w mut OtgHsHcsplt11W
    {
        let bits = self.register.read();
        let r = OtgHsHcsplt11R { bits: bits };
        let mut w = OtgHsHcsplt11W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcsplt11R {
        OtgHsHcsplt11R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcsplt11W) -> &mut OtgHsHcsplt11W
    {
        let mut w = OtgHsHcsplt11W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt11R {
    bits: u32,
}

impl OtgHsHcsplt11R {
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcsplt11W {
    bits: u32,
}

impl OtgHsHcsplt11W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcsplt11W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Port address" ]
    pub fn prtaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:13 - Hub address" ]
    pub fn hubaddr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - XACTPOS" ]
    pub fn xactpos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Do complete split" ]
    pub fn complsplt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Split enable" ]
    pub fn spliten(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsHcint0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint0 {
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
        where for<'w> F: FnOnce(&OtgHsHcint0R, &'w mut OtgHsHcint0W) -> &'w mut OtgHsHcint0W
    {
        let bits = self.register.read();
        let r = OtgHsHcint0R { bits: bits };
        let mut w = OtgHsHcint0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint0R {
        OtgHsHcint0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint0W) -> &mut OtgHsHcint0W
    {
        let mut w = OtgHsHcint0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint0R {
    bits: u32,
}

impl OtgHsHcint0R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint0W {
    bits: u32,
}

impl OtgHsHcint0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint0W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint1 {
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
        where for<'w> F: FnOnce(&OtgHsHcint1R, &'w mut OtgHsHcint1W) -> &'w mut OtgHsHcint1W
    {
        let bits = self.register.read();
        let r = OtgHsHcint1R { bits: bits };
        let mut w = OtgHsHcint1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint1R {
        OtgHsHcint1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint1W) -> &mut OtgHsHcint1W
    {
        let mut w = OtgHsHcint1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint1R {
    bits: u32,
}

impl OtgHsHcint1R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint1W {
    bits: u32,
}

impl OtgHsHcint1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint1W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint2 {
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
        where for<'w> F: FnOnce(&OtgHsHcint2R, &'w mut OtgHsHcint2W) -> &'w mut OtgHsHcint2W
    {
        let bits = self.register.read();
        let r = OtgHsHcint2R { bits: bits };
        let mut w = OtgHsHcint2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint2R {
        OtgHsHcint2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint2W) -> &mut OtgHsHcint2W
    {
        let mut w = OtgHsHcint2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint2R {
    bits: u32,
}

impl OtgHsHcint2R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint2W {
    bits: u32,
}

impl OtgHsHcint2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint2W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint3 {
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
        where for<'w> F: FnOnce(&OtgHsHcint3R, &'w mut OtgHsHcint3W) -> &'w mut OtgHsHcint3W
    {
        let bits = self.register.read();
        let r = OtgHsHcint3R { bits: bits };
        let mut w = OtgHsHcint3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint3R {
        OtgHsHcint3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint3W) -> &mut OtgHsHcint3W
    {
        let mut w = OtgHsHcint3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint3R {
    bits: u32,
}

impl OtgHsHcint3R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint3W {
    bits: u32,
}

impl OtgHsHcint3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint3W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint4 {
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
        where for<'w> F: FnOnce(&OtgHsHcint4R, &'w mut OtgHsHcint4W) -> &'w mut OtgHsHcint4W
    {
        let bits = self.register.read();
        let r = OtgHsHcint4R { bits: bits };
        let mut w = OtgHsHcint4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint4R {
        OtgHsHcint4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint4W) -> &mut OtgHsHcint4W
    {
        let mut w = OtgHsHcint4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint4R {
    bits: u32,
}

impl OtgHsHcint4R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint4W {
    bits: u32,
}

impl OtgHsHcint4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint4W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint5 {
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
        where for<'w> F: FnOnce(&OtgHsHcint5R, &'w mut OtgHsHcint5W) -> &'w mut OtgHsHcint5W
    {
        let bits = self.register.read();
        let r = OtgHsHcint5R { bits: bits };
        let mut w = OtgHsHcint5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint5R {
        OtgHsHcint5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint5W) -> &mut OtgHsHcint5W
    {
        let mut w = OtgHsHcint5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint5R {
    bits: u32,
}

impl OtgHsHcint5R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint5W {
    bits: u32,
}

impl OtgHsHcint5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint5W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint6 {
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
        where for<'w> F: FnOnce(&OtgHsHcint6R, &'w mut OtgHsHcint6W) -> &'w mut OtgHsHcint6W
    {
        let bits = self.register.read();
        let r = OtgHsHcint6R { bits: bits };
        let mut w = OtgHsHcint6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint6R {
        OtgHsHcint6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint6W) -> &mut OtgHsHcint6W
    {
        let mut w = OtgHsHcint6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint6R {
    bits: u32,
}

impl OtgHsHcint6R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint6W {
    bits: u32,
}

impl OtgHsHcint6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint6W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint7 {
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
        where for<'w> F: FnOnce(&OtgHsHcint7R, &'w mut OtgHsHcint7W) -> &'w mut OtgHsHcint7W
    {
        let bits = self.register.read();
        let r = OtgHsHcint7R { bits: bits };
        let mut w = OtgHsHcint7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint7R {
        OtgHsHcint7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint7W) -> &mut OtgHsHcint7W
    {
        let mut w = OtgHsHcint7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint7R {
    bits: u32,
}

impl OtgHsHcint7R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint7W {
    bits: u32,
}

impl OtgHsHcint7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint7W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint8 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint8 {
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
        where for<'w> F: FnOnce(&OtgHsHcint8R, &'w mut OtgHsHcint8W) -> &'w mut OtgHsHcint8W
    {
        let bits = self.register.read();
        let r = OtgHsHcint8R { bits: bits };
        let mut w = OtgHsHcint8W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint8R {
        OtgHsHcint8R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint8W) -> &mut OtgHsHcint8W
    {
        let mut w = OtgHsHcint8W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint8R {
    bits: u32,
}

impl OtgHsHcint8R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint8W {
    bits: u32,
}

impl OtgHsHcint8W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint8W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint9 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint9 {
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
        where for<'w> F: FnOnce(&OtgHsHcint9R, &'w mut OtgHsHcint9W) -> &'w mut OtgHsHcint9W
    {
        let bits = self.register.read();
        let r = OtgHsHcint9R { bits: bits };
        let mut w = OtgHsHcint9W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint9R {
        OtgHsHcint9R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint9W) -> &mut OtgHsHcint9W
    {
        let mut w = OtgHsHcint9W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint9R {
    bits: u32,
}

impl OtgHsHcint9R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint9W {
    bits: u32,
}

impl OtgHsHcint9W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint9W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint10 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint10 {
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
        where for<'w> F: FnOnce(&OtgHsHcint10R, &'w mut OtgHsHcint10W) -> &'w mut OtgHsHcint10W
    {
        let bits = self.register.read();
        let r = OtgHsHcint10R { bits: bits };
        let mut w = OtgHsHcint10W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint10R {
        OtgHsHcint10R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint10W) -> &mut OtgHsHcint10W
    {
        let mut w = OtgHsHcint10W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint10R {
    bits: u32,
}

impl OtgHsHcint10R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint10W {
    bits: u32,
}

impl OtgHsHcint10W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint10W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcint11 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcint11 {
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
        where for<'w> F: FnOnce(&OtgHsHcint11R, &'w mut OtgHsHcint11W) -> &'w mut OtgHsHcint11W
    {
        let bits = self.register.read();
        let r = OtgHsHcint11R { bits: bits };
        let mut w = OtgHsHcint11W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcint11R {
        OtgHsHcint11R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcint11W) -> &mut OtgHsHcint11W
    {
        let mut w = OtgHsHcint11W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint11R {
    bits: u32,
}

impl OtgHsHcint11R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcint11W {
    bits: u32,
}

impl OtgHsHcint11W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcint11W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Response received interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk0 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk0R , & 'w mut OtgHsHcintmsk0W ) -> & 'w mut OtgHsHcintmsk0W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk0R { bits: bits };
        let mut w = OtgHsHcintmsk0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk0R {
        OtgHsHcintmsk0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk0W) -> &mut OtgHsHcintmsk0W
    {
        let mut w = OtgHsHcintmsk0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk0R {
    bits: u32,
}

impl OtgHsHcintmsk0R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk0W {
    bits: u32,
}

impl OtgHsHcintmsk0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk0W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk1 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk1R , & 'w mut OtgHsHcintmsk1W ) -> & 'w mut OtgHsHcintmsk1W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk1R { bits: bits };
        let mut w = OtgHsHcintmsk1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk1R {
        OtgHsHcintmsk1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk1W) -> &mut OtgHsHcintmsk1W
    {
        let mut w = OtgHsHcintmsk1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk1R {
    bits: u32,
}

impl OtgHsHcintmsk1R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk1W {
    bits: u32,
}

impl OtgHsHcintmsk1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk1W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk2 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk2R , & 'w mut OtgHsHcintmsk2W ) -> & 'w mut OtgHsHcintmsk2W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk2R { bits: bits };
        let mut w = OtgHsHcintmsk2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk2R {
        OtgHsHcintmsk2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk2W) -> &mut OtgHsHcintmsk2W
    {
        let mut w = OtgHsHcintmsk2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk2R {
    bits: u32,
}

impl OtgHsHcintmsk2R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk2W {
    bits: u32,
}

impl OtgHsHcintmsk2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk2W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk3 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk3R , & 'w mut OtgHsHcintmsk3W ) -> & 'w mut OtgHsHcintmsk3W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk3R { bits: bits };
        let mut w = OtgHsHcintmsk3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk3R {
        OtgHsHcintmsk3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk3W) -> &mut OtgHsHcintmsk3W
    {
        let mut w = OtgHsHcintmsk3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk3R {
    bits: u32,
}

impl OtgHsHcintmsk3R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk3W {
    bits: u32,
}

impl OtgHsHcintmsk3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk3W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk4 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk4R , & 'w mut OtgHsHcintmsk4W ) -> & 'w mut OtgHsHcintmsk4W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk4R { bits: bits };
        let mut w = OtgHsHcintmsk4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk4R {
        OtgHsHcintmsk4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk4W) -> &mut OtgHsHcintmsk4W
    {
        let mut w = OtgHsHcintmsk4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk4R {
    bits: u32,
}

impl OtgHsHcintmsk4R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk4W {
    bits: u32,
}

impl OtgHsHcintmsk4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk4W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk5 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk5R , & 'w mut OtgHsHcintmsk5W ) -> & 'w mut OtgHsHcintmsk5W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk5R { bits: bits };
        let mut w = OtgHsHcintmsk5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk5R {
        OtgHsHcintmsk5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk5W) -> &mut OtgHsHcintmsk5W
    {
        let mut w = OtgHsHcintmsk5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk5R {
    bits: u32,
}

impl OtgHsHcintmsk5R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk5W {
    bits: u32,
}

impl OtgHsHcintmsk5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk5W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk6 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk6R , & 'w mut OtgHsHcintmsk6W ) -> & 'w mut OtgHsHcintmsk6W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk6R { bits: bits };
        let mut w = OtgHsHcintmsk6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk6R {
        OtgHsHcintmsk6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk6W) -> &mut OtgHsHcintmsk6W
    {
        let mut w = OtgHsHcintmsk6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk6R {
    bits: u32,
}

impl OtgHsHcintmsk6R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk6W {
    bits: u32,
}

impl OtgHsHcintmsk6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk6W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk7 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk7R , & 'w mut OtgHsHcintmsk7W ) -> & 'w mut OtgHsHcintmsk7W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk7R { bits: bits };
        let mut w = OtgHsHcintmsk7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk7R {
        OtgHsHcintmsk7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk7W) -> &mut OtgHsHcintmsk7W
    {
        let mut w = OtgHsHcintmsk7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk7R {
    bits: u32,
}

impl OtgHsHcintmsk7R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk7W {
    bits: u32,
}

impl OtgHsHcintmsk7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk7W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk8 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk8 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk8R , & 'w mut OtgHsHcintmsk8W ) -> & 'w mut OtgHsHcintmsk8W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk8R { bits: bits };
        let mut w = OtgHsHcintmsk8W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk8R {
        OtgHsHcintmsk8R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk8W) -> &mut OtgHsHcintmsk8W
    {
        let mut w = OtgHsHcintmsk8W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk8R {
    bits: u32,
}

impl OtgHsHcintmsk8R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk8W {
    bits: u32,
}

impl OtgHsHcintmsk8W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk8W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk9 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk9 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsHcintmsk9R , & 'w mut OtgHsHcintmsk9W ) -> & 'w mut OtgHsHcintmsk9W , {
        let bits = self.register.read();
        let r = OtgHsHcintmsk9R { bits: bits };
        let mut w = OtgHsHcintmsk9W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk9R {
        OtgHsHcintmsk9R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk9W) -> &mut OtgHsHcintmsk9W
    {
        let mut w = OtgHsHcintmsk9W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk9R {
    bits: u32,
}

impl OtgHsHcintmsk9R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk9W {
    bits: u32,
}

impl OtgHsHcintmsk9W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk9W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk10 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk10 {
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
        where for<'w> F: FnOnce(&OtgHsHcintmsk10R, &'w mut OtgHsHcintmsk10W)
                                -> &'w mut OtgHsHcintmsk10W
    {
        let bits = self.register.read();
        let r = OtgHsHcintmsk10R { bits: bits };
        let mut w = OtgHsHcintmsk10W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk10R {
        OtgHsHcintmsk10R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk10W) -> &mut OtgHsHcintmsk10W
    {
        let mut w = OtgHsHcintmsk10W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk10R {
    bits: u32,
}

impl OtgHsHcintmsk10R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk10W {
    bits: u32,
}

impl OtgHsHcintmsk10W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk10W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcintmsk11 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcintmsk11 {
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
        where for<'w> F: FnOnce(&OtgHsHcintmsk11R, &'w mut OtgHsHcintmsk11W)
                                -> &'w mut OtgHsHcintmsk11W
    {
        let bits = self.register.read();
        let r = OtgHsHcintmsk11R { bits: bits };
        let mut w = OtgHsHcintmsk11W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcintmsk11R {
        OtgHsHcintmsk11R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcintmsk11W) -> &mut OtgHsHcintmsk11W
    {
        let mut w = OtgHsHcintmsk11W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk11R {
    bits: u32,
}

impl OtgHsHcintmsk11R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcintmsk11W {
    bits: u32,
}

impl OtgHsHcintmsk11W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcintmsk11W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - AHB error" ]
    pub fn ahberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz0 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz0R, &'w mut OtgHsHctsiz0W) -> &'w mut OtgHsHctsiz0W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz0R { bits: bits };
        let mut w = OtgHsHctsiz0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz0R {
        OtgHsHctsiz0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz0W) -> &mut OtgHsHctsiz0W
    {
        let mut w = OtgHsHctsiz0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz0R {
    bits: u32,
}

impl OtgHsHctsiz0R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz0W {
    bits: u32,
}

impl OtgHsHctsiz0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz0W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz1 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz1R, &'w mut OtgHsHctsiz1W) -> &'w mut OtgHsHctsiz1W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz1R { bits: bits };
        let mut w = OtgHsHctsiz1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz1R {
        OtgHsHctsiz1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz1W) -> &mut OtgHsHctsiz1W
    {
        let mut w = OtgHsHctsiz1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz1R {
    bits: u32,
}

impl OtgHsHctsiz1R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz1W {
    bits: u32,
}

impl OtgHsHctsiz1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz1W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz2 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz2R, &'w mut OtgHsHctsiz2W) -> &'w mut OtgHsHctsiz2W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz2R { bits: bits };
        let mut w = OtgHsHctsiz2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz2R {
        OtgHsHctsiz2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz2W) -> &mut OtgHsHctsiz2W
    {
        let mut w = OtgHsHctsiz2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz2R {
    bits: u32,
}

impl OtgHsHctsiz2R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz2W {
    bits: u32,
}

impl OtgHsHctsiz2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz2W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz3 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz3R, &'w mut OtgHsHctsiz3W) -> &'w mut OtgHsHctsiz3W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz3R { bits: bits };
        let mut w = OtgHsHctsiz3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz3R {
        OtgHsHctsiz3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz3W) -> &mut OtgHsHctsiz3W
    {
        let mut w = OtgHsHctsiz3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz3R {
    bits: u32,
}

impl OtgHsHctsiz3R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz3W {
    bits: u32,
}

impl OtgHsHctsiz3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz3W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz4 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz4R, &'w mut OtgHsHctsiz4W) -> &'w mut OtgHsHctsiz4W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz4R { bits: bits };
        let mut w = OtgHsHctsiz4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz4R {
        OtgHsHctsiz4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz4W) -> &mut OtgHsHctsiz4W
    {
        let mut w = OtgHsHctsiz4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz4R {
    bits: u32,
}

impl OtgHsHctsiz4R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz4W {
    bits: u32,
}

impl OtgHsHctsiz4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz4W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz5 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz5R, &'w mut OtgHsHctsiz5W) -> &'w mut OtgHsHctsiz5W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz5R { bits: bits };
        let mut w = OtgHsHctsiz5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz5R {
        OtgHsHctsiz5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz5W) -> &mut OtgHsHctsiz5W
    {
        let mut w = OtgHsHctsiz5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz5R {
    bits: u32,
}

impl OtgHsHctsiz5R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz5W {
    bits: u32,
}

impl OtgHsHctsiz5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz5W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz6 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz6R, &'w mut OtgHsHctsiz6W) -> &'w mut OtgHsHctsiz6W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz6R { bits: bits };
        let mut w = OtgHsHctsiz6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz6R {
        OtgHsHctsiz6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz6W) -> &mut OtgHsHctsiz6W
    {
        let mut w = OtgHsHctsiz6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz6R {
    bits: u32,
}

impl OtgHsHctsiz6R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz6W {
    bits: u32,
}

impl OtgHsHctsiz6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz6W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz7 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz7R, &'w mut OtgHsHctsiz7W) -> &'w mut OtgHsHctsiz7W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz7R { bits: bits };
        let mut w = OtgHsHctsiz7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz7R {
        OtgHsHctsiz7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz7W) -> &mut OtgHsHctsiz7W
    {
        let mut w = OtgHsHctsiz7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz7R {
    bits: u32,
}

impl OtgHsHctsiz7R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz7W {
    bits: u32,
}

impl OtgHsHctsiz7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz7W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz8 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz8 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz8R, &'w mut OtgHsHctsiz8W) -> &'w mut OtgHsHctsiz8W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz8R { bits: bits };
        let mut w = OtgHsHctsiz8W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz8R {
        OtgHsHctsiz8R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz8W) -> &mut OtgHsHctsiz8W
    {
        let mut w = OtgHsHctsiz8W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz8R {
    bits: u32,
}

impl OtgHsHctsiz8R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz8W {
    bits: u32,
}

impl OtgHsHctsiz8W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz8W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz9 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz9 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz9R, &'w mut OtgHsHctsiz9W) -> &'w mut OtgHsHctsiz9W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz9R { bits: bits };
        let mut w = OtgHsHctsiz9W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz9R {
        OtgHsHctsiz9R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz9W) -> &mut OtgHsHctsiz9W
    {
        let mut w = OtgHsHctsiz9W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz9R {
    bits: u32,
}

impl OtgHsHctsiz9R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz9W {
    bits: u32,
}

impl OtgHsHctsiz9W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz9W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz10 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz10 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz10R, &'w mut OtgHsHctsiz10W) -> &'w mut OtgHsHctsiz10W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz10R { bits: bits };
        let mut w = OtgHsHctsiz10W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz10R {
        OtgHsHctsiz10R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz10W) -> &mut OtgHsHctsiz10W
    {
        let mut w = OtgHsHctsiz10W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz10R {
    bits: u32,
}

impl OtgHsHctsiz10R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz10W {
    bits: u32,
}

impl OtgHsHctsiz10W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz10W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHctsiz11 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHctsiz11 {
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
        where for<'w> F: FnOnce(&OtgHsHctsiz11R, &'w mut OtgHsHctsiz11W) -> &'w mut OtgHsHctsiz11W
    {
        let bits = self.register.read();
        let r = OtgHsHctsiz11R { bits: bits };
        let mut w = OtgHsHctsiz11W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHctsiz11R {
        OtgHsHctsiz11R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHctsiz11W) -> &mut OtgHsHctsiz11W
    {
        let mut w = OtgHsHctsiz11W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz11R {
    bits: u32,
}

impl OtgHsHctsiz11R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHctsiz11W {
    bits: u32,
}

impl OtgHsHctsiz11W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHctsiz11W { bits: 0 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma0 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma0R, &'w mut OtgHsHcdma0W) -> &'w mut OtgHsHcdma0W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma0R { bits: bits };
        let mut w = OtgHsHcdma0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma0R {
        OtgHsHcdma0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma0W) -> &mut OtgHsHcdma0W
    {
        let mut w = OtgHsHcdma0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma0R {
    bits: u32,
}

impl OtgHsHcdma0R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma0W {
    bits: u32,
}

impl OtgHsHcdma0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma0W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma1 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma1R, &'w mut OtgHsHcdma1W) -> &'w mut OtgHsHcdma1W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma1R { bits: bits };
        let mut w = OtgHsHcdma1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma1R {
        OtgHsHcdma1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma1W) -> &mut OtgHsHcdma1W
    {
        let mut w = OtgHsHcdma1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma1R {
    bits: u32,
}

impl OtgHsHcdma1R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma1W {
    bits: u32,
}

impl OtgHsHcdma1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma1W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma2 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma2R, &'w mut OtgHsHcdma2W) -> &'w mut OtgHsHcdma2W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma2R { bits: bits };
        let mut w = OtgHsHcdma2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma2R {
        OtgHsHcdma2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma2W) -> &mut OtgHsHcdma2W
    {
        let mut w = OtgHsHcdma2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma2R {
    bits: u32,
}

impl OtgHsHcdma2R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma2W {
    bits: u32,
}

impl OtgHsHcdma2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma2W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma3 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma3R, &'w mut OtgHsHcdma3W) -> &'w mut OtgHsHcdma3W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma3R { bits: bits };
        let mut w = OtgHsHcdma3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma3R {
        OtgHsHcdma3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma3W) -> &mut OtgHsHcdma3W
    {
        let mut w = OtgHsHcdma3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma3R {
    bits: u32,
}

impl OtgHsHcdma3R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma3W {
    bits: u32,
}

impl OtgHsHcdma3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma3W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma4 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma4R, &'w mut OtgHsHcdma4W) -> &'w mut OtgHsHcdma4W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma4R { bits: bits };
        let mut w = OtgHsHcdma4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma4R {
        OtgHsHcdma4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma4W) -> &mut OtgHsHcdma4W
    {
        let mut w = OtgHsHcdma4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma4R {
    bits: u32,
}

impl OtgHsHcdma4R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma4W {
    bits: u32,
}

impl OtgHsHcdma4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma4W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma5 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma5R, &'w mut OtgHsHcdma5W) -> &'w mut OtgHsHcdma5W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma5R { bits: bits };
        let mut w = OtgHsHcdma5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma5R {
        OtgHsHcdma5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma5W) -> &mut OtgHsHcdma5W
    {
        let mut w = OtgHsHcdma5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma5R {
    bits: u32,
}

impl OtgHsHcdma5R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma5W {
    bits: u32,
}

impl OtgHsHcdma5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma5W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma6 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma6R, &'w mut OtgHsHcdma6W) -> &'w mut OtgHsHcdma6W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma6R { bits: bits };
        let mut w = OtgHsHcdma6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma6R {
        OtgHsHcdma6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma6W) -> &mut OtgHsHcdma6W
    {
        let mut w = OtgHsHcdma6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma6R {
    bits: u32,
}

impl OtgHsHcdma6R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma6W {
    bits: u32,
}

impl OtgHsHcdma6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma6W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma7 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma7R, &'w mut OtgHsHcdma7W) -> &'w mut OtgHsHcdma7W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma7R { bits: bits };
        let mut w = OtgHsHcdma7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma7R {
        OtgHsHcdma7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma7W) -> &mut OtgHsHcdma7W
    {
        let mut w = OtgHsHcdma7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma7R {
    bits: u32,
}

impl OtgHsHcdma7R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma7W {
    bits: u32,
}

impl OtgHsHcdma7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma7W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma8 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma8 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma8R, &'w mut OtgHsHcdma8W) -> &'w mut OtgHsHcdma8W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma8R { bits: bits };
        let mut w = OtgHsHcdma8W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma8R {
        OtgHsHcdma8R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma8W) -> &mut OtgHsHcdma8W
    {
        let mut w = OtgHsHcdma8W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma8R {
    bits: u32,
}

impl OtgHsHcdma8R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma8W {
    bits: u32,
}

impl OtgHsHcdma8W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma8W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma9 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma9 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma9R, &'w mut OtgHsHcdma9W) -> &'w mut OtgHsHcdma9W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma9R { bits: bits };
        let mut w = OtgHsHcdma9W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma9R {
        OtgHsHcdma9R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma9W) -> &mut OtgHsHcdma9W
    {
        let mut w = OtgHsHcdma9W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma9R {
    bits: u32,
}

impl OtgHsHcdma9R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma9W {
    bits: u32,
}

impl OtgHsHcdma9W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma9W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma10 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma10 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma10R, &'w mut OtgHsHcdma10W) -> &'w mut OtgHsHcdma10W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma10R { bits: bits };
        let mut w = OtgHsHcdma10W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma10R {
        OtgHsHcdma10R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma10W) -> &mut OtgHsHcdma10W
    {
        let mut w = OtgHsHcdma10W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma10R {
    bits: u32,
}

impl OtgHsHcdma10R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma10W {
    bits: u32,
}

impl OtgHsHcdma10W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma10W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHcdma11 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHcdma11 {
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
        where for<'w> F: FnOnce(&OtgHsHcdma11R, &'w mut OtgHsHcdma11W) -> &'w mut OtgHsHcdma11W
    {
        let bits = self.register.read();
        let r = OtgHsHcdma11R { bits: bits };
        let mut w = OtgHsHcdma11W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHcdma11R {
        OtgHsHcdma11R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHcdma11W) -> &mut OtgHsHcdma11W
    {
        let mut w = OtgHsHcdma11W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma11R {
    bits: u32,
}

impl OtgHsHcdma11R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHcdma11W {
    bits: u32,
}

impl OtgHsHcdma11W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHcdma11W { bits: 0 }
    }
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
