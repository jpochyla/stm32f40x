# [ doc = "USB on the go high speed" ]
# [ repr ( C ) ]
pub struct OtgHsDevice {
    # [ doc = "0x00 - OTG_HS device configuration register" ]
    pub otg_hs_dcfg: OtgHsDcfg,
    # [ doc = "0x04 - OTG_HS device control register" ]
    pub otg_hs_dctl: OtgHsDctl,
    # [ doc = "0x08 - OTG_HS device status register" ]
    pub otg_hs_dsts: OtgHsDsts,
    _reserved0: [u8; 4usize],
    # [ doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register" ]
    pub otg_hs_diepmsk: OtgHsDiepmsk,
    # [ doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register" ]
    pub otg_hs_doepmsk: OtgHsDoepmsk,
    # [ doc = "0x18 - OTG_HS device all endpoints interrupt register" ]
    pub otg_hs_daint: OtgHsDaint,
    # [ doc = "0x1c - OTG_HS all endpoints interrupt mask register" ]
    pub otg_hs_daintmsk: OtgHsDaintmsk,
    _reserved1: [u8; 8usize],
    # [ doc = "0x28 - OTG_HS device VBUS discharge time register" ]
    pub otg_hs_dvbusdis: OtgHsDvbusdis,
    # [ doc = "0x2c - OTG_HS device VBUS pulsing time register" ]
    pub otg_hs_dvbuspulse: OtgHsDvbuspulse,
    # [ doc = "0x30 - OTG_HS Device threshold control register" ]
    pub otg_hs_dthrctl: OtgHsDthrctl,
    # [ doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register" ]
    pub otg_hs_diepempmsk: OtgHsDiepempmsk,
    # [ doc = "0x38 - OTG_HS device each endpoint interrupt register" ]
    pub otg_hs_deachint: OtgHsDeachint,
    # [ doc = "0x3c - OTG_HS device each endpoint interrupt register mask" ]
    pub otg_hs_deachintmsk: OtgHsDeachintmsk,
    # [ doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register" ]
    pub otg_hs_diepeachmsk1: OtgHsDiepeachmsk1,
    _reserved2: [u8; 60usize],
    # [ doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register" ]
    pub otg_hs_doepeachmsk1: OtgHsDoepeachmsk1,
    _reserved3: [u8; 124usize],
    # [ doc = "0x100 - OTG device endpoint-0 control register" ]
    pub otg_hs_diepctl0: OtgHsDiepctl0,
    _reserved4: [u8; 4usize],
    # [ doc = "0x108 - OTG device endpoint-0 interrupt register" ]
    pub otg_hs_diepint0: OtgHsDiepint0,
    _reserved5: [u8; 4usize],
    # [ doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register" ]
    pub otg_hs_dieptsiz0: OtgHsDieptsiz0,
    # [ doc = "0x114 - OTG_HS device endpoint-1 DMA address register" ]
    pub otg_hs_diepdma1: OtgHsDiepdma1,
    # [ doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register" ]
    pub otg_hs_dtxfsts0: OtgHsDtxfsts0,
    _reserved6: [u8; 4usize],
    # [ doc = "0x120 - OTG device endpoint-1 control register" ]
    pub otg_hs_diepctl1: OtgHsDiepctl1,
    _reserved7: [u8; 4usize],
    # [ doc = "0x128 - OTG device endpoint-1 interrupt register" ]
    pub otg_hs_diepint1: OtgHsDiepint1,
    _reserved8: [u8; 4usize],
    # [ doc = "0x130 - OTG_HS device endpoint transfer size register" ]
    pub otg_hs_dieptsiz1: OtgHsDieptsiz1,
    # [ doc = "0x134 - OTG_HS device endpoint-2 DMA address register" ]
    pub otg_hs_diepdma2: OtgHsDiepdma2,
    # [ doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register" ]
    pub otg_hs_dtxfsts1: OtgHsDtxfsts1,
    _reserved9: [u8; 4usize],
    # [ doc = "0x140 - OTG device endpoint-2 control register" ]
    pub otg_hs_diepctl2: OtgHsDiepctl2,
    _reserved10: [u8; 4usize],
    # [ doc = "0x148 - OTG device endpoint-2 interrupt register" ]
    pub otg_hs_diepint2: OtgHsDiepint2,
    _reserved11: [u8; 4usize],
    # [ doc = "0x150 - OTG_HS device endpoint transfer size register" ]
    pub otg_hs_dieptsiz2: OtgHsDieptsiz2,
    # [ doc = "0x154 - OTG_HS device endpoint-3 DMA address register" ]
    pub otg_hs_diepdma3: OtgHsDiepdma3,
    # [ doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register" ]
    pub otg_hs_dtxfsts2: OtgHsDtxfsts2,
    _reserved12: [u8; 4usize],
    # [ doc = "0x160 - OTG device endpoint-3 control register" ]
    pub otg_hs_diepctl3: OtgHsDiepctl3,
    _reserved13: [u8; 4usize],
    # [ doc = "0x168 - OTG device endpoint-3 interrupt register" ]
    pub otg_hs_diepint3: OtgHsDiepint3,
    _reserved14: [u8; 4usize],
    # [ doc = "0x170 - OTG_HS device endpoint transfer size register" ]
    pub otg_hs_dieptsiz3: OtgHsDieptsiz3,
    # [ doc = "0x174 - OTG_HS device endpoint-4 DMA address register" ]
    pub otg_hs_diepdma4: OtgHsDiepdma4,
    # [ doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register" ]
    pub otg_hs_dtxfsts3: OtgHsDtxfsts3,
    _reserved15: [u8; 4usize],
    # [ doc = "0x180 - OTG device endpoint-4 control register" ]
    pub otg_hs_diepctl4: OtgHsDiepctl4,
    _reserved16: [u8; 4usize],
    # [ doc = "0x188 - OTG device endpoint-4 interrupt register" ]
    pub otg_hs_diepint4: OtgHsDiepint4,
    _reserved17: [u8; 4usize],
    # [ doc = "0x190 - OTG_HS device endpoint transfer size register" ]
    pub otg_hs_dieptsiz4: OtgHsDieptsiz4,
    # [ doc = "0x194 - OTG_HS device endpoint-5 DMA address register" ]
    pub otg_hs_diepdma5: OtgHsDiepdma5,
    # [ doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register" ]
    pub otg_hs_dtxfsts4: OtgHsDtxfsts4,
    _reserved18: [u8; 4usize],
    # [ doc = "0x1a0 - OTG device endpoint-5 control register" ]
    pub otg_hs_diepctl5: OtgHsDiepctl5,
    _reserved19: [u8; 4usize],
    # [ doc = "0x1a8 - OTG device endpoint-5 interrupt register" ]
    pub otg_hs_diepint5: OtgHsDiepint5,
    _reserved20: [u8; 4usize],
    # [ doc = "0x1b0 - OTG_HS device endpoint transfer size register" ]
    pub otg_hs_dieptsiz5: OtgHsDieptsiz5,
    _reserved21: [u8; 4usize],
    # [ doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register" ]
    pub otg_hs_dtxfsts5: OtgHsDtxfsts5,
    _reserved22: [u8; 4usize],
    # [ doc = "0x1c0 - OTG device endpoint-6 control register" ]
    pub otg_hs_diepctl6: OtgHsDiepctl6,
    _reserved23: [u8; 4usize],
    # [ doc = "0x1c8 - OTG device endpoint-6 interrupt register" ]
    pub otg_hs_diepint6: OtgHsDiepint6,
    _reserved24: [u8; 20usize],
    # [ doc = "0x1e0 - OTG device endpoint-7 control register" ]
    pub otg_hs_diepctl7: OtgHsDiepctl7,
    _reserved25: [u8; 4usize],
    # [ doc = "0x1e8 - OTG device endpoint-7 interrupt register" ]
    pub otg_hs_diepint7: OtgHsDiepint7,
    _reserved26: [u8; 276usize],
    # [ doc = "0x300 - OTG_HS device control OUT endpoint 0 control register" ]
    pub otg_hs_doepctl0: OtgHsDoepctl0,
    _reserved27: [u8; 4usize],
    # [ doc = "0x308 - OTG_HS device endpoint-0 interrupt register" ]
    pub otg_hs_doepint0: OtgHsDoepint0,
    _reserved28: [u8; 4usize],
    # [ doc = "0x310 - OTG_HS device endpoint-1 transfer size register" ]
    pub otg_hs_doeptsiz0: OtgHsDoeptsiz0,
    _reserved29: [u8; 12usize],
    # [ doc = "0x320 - OTG device endpoint-1 control register" ]
    pub otg_hs_doepctl1: OtgHsDoepctl1,
    _reserved30: [u8; 4usize],
    # [ doc = "0x328 - OTG_HS device endpoint-1 interrupt register" ]
    pub otg_hs_doepint1: OtgHsDoepint1,
    _reserved31: [u8; 4usize],
    # [ doc = "0x330 - OTG_HS device endpoint-2 transfer size register" ]
    pub otg_hs_doeptsiz1: OtgHsDoeptsiz1,
    _reserved32: [u8; 12usize],
    # [ doc = "0x340 - OTG device endpoint-2 control register" ]
    pub otg_hs_doepctl2: OtgHsDoepctl2,
    _reserved33: [u8; 4usize],
    # [ doc = "0x348 - OTG_HS device endpoint-2 interrupt register" ]
    pub otg_hs_doepint2: OtgHsDoepint2,
    _reserved34: [u8; 4usize],
    # [ doc = "0x350 - OTG_HS device endpoint-3 transfer size register" ]
    pub otg_hs_doeptsiz2: OtgHsDoeptsiz2,
    _reserved35: [u8; 12usize],
    # [ doc = "0x360 - OTG device endpoint-3 control register" ]
    pub otg_hs_doepctl3: OtgHsDoepctl3,
    _reserved36: [u8; 4usize],
    # [ doc = "0x368 - OTG_HS device endpoint-3 interrupt register" ]
    pub otg_hs_doepint3: OtgHsDoepint3,
    _reserved37: [u8; 4usize],
    # [ doc = "0x370 - OTG_HS device endpoint-4 transfer size register" ]
    pub otg_hs_doeptsiz3: OtgHsDoeptsiz3,
    _reserved38: [u8; 20usize],
    # [ doc = "0x388 - OTG_HS device endpoint-4 interrupt register" ]
    pub otg_hs_doepint4: OtgHsDoepint4,
    _reserved39: [u8; 4usize],
    # [ doc = "0x390 - OTG_HS device endpoint-5 transfer size register" ]
    pub otg_hs_doeptsiz4: OtgHsDoeptsiz4,
    _reserved40: [u8; 20usize],
    # [ doc = "0x3a8 - OTG_HS device endpoint-5 interrupt register" ]
    pub otg_hs_doepint5: OtgHsDoepint5,
    _reserved41: [u8; 28usize],
    # [ doc = "0x3c8 - OTG_HS device endpoint-6 interrupt register" ]
    pub otg_hs_doepint6: OtgHsDoepint6,
    _reserved42: [u8; 28usize],
    # [ doc = "0x3e8 - OTG_HS device endpoint-7 interrupt register" ]
    pub otg_hs_doepint7: OtgHsDoepint7,
}

# [ repr ( C ) ]
pub struct OtgHsDcfg {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDcfg {
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
        where for<'w> F: FnOnce(&OtgHsDcfgR, &'w mut OtgHsDcfgW) -> &'w mut OtgHsDcfgW
    {
        let bits = self.register.read();
        let r = OtgHsDcfgR { bits: bits };
        let mut w = OtgHsDcfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDcfgR {
        OtgHsDcfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDcfgW) -> &mut OtgHsDcfgW
    {
        let mut w = OtgHsDcfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDcfgR {
    bits: u32,
}

impl OtgHsDcfgR {
    # [ doc = "Bits 0:1 - Device speed" ]
    pub fn dspd(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Nonzero-length status OUT handshake" ]
    pub fn nzlsohsk(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:10 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Periodic (micro)frame interval" ]
    pub fn pfivl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:25 - Periodic scheduling interval" ]
    pub fn perschivl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDcfgW {
    bits: u32,
}

impl OtgHsDcfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDcfgW { bits: 35651584 }
    }
    # [ doc = "Bits 0:1 - Device speed" ]
    pub fn dspd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 2 - Nonzero-length status OUT handshake" ]
    pub fn nzlsohsk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:10 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Periodic (micro)frame interval" ]
    pub fn pfivl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:25 - Periodic scheduling interval" ]
    pub fn perschivl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDctl {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDctl {
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
        where for<'w> F: FnOnce(&OtgHsDctlR, &'w mut OtgHsDctlW) -> &'w mut OtgHsDctlW
    {
        let bits = self.register.read();
        let r = OtgHsDctlR { bits: bits };
        let mut w = OtgHsDctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDctlR {
        OtgHsDctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDctlW) -> &mut OtgHsDctlW
    {
        let mut w = OtgHsDctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDctlR {
    bits: u32,
}

impl OtgHsDctlR {
    # [ doc = "Bit 0 - Remote wakeup signaling" ]
    pub fn rwusig(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Soft disconnect" ]
    pub fn sdis(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Global IN NAK status" ]
    pub fn ginsts(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Global OUT NAK status" ]
    pub fn gonsts(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Test control" ]
    pub fn tctl(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Power-on programming done" ]
    pub fn poprgdne(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDctlW {
    bits: u32,
}

impl OtgHsDctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDctlW { bits: 0 }
    }
    # [ doc = "Bit 0 - Remote wakeup signaling" ]
    pub fn rwusig(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Soft disconnect" ]
    pub fn sdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Test control" ]
    pub fn tctl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Set global IN NAK" ]
    pub fn sginak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Clear global IN NAK" ]
    pub fn cginak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Set global OUT NAK" ]
    pub fn sgonak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Clear global OUT NAK" ]
    pub fn cgonak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Power-on programming done" ]
    pub fn poprgdne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDsts {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsDsts {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsDstsR {
        OtgHsDstsR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDstsR {
    bits: u32,
}

impl OtgHsDstsR {
    # [ doc = "Bit 0 - Suspend status" ]
    pub fn suspsts(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:2 - Enumerated speed" ]
    pub fn enumspd(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Erratic error" ]
    pub fn eerr(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:21 - Frame number of the received SOF" ]
    pub fn fnsof(&self) -> u16 {
        const MASK: u32 = 16383;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepmsk {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepmsk {
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
        where for<'w> F: FnOnce(&OtgHsDiepmskR, &'w mut OtgHsDiepmskW) -> &'w mut OtgHsDiepmskW
    {
        let bits = self.register.read();
        let r = OtgHsDiepmskR { bits: bits };
        let mut w = OtgHsDiepmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepmskR {
        OtgHsDiepmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepmskW) -> &mut OtgHsDiepmskW
    {
        let mut w = OtgHsDiepmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepmskR {
    bits: u32,
}

impl OtgHsDiepmskR {
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)" ]
    pub fn tom(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO empty mask" ]
    pub fn ittxfemsk(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - IN token received with EP mismatch mask" ]
    pub fn inepnmm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective mask" ]
    pub fn inepnem(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - FIFO underrun mask" ]
    pub fn txfurm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - BNA interrupt mask" ]
    pub fn bim(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepmskW {
    bits: u32,
}

impl OtgHsDiepmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepmskW { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)" ]
    pub fn tom(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO empty mask" ]
    pub fn ittxfemsk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - IN token received with EP mismatch mask" ]
    pub fn inepnmm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective mask" ]
    pub fn inepnem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - FIFO underrun mask" ]
    pub fn txfurm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - BNA interrupt mask" ]
    pub fn bim(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepmsk {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepmsk {
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
        where for<'w> F: FnOnce(&OtgHsDoepmskR, &'w mut OtgHsDoepmskW) -> &'w mut OtgHsDoepmskW
    {
        let bits = self.register.read();
        let r = OtgHsDoepmskR { bits: bits };
        let mut w = OtgHsDoepmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepmskR {
        OtgHsDoepmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepmskW) -> &mut OtgHsDoepmskW
    {
        let mut w = OtgHsDoepmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepmskR {
    bits: u32,
}

impl OtgHsDoepmskR {
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done mask" ]
    pub fn stupm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled mask" ]
    pub fn otepdm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received mask" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - OUT packet error mask" ]
    pub fn opem(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - BNA interrupt mask" ]
    pub fn boim(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepmskW {
    bits: u32,
}

impl OtgHsDoepmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepmskW { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done mask" ]
    pub fn stupm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled mask" ]
    pub fn otepdm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received mask" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - OUT packet error mask" ]
    pub fn opem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - BNA interrupt mask" ]
    pub fn boim(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDaint {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsDaint {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsDaintR {
        OtgHsDaintR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDaintR {
    bits: u32,
}

impl OtgHsDaintR {
    # [ doc = "Bits 0:15 - IN endpoint interrupt bits" ]
    pub fn iepint(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - OUT endpoint interrupt bits" ]
    pub fn oepint(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsDaintmsk {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDaintmsk {
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
        where for<'w> F: FnOnce(&OtgHsDaintmskR, &'w mut OtgHsDaintmskW) -> &'w mut OtgHsDaintmskW
    {
        let bits = self.register.read();
        let r = OtgHsDaintmskR { bits: bits };
        let mut w = OtgHsDaintmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDaintmskR {
        OtgHsDaintmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDaintmskW) -> &mut OtgHsDaintmskW
    {
        let mut w = OtgHsDaintmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDaintmskR {
    bits: u32,
}

impl OtgHsDaintmskR {
    # [ doc = "Bits 0:15 - IN EP interrupt mask bits" ]
    pub fn iepm(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - OUT EP interrupt mask bits" ]
    pub fn oepm(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDaintmskW {
    bits: u32,
}

impl OtgHsDaintmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDaintmskW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - IN EP interrupt mask bits" ]
    pub fn iepm(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - OUT EP interrupt mask bits" ]
    pub fn oepm(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDvbusdis {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDvbusdis {
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
        where for<'w> F: FnOnce(&OtgHsDvbusdisR, &'w mut OtgHsDvbusdisW) -> &'w mut OtgHsDvbusdisW
    {
        let bits = self.register.read();
        let r = OtgHsDvbusdisR { bits: bits };
        let mut w = OtgHsDvbusdisW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDvbusdisR {
        OtgHsDvbusdisR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDvbusdisW) -> &mut OtgHsDvbusdisW
    {
        let mut w = OtgHsDvbusdisW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDvbusdisR {
    bits: u32,
}

impl OtgHsDvbusdisR {
    # [ doc = "Bits 0:15 - Device VBUS discharge time" ]
    pub fn vbusdt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDvbusdisW {
    bits: u32,
}

impl OtgHsDvbusdisW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDvbusdisW { bits: 6103 }
    }
    # [ doc = "Bits 0:15 - Device VBUS discharge time" ]
    pub fn vbusdt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDvbuspulse {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDvbuspulse {
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
        where for<'w> F: FnOnce(&OtgHsDvbuspulseR, &'w mut OtgHsDvbuspulseW)
                                -> &'w mut OtgHsDvbuspulseW
    {
        let bits = self.register.read();
        let r = OtgHsDvbuspulseR { bits: bits };
        let mut w = OtgHsDvbuspulseW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDvbuspulseR {
        OtgHsDvbuspulseR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDvbuspulseW) -> &mut OtgHsDvbuspulseW
    {
        let mut w = OtgHsDvbuspulseW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDvbuspulseR {
    bits: u32,
}

impl OtgHsDvbuspulseR {
    # [ doc = "Bits 0:11 - Device VBUS pulsing time" ]
    pub fn dvbusp(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDvbuspulseW {
    bits: u32,
}

impl OtgHsDvbuspulseW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDvbuspulseW { bits: 1464 }
    }
    # [ doc = "Bits 0:11 - Device VBUS pulsing time" ]
    pub fn dvbusp(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDthrctl {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDthrctl {
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
        where for<'w> F: FnOnce(&OtgHsDthrctlR, &'w mut OtgHsDthrctlW) -> &'w mut OtgHsDthrctlW
    {
        let bits = self.register.read();
        let r = OtgHsDthrctlR { bits: bits };
        let mut w = OtgHsDthrctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDthrctlR {
        OtgHsDthrctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDthrctlW) -> &mut OtgHsDthrctlW
    {
        let mut w = OtgHsDthrctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDthrctlR {
    bits: u32,
}

impl OtgHsDthrctlR {
    # [ doc = "Bit 0 - Nonisochronous IN endpoints threshold enable" ]
    pub fn nonisothren(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - ISO IN endpoint threshold enable" ]
    pub fn isothren(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:10 - Transmit threshold length" ]
    pub fn txthrlen(&self) -> u16 {
        const MASK: u32 = 511;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 16 - Receive threshold enable" ]
    pub fn rxthren(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 17:25 - Receive threshold length" ]
    pub fn rxthrlen(&self) -> u16 {
        const MASK: u32 = 511;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 27 - Arbiter parking enable" ]
    pub fn arpen(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDthrctlW {
    bits: u32,
}

impl OtgHsDthrctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDthrctlW { bits: 0 }
    }
    # [ doc = "Bit 0 - Nonisochronous IN endpoints threshold enable" ]
    pub fn nonisothren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - ISO IN endpoint threshold enable" ]
    pub fn isothren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:10 - Transmit threshold length" ]
    pub fn txthrlen(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u16 = 511;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Receive threshold enable" ]
    pub fn rxthren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 17:25 - Receive threshold length" ]
    pub fn rxthrlen(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 17u8;
        const MASK: u16 = 511;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 27 - Arbiter parking enable" ]
    pub fn arpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepempmsk {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepempmsk {
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
        where for<'w> F: FnOnce(&OtgHsDiepempmskR, &'w mut OtgHsDiepempmskW)
                                -> &'w mut OtgHsDiepempmskW
    {
        let bits = self.register.read();
        let r = OtgHsDiepempmskR { bits: bits };
        let mut w = OtgHsDiepempmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepempmskR {
        OtgHsDiepempmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepempmskW) -> &mut OtgHsDiepempmskW
    {
        let mut w = OtgHsDiepempmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepempmskR {
    bits: u32,
}

impl OtgHsDiepempmskR {
    # [ doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits" ]
    pub fn ineptxfem(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepempmskW {
    bits: u32,
}

impl OtgHsDiepempmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepempmskW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits" ]
    pub fn ineptxfem(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDeachint {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDeachint {
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
        where for<'w> F: FnOnce(&OtgHsDeachintR, &'w mut OtgHsDeachintW) -> &'w mut OtgHsDeachintW
    {
        let bits = self.register.read();
        let r = OtgHsDeachintR { bits: bits };
        let mut w = OtgHsDeachintW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDeachintR {
        OtgHsDeachintR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDeachintW) -> &mut OtgHsDeachintW
    {
        let mut w = OtgHsDeachintW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDeachintR {
    bits: u32,
}

impl OtgHsDeachintR {
    # [ doc = "Bit 1 - IN endpoint 1interrupt bit" ]
    pub fn iep1int(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - OUT endpoint 1 interrupt bit" ]
    pub fn oep1int(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDeachintW {
    bits: u32,
}

impl OtgHsDeachintW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDeachintW { bits: 0 }
    }
    # [ doc = "Bit 1 - IN endpoint 1interrupt bit" ]
    pub fn iep1int(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - OUT endpoint 1 interrupt bit" ]
    pub fn oep1int(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDeachintmsk {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDeachintmsk {
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
        where for<'w> F: FnOnce(&OtgHsDeachintmskR, &'w mut OtgHsDeachintmskW)
                                -> &'w mut OtgHsDeachintmskW
    {
        let bits = self.register.read();
        let r = OtgHsDeachintmskR { bits: bits };
        let mut w = OtgHsDeachintmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDeachintmskR {
        OtgHsDeachintmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDeachintmskW) -> &mut OtgHsDeachintmskW
    {
        let mut w = OtgHsDeachintmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDeachintmskR {
    bits: u32,
}

impl OtgHsDeachintmskR {
    # [ doc = "Bit 1 - IN Endpoint 1 interrupt mask bit" ]
    pub fn iep1intm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit" ]
    pub fn oep1intm(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDeachintmskW {
    bits: u32,
}

impl OtgHsDeachintmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDeachintmskW { bits: 0 }
    }
    # [ doc = "Bit 1 - IN Endpoint 1 interrupt mask bit" ]
    pub fn iep1intm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit" ]
    pub fn oep1intm(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepeachmsk1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepeachmsk1 {
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
        where for<'w> F: FnOnce(&OtgHsDiepeachmsk1R, &'w mut OtgHsDiepeachmsk1W)
                                -> &'w mut OtgHsDiepeachmsk1W
    {
        let bits = self.register.read();
        let r = OtgHsDiepeachmsk1R { bits: bits };
        let mut w = OtgHsDiepeachmsk1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepeachmsk1R {
        OtgHsDiepeachmsk1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepeachmsk1W) -> &mut OtgHsDiepeachmsk1W
    {
        let mut w = OtgHsDiepeachmsk1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepeachmsk1R {
    bits: u32,
}

impl OtgHsDiepeachmsk1R {
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)" ]
    pub fn tom(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO empty mask" ]
    pub fn ittxfemsk(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - IN token received with EP mismatch mask" ]
    pub fn inepnmm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective mask" ]
    pub fn inepnem(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - FIFO underrun mask" ]
    pub fn txfurm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - BNA interrupt mask" ]
    pub fn bim(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepeachmsk1W {
    bits: u32,
}

impl OtgHsDiepeachmsk1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepeachmsk1W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)" ]
    pub fn tom(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO empty mask" ]
    pub fn ittxfemsk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - IN token received with EP mismatch mask" ]
    pub fn inepnmm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective mask" ]
    pub fn inepnem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - FIFO underrun mask" ]
    pub fn txfurm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - BNA interrupt mask" ]
    pub fn bim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDoepeachmsk1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepeachmsk1 {
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
        where for<'w> F: FnOnce(&OtgHsDoepeachmsk1R, &'w mut OtgHsDoepeachmsk1W)
                                -> &'w mut OtgHsDoepeachmsk1W
    {
        let bits = self.register.read();
        let r = OtgHsDoepeachmsk1R { bits: bits };
        let mut w = OtgHsDoepeachmsk1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepeachmsk1R {
        OtgHsDoepeachmsk1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepeachmsk1W) -> &mut OtgHsDoepeachmsk1W
    {
        let mut w = OtgHsDoepeachmsk1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepeachmsk1R {
    bits: u32,
}

impl OtgHsDoepeachmsk1R {
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition mask" ]
    pub fn tom(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO empty mask" ]
    pub fn ittxfemsk(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - IN token received with EP mismatch mask" ]
    pub fn inepnmm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective mask" ]
    pub fn inepnem(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - OUT packet error mask" ]
    pub fn txfurm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - BNA interrupt mask" ]
    pub fn bim(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Bubble error interrupt mask" ]
    pub fn berrm(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - NYET interrupt mask" ]
    pub fn nyetm(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepeachmsk1W {
    bits: u32,
}

impl OtgHsDoepeachmsk1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepeachmsk1W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition mask" ]
    pub fn tom(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO empty mask" ]
    pub fn ittxfemsk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - IN token received with EP mismatch mask" ]
    pub fn inepnmm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective mask" ]
    pub fn inepnem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - OUT packet error mask" ]
    pub fn txfurm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - BNA interrupt mask" ]
    pub fn bim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Bubble error interrupt mask" ]
    pub fn berrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - NYET interrupt mask" ]
    pub fn nyetm(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepctl0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepctl0 {
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
        where for<'w> F: FnOnce(&OtgHsDiepctl0R, &'w mut OtgHsDiepctl0W) -> &'w mut OtgHsDiepctl0W
    {
        let bits = self.register.read();
        let r = OtgHsDiepctl0R { bits: bits };
        let mut w = OtgHsDiepctl0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepctl0R {
        OtgHsDiepctl0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepctl0W) -> &mut OtgHsDiepctl0W
    {
        let mut w = OtgHsDiepctl0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl0R {
    bits: u32,
}

impl OtgHsDiepctl0R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even/odd frame" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl0W {
    bits: u32,
}

impl OtgHsDiepctl0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepctl0W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepctl1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepctl1 {
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
        where for<'w> F: FnOnce(&OtgHsDiepctl1R, &'w mut OtgHsDiepctl1W) -> &'w mut OtgHsDiepctl1W
    {
        let bits = self.register.read();
        let r = OtgHsDiepctl1R { bits: bits };
        let mut w = OtgHsDiepctl1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepctl1R {
        OtgHsDiepctl1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepctl1W) -> &mut OtgHsDiepctl1W
    {
        let mut w = OtgHsDiepctl1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl1R {
    bits: u32,
}

impl OtgHsDiepctl1R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even/odd frame" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl1W {
    bits: u32,
}

impl OtgHsDiepctl1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepctl1W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepctl2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepctl2 {
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
        where for<'w> F: FnOnce(&OtgHsDiepctl2R, &'w mut OtgHsDiepctl2W) -> &'w mut OtgHsDiepctl2W
    {
        let bits = self.register.read();
        let r = OtgHsDiepctl2R { bits: bits };
        let mut w = OtgHsDiepctl2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepctl2R {
        OtgHsDiepctl2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepctl2W) -> &mut OtgHsDiepctl2W
    {
        let mut w = OtgHsDiepctl2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl2R {
    bits: u32,
}

impl OtgHsDiepctl2R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even/odd frame" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl2W {
    bits: u32,
}

impl OtgHsDiepctl2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepctl2W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepctl3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepctl3 {
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
        where for<'w> F: FnOnce(&OtgHsDiepctl3R, &'w mut OtgHsDiepctl3W) -> &'w mut OtgHsDiepctl3W
    {
        let bits = self.register.read();
        let r = OtgHsDiepctl3R { bits: bits };
        let mut w = OtgHsDiepctl3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepctl3R {
        OtgHsDiepctl3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepctl3W) -> &mut OtgHsDiepctl3W
    {
        let mut w = OtgHsDiepctl3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl3R {
    bits: u32,
}

impl OtgHsDiepctl3R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even/odd frame" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl3W {
    bits: u32,
}

impl OtgHsDiepctl3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepctl3W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepctl4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepctl4 {
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
        where for<'w> F: FnOnce(&OtgHsDiepctl4R, &'w mut OtgHsDiepctl4W) -> &'w mut OtgHsDiepctl4W
    {
        let bits = self.register.read();
        let r = OtgHsDiepctl4R { bits: bits };
        let mut w = OtgHsDiepctl4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepctl4R {
        OtgHsDiepctl4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepctl4W) -> &mut OtgHsDiepctl4W
    {
        let mut w = OtgHsDiepctl4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl4R {
    bits: u32,
}

impl OtgHsDiepctl4R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even/odd frame" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl4W {
    bits: u32,
}

impl OtgHsDiepctl4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepctl4W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepctl5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepctl5 {
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
        where for<'w> F: FnOnce(&OtgHsDiepctl5R, &'w mut OtgHsDiepctl5W) -> &'w mut OtgHsDiepctl5W
    {
        let bits = self.register.read();
        let r = OtgHsDiepctl5R { bits: bits };
        let mut w = OtgHsDiepctl5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepctl5R {
        OtgHsDiepctl5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepctl5W) -> &mut OtgHsDiepctl5W
    {
        let mut w = OtgHsDiepctl5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl5R {
    bits: u32,
}

impl OtgHsDiepctl5R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even/odd frame" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl5W {
    bits: u32,
}

impl OtgHsDiepctl5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepctl5W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepctl6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepctl6 {
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
        where for<'w> F: FnOnce(&OtgHsDiepctl6R, &'w mut OtgHsDiepctl6W) -> &'w mut OtgHsDiepctl6W
    {
        let bits = self.register.read();
        let r = OtgHsDiepctl6R { bits: bits };
        let mut w = OtgHsDiepctl6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepctl6R {
        OtgHsDiepctl6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepctl6W) -> &mut OtgHsDiepctl6W
    {
        let mut w = OtgHsDiepctl6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl6R {
    bits: u32,
}

impl OtgHsDiepctl6R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even/odd frame" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl6W {
    bits: u32,
}

impl OtgHsDiepctl6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepctl6W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepctl7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepctl7 {
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
        where for<'w> F: FnOnce(&OtgHsDiepctl7R, &'w mut OtgHsDiepctl7W) -> &'w mut OtgHsDiepctl7W
    {
        let bits = self.register.read();
        let r = OtgHsDiepctl7R { bits: bits };
        let mut w = OtgHsDiepctl7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepctl7R {
        OtgHsDiepctl7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepctl7W) -> &mut OtgHsDiepctl7W
    {
        let mut w = OtgHsDiepctl7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl7R {
    bits: u32,
}

impl OtgHsDiepctl7R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even/odd frame" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepctl7W {
    bits: u32,
}

impl OtgHsDiepctl7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepctl7W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDiepint0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepint0 {
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
        where for<'w> F: FnOnce(&OtgHsDiepint0R, &'w mut OtgHsDiepint0W) -> &'w mut OtgHsDiepint0W
    {
        let bits = self.register.read();
        let r = OtgHsDiepint0R { bits: bits };
        let mut w = OtgHsDiepint0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepint0R {
        OtgHsDiepint0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepint0W) -> &mut OtgHsDiepint0W
    {
        let mut w = OtgHsDiepint0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint0R {
    bits: u32,
}

impl OtgHsDiepint0R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit FIFO empty" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint0W {
    bits: u32,
}

impl OtgHsDiepint0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepint0W { bits: 128 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepint1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepint1 {
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
        where for<'w> F: FnOnce(&OtgHsDiepint1R, &'w mut OtgHsDiepint1W) -> &'w mut OtgHsDiepint1W
    {
        let bits = self.register.read();
        let r = OtgHsDiepint1R { bits: bits };
        let mut w = OtgHsDiepint1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepint1R {
        OtgHsDiepint1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepint1W) -> &mut OtgHsDiepint1W
    {
        let mut w = OtgHsDiepint1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint1R {
    bits: u32,
}

impl OtgHsDiepint1R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit FIFO empty" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint1W {
    bits: u32,
}

impl OtgHsDiepint1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepint1W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepint2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepint2 {
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
        where for<'w> F: FnOnce(&OtgHsDiepint2R, &'w mut OtgHsDiepint2W) -> &'w mut OtgHsDiepint2W
    {
        let bits = self.register.read();
        let r = OtgHsDiepint2R { bits: bits };
        let mut w = OtgHsDiepint2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepint2R {
        OtgHsDiepint2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepint2W) -> &mut OtgHsDiepint2W
    {
        let mut w = OtgHsDiepint2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint2R {
    bits: u32,
}

impl OtgHsDiepint2R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit FIFO empty" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint2W {
    bits: u32,
}

impl OtgHsDiepint2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepint2W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepint3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepint3 {
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
        where for<'w> F: FnOnce(&OtgHsDiepint3R, &'w mut OtgHsDiepint3W) -> &'w mut OtgHsDiepint3W
    {
        let bits = self.register.read();
        let r = OtgHsDiepint3R { bits: bits };
        let mut w = OtgHsDiepint3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepint3R {
        OtgHsDiepint3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepint3W) -> &mut OtgHsDiepint3W
    {
        let mut w = OtgHsDiepint3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint3R {
    bits: u32,
}

impl OtgHsDiepint3R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit FIFO empty" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint3W {
    bits: u32,
}

impl OtgHsDiepint3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepint3W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepint4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepint4 {
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
        where for<'w> F: FnOnce(&OtgHsDiepint4R, &'w mut OtgHsDiepint4W) -> &'w mut OtgHsDiepint4W
    {
        let bits = self.register.read();
        let r = OtgHsDiepint4R { bits: bits };
        let mut w = OtgHsDiepint4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepint4R {
        OtgHsDiepint4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepint4W) -> &mut OtgHsDiepint4W
    {
        let mut w = OtgHsDiepint4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint4R {
    bits: u32,
}

impl OtgHsDiepint4R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit FIFO empty" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint4W {
    bits: u32,
}

impl OtgHsDiepint4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepint4W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepint5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepint5 {
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
        where for<'w> F: FnOnce(&OtgHsDiepint5R, &'w mut OtgHsDiepint5W) -> &'w mut OtgHsDiepint5W
    {
        let bits = self.register.read();
        let r = OtgHsDiepint5R { bits: bits };
        let mut w = OtgHsDiepint5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepint5R {
        OtgHsDiepint5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepint5W) -> &mut OtgHsDiepint5W
    {
        let mut w = OtgHsDiepint5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint5R {
    bits: u32,
}

impl OtgHsDiepint5R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit FIFO empty" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint5W {
    bits: u32,
}

impl OtgHsDiepint5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepint5W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepint6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepint6 {
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
        where for<'w> F: FnOnce(&OtgHsDiepint6R, &'w mut OtgHsDiepint6W) -> &'w mut OtgHsDiepint6W
    {
        let bits = self.register.read();
        let r = OtgHsDiepint6R { bits: bits };
        let mut w = OtgHsDiepint6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepint6R {
        OtgHsDiepint6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepint6W) -> &mut OtgHsDiepint6W
    {
        let mut w = OtgHsDiepint6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint6R {
    bits: u32,
}

impl OtgHsDiepint6R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit FIFO empty" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint6W {
    bits: u32,
}

impl OtgHsDiepint6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepint6W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepint7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepint7 {
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
        where for<'w> F: FnOnce(&OtgHsDiepint7R, &'w mut OtgHsDiepint7W) -> &'w mut OtgHsDiepint7W
    {
        let bits = self.register.read();
        let r = OtgHsDiepint7R { bits: bits };
        let mut w = OtgHsDiepint7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepint7R {
        OtgHsDiepint7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepint7W) -> &mut OtgHsDiepint7W
    {
        let mut w = OtgHsDiepint7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint7R {
    bits: u32,
}

impl OtgHsDiepint7R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit FIFO empty" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepint7W {
    bits: u32,
}

impl OtgHsDiepint7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepint7W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO is empty" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Transmit Fifo Underrun" ]
    pub fn txfifoudrn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Buffer not available interrupt" ]
    pub fn bna(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Packet dropped status" ]
    pub fn pktdrpsts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Babble error interrupt" ]
    pub fn berr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - NAK interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptsiz0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptsiz0 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDieptsiz0R , & 'w mut OtgHsDieptsiz0W ) -> & 'w mut OtgHsDieptsiz0W , {
        let bits = self.register.read();
        let r = OtgHsDieptsiz0R { bits: bits };
        let mut w = OtgHsDieptsiz0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptsiz0R {
        OtgHsDieptsiz0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptsiz0W) -> &mut OtgHsDieptsiz0W
    {
        let mut w = OtgHsDieptsiz0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz0R {
    bits: u32,
}

impl OtgHsDieptsiz0R {
    # [ doc = "Bits 0:6 - Transfer size" ]
    pub fn xfrsiz(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 19:20 - Packet count" ]
    pub fn pktcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz0W {
    bits: u32,
}

impl OtgHsDieptsiz0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptsiz0W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:20 - Packet count" ]
    pub fn pktcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDiepdma1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepdma1 {
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
        where for<'w> F: FnOnce(&OtgHsDiepdma1R, &'w mut OtgHsDiepdma1W) -> &'w mut OtgHsDiepdma1W
    {
        let bits = self.register.read();
        let r = OtgHsDiepdma1R { bits: bits };
        let mut w = OtgHsDiepdma1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepdma1R {
        OtgHsDiepdma1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepdma1W) -> &mut OtgHsDiepdma1W
    {
        let mut w = OtgHsDiepdma1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma1R {
    bits: u32,
}

impl OtgHsDiepdma1R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma1W {
    bits: u32,
}

impl OtgHsDiepdma1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepdma1W { bits: 0 }
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
pub struct OtgHsDiepdma2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepdma2 {
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
        where for<'w> F: FnOnce(&OtgHsDiepdma2R, &'w mut OtgHsDiepdma2W) -> &'w mut OtgHsDiepdma2W
    {
        let bits = self.register.read();
        let r = OtgHsDiepdma2R { bits: bits };
        let mut w = OtgHsDiepdma2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepdma2R {
        OtgHsDiepdma2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepdma2W) -> &mut OtgHsDiepdma2W
    {
        let mut w = OtgHsDiepdma2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma2R {
    bits: u32,
}

impl OtgHsDiepdma2R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma2W {
    bits: u32,
}

impl OtgHsDiepdma2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepdma2W { bits: 0 }
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
pub struct OtgHsDiepdma3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepdma3 {
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
        where for<'w> F: FnOnce(&OtgHsDiepdma3R, &'w mut OtgHsDiepdma3W) -> &'w mut OtgHsDiepdma3W
    {
        let bits = self.register.read();
        let r = OtgHsDiepdma3R { bits: bits };
        let mut w = OtgHsDiepdma3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepdma3R {
        OtgHsDiepdma3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepdma3W) -> &mut OtgHsDiepdma3W
    {
        let mut w = OtgHsDiepdma3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma3R {
    bits: u32,
}

impl OtgHsDiepdma3R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma3W {
    bits: u32,
}

impl OtgHsDiepdma3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepdma3W { bits: 0 }
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
pub struct OtgHsDiepdma4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepdma4 {
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
        where for<'w> F: FnOnce(&OtgHsDiepdma4R, &'w mut OtgHsDiepdma4W) -> &'w mut OtgHsDiepdma4W
    {
        let bits = self.register.read();
        let r = OtgHsDiepdma4R { bits: bits };
        let mut w = OtgHsDiepdma4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepdma4R {
        OtgHsDiepdma4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepdma4W) -> &mut OtgHsDiepdma4W
    {
        let mut w = OtgHsDiepdma4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma4R {
    bits: u32,
}

impl OtgHsDiepdma4R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma4W {
    bits: u32,
}

impl OtgHsDiepdma4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepdma4W { bits: 0 }
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
pub struct OtgHsDiepdma5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDiepdma5 {
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
        where for<'w> F: FnOnce(&OtgHsDiepdma5R, &'w mut OtgHsDiepdma5W) -> &'w mut OtgHsDiepdma5W
    {
        let bits = self.register.read();
        let r = OtgHsDiepdma5R { bits: bits };
        let mut w = OtgHsDiepdma5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDiepdma5R {
        OtgHsDiepdma5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDiepdma5W) -> &mut OtgHsDiepdma5W
    {
        let mut w = OtgHsDiepdma5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma5R {
    bits: u32,
}

impl OtgHsDiepdma5R {
    # [ doc = "Bits 0:31 - DMA address" ]
    pub fn dmaaddr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDiepdma5W {
    bits: u32,
}

impl OtgHsDiepdma5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDiepdma5W { bits: 0 }
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
pub struct OtgHsDtxfsts0 {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsDtxfsts0 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsDtxfsts0R {
        OtgHsDtxfsts0R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDtxfsts0R {
    bits: u32,
}

impl OtgHsDtxfsts0R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space avail" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsDtxfsts1 {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsDtxfsts1 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsDtxfsts1R {
        OtgHsDtxfsts1R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDtxfsts1R {
    bits: u32,
}

impl OtgHsDtxfsts1R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space avail" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsDtxfsts2 {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsDtxfsts2 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsDtxfsts2R {
        OtgHsDtxfsts2R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDtxfsts2R {
    bits: u32,
}

impl OtgHsDtxfsts2R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space avail" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsDtxfsts3 {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsDtxfsts3 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsDtxfsts3R {
        OtgHsDtxfsts3R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDtxfsts3R {
    bits: u32,
}

impl OtgHsDtxfsts3R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space avail" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsDtxfsts4 {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsDtxfsts4 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsDtxfsts4R {
        OtgHsDtxfsts4R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDtxfsts4R {
    bits: u32,
}

impl OtgHsDtxfsts4R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space avail" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsDtxfsts5 {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsDtxfsts5 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsDtxfsts5R {
        OtgHsDtxfsts5R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDtxfsts5R {
    bits: u32,
}

impl OtgHsDtxfsts5R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space avail" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptsiz1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptsiz1 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDieptsiz1R , & 'w mut OtgHsDieptsiz1W ) -> & 'w mut OtgHsDieptsiz1W , {
        let bits = self.register.read();
        let r = OtgHsDieptsiz1R { bits: bits };
        let mut w = OtgHsDieptsiz1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptsiz1R {
        OtgHsDieptsiz1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptsiz1W) -> &mut OtgHsDieptsiz1W
    {
        let mut w = OtgHsDieptsiz1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz1R {
    bits: u32,
}

impl OtgHsDieptsiz1R {
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz1W {
    bits: u32,
}

impl OtgHsDieptsiz1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptsiz1W { bits: 0 }
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptsiz2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptsiz2 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDieptsiz2R , & 'w mut OtgHsDieptsiz2W ) -> & 'w mut OtgHsDieptsiz2W , {
        let bits = self.register.read();
        let r = OtgHsDieptsiz2R { bits: bits };
        let mut w = OtgHsDieptsiz2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptsiz2R {
        OtgHsDieptsiz2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptsiz2W) -> &mut OtgHsDieptsiz2W
    {
        let mut w = OtgHsDieptsiz2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz2R {
    bits: u32,
}

impl OtgHsDieptsiz2R {
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz2W {
    bits: u32,
}

impl OtgHsDieptsiz2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptsiz2W { bits: 0 }
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptsiz3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptsiz3 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDieptsiz3R , & 'w mut OtgHsDieptsiz3W ) -> & 'w mut OtgHsDieptsiz3W , {
        let bits = self.register.read();
        let r = OtgHsDieptsiz3R { bits: bits };
        let mut w = OtgHsDieptsiz3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptsiz3R {
        OtgHsDieptsiz3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptsiz3W) -> &mut OtgHsDieptsiz3W
    {
        let mut w = OtgHsDieptsiz3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz3R {
    bits: u32,
}

impl OtgHsDieptsiz3R {
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz3W {
    bits: u32,
}

impl OtgHsDieptsiz3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptsiz3W { bits: 0 }
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptsiz4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptsiz4 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDieptsiz4R , & 'w mut OtgHsDieptsiz4W ) -> & 'w mut OtgHsDieptsiz4W , {
        let bits = self.register.read();
        let r = OtgHsDieptsiz4R { bits: bits };
        let mut w = OtgHsDieptsiz4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptsiz4R {
        OtgHsDieptsiz4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptsiz4W) -> &mut OtgHsDieptsiz4W
    {
        let mut w = OtgHsDieptsiz4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz4R {
    bits: u32,
}

impl OtgHsDieptsiz4R {
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz4W {
    bits: u32,
}

impl OtgHsDieptsiz4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptsiz4W { bits: 0 }
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptsiz5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptsiz5 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDieptsiz5R , & 'w mut OtgHsDieptsiz5W ) -> & 'w mut OtgHsDieptsiz5W , {
        let bits = self.register.read();
        let r = OtgHsDieptsiz5R { bits: bits };
        let mut w = OtgHsDieptsiz5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptsiz5R {
        OtgHsDieptsiz5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptsiz5W) -> &mut OtgHsDieptsiz5W
    {
        let mut w = OtgHsDieptsiz5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz5R {
    bits: u32,
}

impl OtgHsDieptsiz5R {
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptsiz5W {
    bits: u32,
}

impl OtgHsDieptsiz5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptsiz5W { bits: 0 }
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
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDoepctl0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepctl0 {
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
        where for<'w> F: FnOnce(&OtgHsDoepctl0R, &'w mut OtgHsDoepctl0W) -> &'w mut OtgHsDoepctl0W
    {
        let bits = self.register.read();
        let r = OtgHsDoepctl0R { bits: bits };
        let mut w = OtgHsDoepctl0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepctl0R {
        OtgHsDoepctl0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepctl0W) -> &mut OtgHsDoepctl0W
    {
        let mut w = OtgHsDoepctl0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepctl0R {
    bits: u32,
}

impl OtgHsDoepctl0R {
    # [ doc = "Bits 0:1 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - Snoop mode" ]
    pub fn snpm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepctl0W {
    bits: u32,
}

impl OtgHsDoepctl0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepctl0W { bits: 32768 }
    }
    # [ doc = "Bit 20 - Snoop mode" ]
    pub fn snpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepctl1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepctl1 {
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
        where for<'w> F: FnOnce(&OtgHsDoepctl1R, &'w mut OtgHsDoepctl1W) -> &'w mut OtgHsDoepctl1W
    {
        let bits = self.register.read();
        let r = OtgHsDoepctl1R { bits: bits };
        let mut w = OtgHsDoepctl1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepctl1R {
        OtgHsDoepctl1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepctl1W) -> &mut OtgHsDoepctl1W
    {
        let mut w = OtgHsDoepctl1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepctl1R {
    bits: u32,
}

impl OtgHsDoepctl1R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even odd frame/Endpoint data PID" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - Snoop mode" ]
    pub fn snpm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepctl1W {
    bits: u32,
}

impl OtgHsDoepctl1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepctl1W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 20 - Snoop mode" ]
    pub fn snpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID/Set even frame" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepctl2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepctl2 {
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
        where for<'w> F: FnOnce(&OtgHsDoepctl2R, &'w mut OtgHsDoepctl2W) -> &'w mut OtgHsDoepctl2W
    {
        let bits = self.register.read();
        let r = OtgHsDoepctl2R { bits: bits };
        let mut w = OtgHsDoepctl2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepctl2R {
        OtgHsDoepctl2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepctl2W) -> &mut OtgHsDoepctl2W
    {
        let mut w = OtgHsDoepctl2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepctl2R {
    bits: u32,
}

impl OtgHsDoepctl2R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even odd frame/Endpoint data PID" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - Snoop mode" ]
    pub fn snpm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepctl2W {
    bits: u32,
}

impl OtgHsDoepctl2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepctl2W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 20 - Snoop mode" ]
    pub fn snpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID/Set even frame" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepctl3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepctl3 {
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
        where for<'w> F: FnOnce(&OtgHsDoepctl3R, &'w mut OtgHsDoepctl3W) -> &'w mut OtgHsDoepctl3W
    {
        let bits = self.register.read();
        let r = OtgHsDoepctl3R { bits: bits };
        let mut w = OtgHsDoepctl3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepctl3R {
        OtgHsDoepctl3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepctl3W) -> &mut OtgHsDoepctl3W
    {
        let mut w = OtgHsDoepctl3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepctl3R {
    bits: u32,
}

impl OtgHsDoepctl3R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Even odd frame/Endpoint data PID" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - Snoop mode" ]
    pub fn snpm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepctl3W {
    bits: u32,
}

impl OtgHsDoepctl3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepctl3W { bits: 0 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 20 - Snoop mode" ]
    pub fn snpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Set DATA0 PID/Set even frame" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Set odd frame" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepint0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepint0 {
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
        where for<'w> F: FnOnce(&OtgHsDoepint0R, &'w mut OtgHsDoepint0W) -> &'w mut OtgHsDoepint0W
    {
        let bits = self.register.read();
        let r = OtgHsDoepint0R { bits: bits };
        let mut w = OtgHsDoepint0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepint0R {
        OtgHsDoepint0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepint0W) -> &mut OtgHsDoepint0W
    {
        let mut w = OtgHsDoepint0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint0R {
    bits: u32,
}

impl OtgHsDoepint0R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint0W {
    bits: u32,
}

impl OtgHsDoepint0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepint0W { bits: 128 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepint1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepint1 {
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
        where for<'w> F: FnOnce(&OtgHsDoepint1R, &'w mut OtgHsDoepint1W) -> &'w mut OtgHsDoepint1W
    {
        let bits = self.register.read();
        let r = OtgHsDoepint1R { bits: bits };
        let mut w = OtgHsDoepint1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepint1R {
        OtgHsDoepint1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepint1W) -> &mut OtgHsDoepint1W
    {
        let mut w = OtgHsDoepint1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint1R {
    bits: u32,
}

impl OtgHsDoepint1R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint1W {
    bits: u32,
}

impl OtgHsDoepint1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepint1W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepint2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepint2 {
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
        where for<'w> F: FnOnce(&OtgHsDoepint2R, &'w mut OtgHsDoepint2W) -> &'w mut OtgHsDoepint2W
    {
        let bits = self.register.read();
        let r = OtgHsDoepint2R { bits: bits };
        let mut w = OtgHsDoepint2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepint2R {
        OtgHsDoepint2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepint2W) -> &mut OtgHsDoepint2W
    {
        let mut w = OtgHsDoepint2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint2R {
    bits: u32,
}

impl OtgHsDoepint2R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint2W {
    bits: u32,
}

impl OtgHsDoepint2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepint2W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepint3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepint3 {
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
        where for<'w> F: FnOnce(&OtgHsDoepint3R, &'w mut OtgHsDoepint3W) -> &'w mut OtgHsDoepint3W
    {
        let bits = self.register.read();
        let r = OtgHsDoepint3R { bits: bits };
        let mut w = OtgHsDoepint3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepint3R {
        OtgHsDoepint3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepint3W) -> &mut OtgHsDoepint3W
    {
        let mut w = OtgHsDoepint3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint3R {
    bits: u32,
}

impl OtgHsDoepint3R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint3W {
    bits: u32,
}

impl OtgHsDoepint3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepint3W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepint4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepint4 {
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
        where for<'w> F: FnOnce(&OtgHsDoepint4R, &'w mut OtgHsDoepint4W) -> &'w mut OtgHsDoepint4W
    {
        let bits = self.register.read();
        let r = OtgHsDoepint4R { bits: bits };
        let mut w = OtgHsDoepint4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepint4R {
        OtgHsDoepint4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepint4W) -> &mut OtgHsDoepint4W
    {
        let mut w = OtgHsDoepint4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint4R {
    bits: u32,
}

impl OtgHsDoepint4R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint4W {
    bits: u32,
}

impl OtgHsDoepint4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepint4W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepint5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepint5 {
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
        where for<'w> F: FnOnce(&OtgHsDoepint5R, &'w mut OtgHsDoepint5W) -> &'w mut OtgHsDoepint5W
    {
        let bits = self.register.read();
        let r = OtgHsDoepint5R { bits: bits };
        let mut w = OtgHsDoepint5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepint5R {
        OtgHsDoepint5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepint5W) -> &mut OtgHsDoepint5W
    {
        let mut w = OtgHsDoepint5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint5R {
    bits: u32,
}

impl OtgHsDoepint5R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint5W {
    bits: u32,
}

impl OtgHsDoepint5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepint5W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepint6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepint6 {
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
        where for<'w> F: FnOnce(&OtgHsDoepint6R, &'w mut OtgHsDoepint6W) -> &'w mut OtgHsDoepint6W
    {
        let bits = self.register.read();
        let r = OtgHsDoepint6R { bits: bits };
        let mut w = OtgHsDoepint6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepint6R {
        OtgHsDoepint6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepint6W) -> &mut OtgHsDoepint6W
    {
        let mut w = OtgHsDoepint6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint6R {
    bits: u32,
}

impl OtgHsDoepint6R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint6W {
    bits: u32,
}

impl OtgHsDoepint6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepint6W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoepint7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoepint7 {
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
        where for<'w> F: FnOnce(&OtgHsDoepint7R, &'w mut OtgHsDoepint7W) -> &'w mut OtgHsDoepint7W
    {
        let bits = self.register.read();
        let r = OtgHsDoepint7R { bits: bits };
        let mut w = OtgHsDoepint7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoepint7R {
        OtgHsDoepint7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoepint7W) -> &mut OtgHsDoepint7W
    {
        let mut w = OtgHsDoepint7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint7R {
    bits: u32,
}

impl OtgHsDoepint7R {
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoepint7W {
    bits: u32,
}

impl OtgHsDoepint7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoepint7W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Back-to-back SETUP packets received" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - NYET interrupt" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsDoeptsiz0 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoeptsiz0 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDoeptsiz0R , & 'w mut OtgHsDoeptsiz0W ) -> & 'w mut OtgHsDoeptsiz0W , {
        let bits = self.register.read();
        let r = OtgHsDoeptsiz0R { bits: bits };
        let mut w = OtgHsDoeptsiz0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoeptsiz0R {
        OtgHsDoeptsiz0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoeptsiz0W) -> &mut OtgHsDoeptsiz0W
    {
        let mut w = OtgHsDoeptsiz0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz0R {
    bits: u32,
}

impl OtgHsDoeptsiz0R {
    # [ doc = "Bits 0:6 - Transfer size" ]
    pub fn xfrsiz(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 19 - Packet count" ]
    pub fn pktcnt(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 29:30 - SETUP packet count" ]
    pub fn stupcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz0W {
    bits: u32,
}

impl OtgHsDoeptsiz0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoeptsiz0W { bits: 0 }
    }
    # [ doc = "Bits 0:6 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 19 - Packet count" ]
    pub fn pktcnt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 29:30 - SETUP packet count" ]
    pub fn stupcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDoeptsiz1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoeptsiz1 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDoeptsiz1R , & 'w mut OtgHsDoeptsiz1W ) -> & 'w mut OtgHsDoeptsiz1W , {
        let bits = self.register.read();
        let r = OtgHsDoeptsiz1R { bits: bits };
        let mut w = OtgHsDoeptsiz1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoeptsiz1R {
        OtgHsDoeptsiz1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoeptsiz1W) -> &mut OtgHsDoeptsiz1W
    {
        let mut w = OtgHsDoeptsiz1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz1R {
    bits: u32,
}

impl OtgHsDoeptsiz1R {
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
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz1W {
    bits: u32,
}

impl OtgHsDoeptsiz1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoeptsiz1W { bits: 0 }
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
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDoeptsiz2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoeptsiz2 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDoeptsiz2R , & 'w mut OtgHsDoeptsiz2W ) -> & 'w mut OtgHsDoeptsiz2W , {
        let bits = self.register.read();
        let r = OtgHsDoeptsiz2R { bits: bits };
        let mut w = OtgHsDoeptsiz2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoeptsiz2R {
        OtgHsDoeptsiz2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoeptsiz2W) -> &mut OtgHsDoeptsiz2W
    {
        let mut w = OtgHsDoeptsiz2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz2R {
    bits: u32,
}

impl OtgHsDoeptsiz2R {
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
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz2W {
    bits: u32,
}

impl OtgHsDoeptsiz2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoeptsiz2W { bits: 0 }
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
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDoeptsiz3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoeptsiz3 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDoeptsiz3R , & 'w mut OtgHsDoeptsiz3W ) -> & 'w mut OtgHsDoeptsiz3W , {
        let bits = self.register.read();
        let r = OtgHsDoeptsiz3R { bits: bits };
        let mut w = OtgHsDoeptsiz3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoeptsiz3R {
        OtgHsDoeptsiz3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoeptsiz3W) -> &mut OtgHsDoeptsiz3W
    {
        let mut w = OtgHsDoeptsiz3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz3R {
    bits: u32,
}

impl OtgHsDoeptsiz3R {
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
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz3W {
    bits: u32,
}

impl OtgHsDoeptsiz3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoeptsiz3W { bits: 0 }
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
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDoeptsiz4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDoeptsiz4 {
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
    } pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & OtgHsDoeptsiz4R , & 'w mut OtgHsDoeptsiz4W ) -> & 'w mut OtgHsDoeptsiz4W , {
        let bits = self.register.read();
        let r = OtgHsDoeptsiz4R { bits: bits };
        let mut w = OtgHsDoeptsiz4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDoeptsiz4R {
        OtgHsDoeptsiz4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDoeptsiz4W) -> &mut OtgHsDoeptsiz4W
    {
        let mut w = OtgHsDoeptsiz4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz4R {
    bits: u32,
}

impl OtgHsDoeptsiz4R {
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
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDoeptsiz4W {
    bits: u32,
}

impl OtgHsDoeptsiz4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDoeptsiz4W { bits: 0 }
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
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
