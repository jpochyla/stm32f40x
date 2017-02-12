# [ doc = "USB on the go high speed" ]
# [ repr ( C ) ]
pub struct OtgHsGlobal {
    # [ doc = "0x00 - OTG_HS control and status register" ]
    pub otg_hs_gotgctl: OtgHsGotgctl,
    # [ doc = "0x04 - OTG_HS interrupt register" ]
    pub otg_hs_gotgint: OtgHsGotgint,
    # [ doc = "0x08 - OTG_HS AHB configuration register" ]
    pub otg_hs_gahbcfg: OtgHsGahbcfg,
    # [ doc = "0x0c - OTG_HS USB configuration register" ]
    pub otg_hs_gusbcfg: OtgHsGusbcfg,
    # [ doc = "0x10 - OTG_HS reset register" ]
    pub otg_hs_grstctl: OtgHsGrstctl,
    # [ doc = "0x14 - OTG_HS core interrupt register" ]
    pub otg_hs_gintsts: OtgHsGintsts,
    # [ doc = "0x18 - OTG_HS interrupt mask register" ]
    pub otg_hs_gintmsk: OtgHsGintmsk,
    # [ doc = "0x1c - OTG_HS Receive status debug read register (host mode)" ]
    pub otg_hs_grxstsr_host: OtgHsGrxstsrHost,
    # [ doc = "0x20 - OTG_HS status read and pop register (host mode)" ]
    pub otg_hs_grxstsp_host: OtgHsGrxstspHost,
    # [ doc = "0x24 - OTG_HS Receive FIFO size register" ]
    pub otg_hs_grxfsiz: OtgHsGrxfsiz,
    # [ doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)" ]
    pub otg_hs_gnptxfsiz_host: OtgHsGnptxfsizHost,
    # [ doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register" ]
    pub otg_hs_gnptxsts: OtgHsGnptxsts,
    _reserved0: [u8; 8usize],
    # [ doc = "0x38 - OTG_HS general core configuration register" ]
    pub otg_hs_gccfg: OtgHsGccfg,
    # [ doc = "0x3c - OTG_HS core ID register" ]
    pub otg_hs_cid: OtgHsCid,
    _reserved1: [u8; 192usize],
    # [ doc = "0x100 - OTG_HS Host periodic transmit FIFO size register" ]
    pub otg_hs_hptxfsiz: OtgHsHptxfsiz,
    # [ doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register" ]
    pub otg_hs_dieptxf1: OtgHsDieptxf1,
    # [ doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register" ]
    pub otg_hs_dieptxf2: OtgHsDieptxf2,
    _reserved2: [u8; 16usize],
    # [ doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register" ]
    pub otg_hs_dieptxf3: OtgHsDieptxf3,
    # [ doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register" ]
    pub otg_hs_dieptxf4: OtgHsDieptxf4,
    # [ doc = "0x124 - OTG_HS device IN endpoint transmit FIFO size register" ]
    pub otg_hs_dieptxf5: OtgHsDieptxf5,
    # [ doc = "0x128 - OTG_HS device IN endpoint transmit FIFO size register" ]
    pub otg_hs_dieptxf6: OtgHsDieptxf6,
    # [ doc = "0x12c - OTG_HS device IN endpoint transmit FIFO size register" ]
    pub otg_hs_dieptxf7: OtgHsDieptxf7,
}

# [ repr ( C ) ]
pub struct OtgHsGotgctl {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGotgctl {
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
        where for<'w> F: FnOnce(&OtgHsGotgctlR, &'w mut OtgHsGotgctlW) -> &'w mut OtgHsGotgctlW
    {
        let bits = self.register.read();
        let r = OtgHsGotgctlR { bits: bits };
        let mut w = OtgHsGotgctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGotgctlR {
        OtgHsGotgctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGotgctlW) -> &mut OtgHsGotgctlW
    {
        let mut w = OtgHsGotgctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGotgctlR {
    bits: u32,
}

impl OtgHsGotgctlR {
    # [ doc = "Bit 0 - Session request success" ]
    pub fn srqscs(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Session request" ]
    pub fn srq(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Host negotiation success" ]
    pub fn hngscs(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - HNP request" ]
    pub fn hnprq(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Host set HNP enable" ]
    pub fn hshnpen(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Device HNP enabled" ]
    pub fn dhnpen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Connector ID status" ]
    pub fn cidsts(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Long/short debounce time" ]
    pub fn dbct(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - A-session valid" ]
    pub fn asvld(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - B-session valid" ]
    pub fn bsvld(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGotgctlW {
    bits: u32,
}

impl OtgHsGotgctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGotgctlW { bits: 2048 }
    }
    # [ doc = "Bit 1 - Session request" ]
    pub fn srq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - HNP request" ]
    pub fn hnprq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Host set HNP enable" ]
    pub fn hshnpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Device HNP enabled" ]
    pub fn dhnpen(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsGotgint {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGotgint {
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
        where for<'w> F: FnOnce(&OtgHsGotgintR, &'w mut OtgHsGotgintW) -> &'w mut OtgHsGotgintW
    {
        let bits = self.register.read();
        let r = OtgHsGotgintR { bits: bits };
        let mut w = OtgHsGotgintW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGotgintR {
        OtgHsGotgintR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGotgintW) -> &mut OtgHsGotgintW
    {
        let mut w = OtgHsGotgintW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGotgintR {
    bits: u32,
}

impl OtgHsGotgintR {
    # [ doc = "Bit 2 - Session end detected" ]
    pub fn sedet(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Session request success status change" ]
    pub fn srsschg(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Host negotiation success status change" ]
    pub fn hnsschg(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Host negotiation detected" ]
    pub fn hngdet(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - A-device timeout change" ]
    pub fn adtochg(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Debounce done" ]
    pub fn dbcdne(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGotgintW {
    bits: u32,
}

impl OtgHsGotgintW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGotgintW { bits: 0 }
    }
    # [ doc = "Bit 2 - Session end detected" ]
    pub fn sedet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Session request success status change" ]
    pub fn srsschg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Host negotiation success status change" ]
    pub fn hnsschg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Host negotiation detected" ]
    pub fn hngdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - A-device timeout change" ]
    pub fn adtochg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Debounce done" ]
    pub fn dbcdne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsGahbcfg {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGahbcfg {
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
        where for<'w> F: FnOnce(&OtgHsGahbcfgR, &'w mut OtgHsGahbcfgW) -> &'w mut OtgHsGahbcfgW
    {
        let bits = self.register.read();
        let r = OtgHsGahbcfgR { bits: bits };
        let mut w = OtgHsGahbcfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGahbcfgR {
        OtgHsGahbcfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGahbcfgW) -> &mut OtgHsGahbcfgW
    {
        let mut w = OtgHsGahbcfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGahbcfgR {
    bits: u32,
}

impl OtgHsGahbcfgR {
    # [ doc = "Bit 0 - Global interrupt mask" ]
    pub fn gint(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:4 - Burst length/type" ]
    pub fn hbstlen(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - DMA enable" ]
    pub fn dmaen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - TxFIFO empty level" ]
    pub fn txfelvl(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Periodic TxFIFO empty level" ]
    pub fn ptxfelvl(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGahbcfgW {
    bits: u32,
}

impl OtgHsGahbcfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGahbcfgW { bits: 0 }
    }
    # [ doc = "Bit 0 - Global interrupt mask" ]
    pub fn gint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:4 - Burst length/type" ]
    pub fn hbstlen(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - DMA enable" ]
    pub fn dmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - TxFIFO empty level" ]
    pub fn txfelvl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Periodic TxFIFO empty level" ]
    pub fn ptxfelvl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsGusbcfg {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGusbcfg {
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
        where for<'w> F: FnOnce(&OtgHsGusbcfgR, &'w mut OtgHsGusbcfgW) -> &'w mut OtgHsGusbcfgW
    {
        let bits = self.register.read();
        let r = OtgHsGusbcfgR { bits: bits };
        let mut w = OtgHsGusbcfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGusbcfgR {
        OtgHsGusbcfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGusbcfgW) -> &mut OtgHsGusbcfgW
    {
        let mut w = OtgHsGusbcfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGusbcfgR {
    bits: u32,
}

impl OtgHsGusbcfgR {
    # [ doc = "Bits 0:2 - FS timeout calibration" ]
    pub fn tocal(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - SRP-capable" ]
    pub fn srpcap(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - HNP-capable" ]
    pub fn hnpcap(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 10:13 - USB turnaround time" ]
    pub fn trdt(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - PHY Low-power clock select" ]
    pub fn phylpcs(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - ULPI FS/LS select" ]
    pub fn ulpifsls(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - ULPI Auto-resume" ]
    pub fn ulpiar(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - ULPI Clock SuspendM" ]
    pub fn ulpicsm(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - ULPI External VBUS Drive" ]
    pub fn ulpievbusd(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - ULPI external VBUS indicator" ]
    pub fn ulpievbusi(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - TermSel DLine pulsing selection" ]
    pub fn tsdps(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Indicator complement" ]
    pub fn pcci(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Indicator pass through" ]
    pub fn ptci(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - ULPI interface protect disable" ]
    pub fn ulpiipd(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Forced host mode" ]
    pub fn fhmod(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Forced peripheral mode" ]
    pub fn fdmod(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Corrupt Tx packet" ]
    pub fn ctxpkt(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGusbcfgW {
    bits: u32,
}

impl OtgHsGusbcfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGusbcfgW { bits: 2560 }
    }
    # [ doc = "Bits 0:2 - FS timeout calibration" ]
    pub fn tocal(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - USB 2.0 high-speed ULPI PHY or USB 1.1 full-speed serial transceiver select" ]
    pub fn physel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - SRP-capable" ]
    pub fn srpcap(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - HNP-capable" ]
    pub fn hnpcap(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 10:13 - USB turnaround time" ]
    pub fn trdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - PHY Low-power clock select" ]
    pub fn phylpcs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - ULPI FS/LS select" ]
    pub fn ulpifsls(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - ULPI Auto-resume" ]
    pub fn ulpiar(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - ULPI Clock SuspendM" ]
    pub fn ulpicsm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - ULPI External VBUS Drive" ]
    pub fn ulpievbusd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - ULPI external VBUS indicator" ]
    pub fn ulpievbusi(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - TermSel DLine pulsing selection" ]
    pub fn tsdps(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Indicator complement" ]
    pub fn pcci(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Indicator pass through" ]
    pub fn ptci(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - ULPI interface protect disable" ]
    pub fn ulpiipd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Forced host mode" ]
    pub fn fhmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Forced peripheral mode" ]
    pub fn fdmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Corrupt Tx packet" ]
    pub fn ctxpkt(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsGrstctl {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGrstctl {
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
        where for<'w> F: FnOnce(&OtgHsGrstctlR, &'w mut OtgHsGrstctlW) -> &'w mut OtgHsGrstctlW
    {
        let bits = self.register.read();
        let r = OtgHsGrstctlR { bits: bits };
        let mut w = OtgHsGrstctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGrstctlR {
        OtgHsGrstctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGrstctlW) -> &mut OtgHsGrstctlW
    {
        let mut w = OtgHsGrstctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGrstctlR {
    bits: u32,
}

impl OtgHsGrstctlR {
    # [ doc = "Bit 0 - Core soft reset" ]
    pub fn csrst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - HCLK soft reset" ]
    pub fn hsrst(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Host frame counter reset" ]
    pub fn fcrst(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - RxFIFO flush" ]
    pub fn rxfflsh(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - TxFIFO flush" ]
    pub fn txfflsh(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:10 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - DMA request signal" ]
    pub fn dmareq(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - AHB master idle" ]
    pub fn ahbidl(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGrstctlW {
    bits: u32,
}

impl OtgHsGrstctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGrstctlW { bits: 536870912 }
    }
    # [ doc = "Bit 0 - Core soft reset" ]
    pub fn csrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - HCLK soft reset" ]
    pub fn hsrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Host frame counter reset" ]
    pub fn fcrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - RxFIFO flush" ]
    pub fn rxfflsh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - TxFIFO flush" ]
    pub fn txfflsh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:10 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsGintsts {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGintsts {
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
        where for<'w> F: FnOnce(&OtgHsGintstsR, &'w mut OtgHsGintstsW) -> &'w mut OtgHsGintstsW
    {
        let bits = self.register.read();
        let r = OtgHsGintstsR { bits: bits };
        let mut w = OtgHsGintstsW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGintstsR {
        OtgHsGintstsR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGintstsW) -> &mut OtgHsGintstsW
    {
        let mut w = OtgHsGintstsW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGintstsR {
    bits: u32,
}

impl OtgHsGintstsR {
    # [ doc = "Bit 0 - Current mode of operation" ]
    pub fn cmod(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Mode mismatch interrupt" ]
    pub fn mmis(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - OTG interrupt" ]
    pub fn otgint(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Start of frame" ]
    pub fn sof(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - RxFIFO nonempty" ]
    pub fn rxflvl(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Nonperiodic TxFIFO empty" ]
    pub fn nptxfe(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Global IN nonperiodic NAK effective" ]
    pub fn ginakeff(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Global OUT NAK effective" ]
    pub fn boutnakeff(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Early suspend" ]
    pub fn esusp(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - USB suspend" ]
    pub fn usbsusp(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - USB reset" ]
    pub fn usbrst(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Enumeration done" ]
    pub fn enumdne(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Isochronous OUT packet dropped interrupt" ]
    pub fn isoodrp(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - End of periodic frame interrupt" ]
    pub fn eopf(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - IN endpoint interrupt" ]
    pub fn iepint(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - OUT endpoint interrupt" ]
    pub fn oepint(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Incomplete isochronous IN transfer" ]
    pub fn iisoixfr(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer" ]
    pub fn pxfr_incompisoout(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Data fetch suspended" ]
    pub fn datafsusp(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Host port interrupt" ]
    pub fn hprtint(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Host channels interrupt" ]
    pub fn hcint(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Periodic TxFIFO empty" ]
    pub fn ptxfe(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Connector ID status change" ]
    pub fn cidschg(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Disconnect detected interrupt" ]
    pub fn discint(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Session request/new session detected interrupt" ]
    pub fn srqint(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Resume/remote wakeup detected interrupt" ]
    pub fn wkuint(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGintstsW {
    bits: u32,
}

impl OtgHsGintstsW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGintstsW { bits: 67108896 }
    }
    # [ doc = "Bit 1 - Mode mismatch interrupt" ]
    pub fn mmis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Start of frame" ]
    pub fn sof(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Early suspend" ]
    pub fn esusp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - USB suspend" ]
    pub fn usbsusp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - USB reset" ]
    pub fn usbrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Enumeration done" ]
    pub fn enumdne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Isochronous OUT packet dropped interrupt" ]
    pub fn isoodrp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - End of periodic frame interrupt" ]
    pub fn eopf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Incomplete isochronous IN transfer" ]
    pub fn iisoixfr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer" ]
    pub fn pxfr_incompisoout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Data fetch suspended" ]
    pub fn datafsusp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Connector ID status change" ]
    pub fn cidschg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Disconnect detected interrupt" ]
    pub fn discint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Session request/new session detected interrupt" ]
    pub fn srqint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Resume/remote wakeup detected interrupt" ]
    pub fn wkuint(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsGintmsk {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGintmsk {
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
        where for<'w> F: FnOnce(&OtgHsGintmskR, &'w mut OtgHsGintmskW) -> &'w mut OtgHsGintmskW
    {
        let bits = self.register.read();
        let r = OtgHsGintmskR { bits: bits };
        let mut w = OtgHsGintmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGintmskR {
        OtgHsGintmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGintmskW) -> &mut OtgHsGintmskW
    {
        let mut w = OtgHsGintmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGintmskR {
    bits: u32,
}

impl OtgHsGintmskR {
    # [ doc = "Bit 1 - Mode mismatch interrupt mask" ]
    pub fn mmism(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - OTG interrupt mask" ]
    pub fn otgint(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Start of frame mask" ]
    pub fn sofm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Receive FIFO nonempty mask" ]
    pub fn rxflvlm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Nonperiodic TxFIFO empty mask" ]
    pub fn nptxfem(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Global nonperiodic IN NAK effective mask" ]
    pub fn ginakeffm(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Global OUT NAK effective mask" ]
    pub fn gonakeffm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Early suspend mask" ]
    pub fn esuspm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - USB suspend mask" ]
    pub fn usbsuspm(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - USB reset mask" ]
    pub fn usbrst(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Enumeration done mask" ]
    pub fn enumdnem(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask" ]
    pub fn isoodrpm(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - End of periodic frame interrupt mask" ]
    pub fn eopfm(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Endpoint mismatch interrupt mask" ]
    pub fn epmism(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - IN endpoints interrupt mask" ]
    pub fn iepint(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - OUT endpoints interrupt mask" ]
    pub fn oepint(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Incomplete isochronous IN transfer mask" ]
    pub fn iisoixfrm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer mask" ]
    pub fn pxfrm_iisooxfrm(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Data fetch suspended mask" ]
    pub fn fsuspm(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Host port interrupt mask" ]
    pub fn prtim(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Host channels interrupt mask" ]
    pub fn hcim(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Periodic TxFIFO empty mask" ]
    pub fn ptxfem(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Connector ID status change mask" ]
    pub fn cidschgm(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Disconnect detected interrupt mask" ]
    pub fn discint(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Session request/new session detected interrupt mask" ]
    pub fn srqim(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Resume/remote wakeup detected interrupt mask" ]
    pub fn wuim(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGintmskW {
    bits: u32,
}

impl OtgHsGintmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGintmskW { bits: 0 }
    }
    # [ doc = "Bit 1 - Mode mismatch interrupt mask" ]
    pub fn mmism(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - OTG interrupt mask" ]
    pub fn otgint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Start of frame mask" ]
    pub fn sofm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Receive FIFO nonempty mask" ]
    pub fn rxflvlm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Nonperiodic TxFIFO empty mask" ]
    pub fn nptxfem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Global nonperiodic IN NAK effective mask" ]
    pub fn ginakeffm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Global OUT NAK effective mask" ]
    pub fn gonakeffm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Early suspend mask" ]
    pub fn esuspm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - USB suspend mask" ]
    pub fn usbsuspm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - USB reset mask" ]
    pub fn usbrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Enumeration done mask" ]
    pub fn enumdnem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask" ]
    pub fn isoodrpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - End of periodic frame interrupt mask" ]
    pub fn eopfm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Endpoint mismatch interrupt mask" ]
    pub fn epmism(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - IN endpoints interrupt mask" ]
    pub fn iepint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - OUT endpoints interrupt mask" ]
    pub fn oepint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Incomplete isochronous IN transfer mask" ]
    pub fn iisoixfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer mask" ]
    pub fn pxfrm_iisooxfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Data fetch suspended mask" ]
    pub fn fsuspm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Host channels interrupt mask" ]
    pub fn hcim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Periodic TxFIFO empty mask" ]
    pub fn ptxfem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Connector ID status change mask" ]
    pub fn cidschgm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Disconnect detected interrupt mask" ]
    pub fn discint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Session request/new session detected interrupt mask" ]
    pub fn srqim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Resume/remote wakeup detected interrupt mask" ]
    pub fn wuim(&mut self, value: bool) -> &mut Self {
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
pub struct OtgHsGrxstsrHost {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsGrxstsrHost {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsGrxstsrHostR {
        OtgHsGrxstsrHostR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGrxstsrHostR {
    bits: u32,
}

impl OtgHsGrxstsrHostR {
    # [ doc = "Bits 0:3 - Channel number" ]
    pub fn chnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:14 - Byte count" ]
    pub fn bcnt(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 15:16 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 17:20 - Packet status" ]
    pub fn pktsts(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct OtgHsGrxstspHost {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsGrxstspHost {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsGrxstspHostR {
        OtgHsGrxstspHostR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGrxstspHostR {
    bits: u32,
}

impl OtgHsGrxstspHostR {
    # [ doc = "Bits 0:3 - Channel number" ]
    pub fn chnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:14 - Byte count" ]
    pub fn bcnt(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 15:16 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 17:20 - Packet status" ]
    pub fn pktsts(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct OtgHsGrxfsiz {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGrxfsiz {
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
        where for<'w> F: FnOnce(&OtgHsGrxfsizR, &'w mut OtgHsGrxfsizW) -> &'w mut OtgHsGrxfsizW
    {
        let bits = self.register.read();
        let r = OtgHsGrxfsizR { bits: bits };
        let mut w = OtgHsGrxfsizW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGrxfsizR {
        OtgHsGrxfsizR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGrxfsizW) -> &mut OtgHsGrxfsizW
    {
        let mut w = OtgHsGrxfsizW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGrxfsizR {
    bits: u32,
}

impl OtgHsGrxfsizR {
    # [ doc = "Bits 0:15 - RxFIFO depth" ]
    pub fn rxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGrxfsizW {
    bits: u32,
}

impl OtgHsGrxfsizW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGrxfsizW { bits: 512 }
    }
    # [ doc = "Bits 0:15 - RxFIFO depth" ]
    pub fn rxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsGnptxfsizHost {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGnptxfsizHost {
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
        where for<'w> F: FnOnce(&OtgHsGnptxfsizHostR, &'w mut OtgHsGnptxfsizHostW)
                                -> &'w mut OtgHsGnptxfsizHostW
    {
        let bits = self.register.read();
        let r = OtgHsGnptxfsizHostR { bits: bits };
        let mut w = OtgHsGnptxfsizHostW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGnptxfsizHostR {
        OtgHsGnptxfsizHostR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGnptxfsizHostW) -> &mut OtgHsGnptxfsizHostW
    {
        let mut w = OtgHsGnptxfsizHostW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGnptxfsizHostR {
    bits: u32,
}

impl OtgHsGnptxfsizHostR {
    # [ doc = "Bits 0:15 - Nonperiodic transmit RAM start address" ]
    pub fn nptxfsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Nonperiodic TxFIFO depth" ]
    pub fn nptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGnptxfsizHostW {
    bits: u32,
}

impl OtgHsGnptxfsizHostW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGnptxfsizHostW { bits: 512 }
    }
    # [ doc = "Bits 0:15 - Nonperiodic transmit RAM start address" ]
    pub fn nptxfsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Nonperiodic TxFIFO depth" ]
    pub fn nptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsTx0fsizPeripheral {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsTx0fsizPeripheral {
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
        where for<'w> F: FnOnce(&OtgHsTx0fsizPeripheralR, &'w mut OtgHsTx0fsizPeripheralW)
                                -> &'w mut OtgHsTx0fsizPeripheralW
    {
        let bits = self.register.read();
        let r = OtgHsTx0fsizPeripheralR { bits: bits };
        let mut w = OtgHsTx0fsizPeripheralW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsTx0fsizPeripheralR {
        OtgHsTx0fsizPeripheralR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsTx0fsizPeripheralW) -> &mut OtgHsTx0fsizPeripheralW
    {
        let mut w = OtgHsTx0fsizPeripheralW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsTx0fsizPeripheralR {
    bits: u32,
}

impl OtgHsTx0fsizPeripheralR {
    # [ doc = "Bits 0:15 - Endpoint 0 transmit RAM start address" ]
    pub fn tx0fsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Endpoint 0 TxFIFO depth" ]
    pub fn tx0fd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsTx0fsizPeripheralW {
    bits: u32,
}

impl OtgHsTx0fsizPeripheralW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsTx0fsizPeripheralW { bits: 512 }
    }
    # [ doc = "Bits 0:15 - Endpoint 0 transmit RAM start address" ]
    pub fn tx0fsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Endpoint 0 TxFIFO depth" ]
    pub fn tx0fd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsGnptxsts {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsGnptxsts {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsGnptxstsR {
        OtgHsGnptxstsR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGnptxstsR {
    bits: u32,
}

impl OtgHsGnptxstsR {
    # [ doc = "Bits 0:15 - Nonperiodic TxFIFO space available" ]
    pub fn nptxfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:23 - Nonperiodic transmit request queue space available" ]
    pub fn nptqxsav(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:30 - Top of the nonperiodic transmit request queue" ]
    pub fn nptxqtop(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct OtgHsGccfg {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsGccfg {
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
        where for<'w> F: FnOnce(&OtgHsGccfgR, &'w mut OtgHsGccfgW) -> &'w mut OtgHsGccfgW
    {
        let bits = self.register.read();
        let r = OtgHsGccfgR { bits: bits };
        let mut w = OtgHsGccfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsGccfgR {
        OtgHsGccfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsGccfgW) -> &mut OtgHsGccfgW
    {
        let mut w = OtgHsGccfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGccfgR {
    bits: u32,
}

impl OtgHsGccfgR {
    # [ doc = "Bit 16 - Power down" ]
    pub fn pwrdwn(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Enable I2C bus connection for the external I2C PHY interface" ]
    pub fn i2cpaden(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Enable the VBUS sensing device" ]
    pub fn vbusasen(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Enable the VBUS sensing device" ]
    pub fn vbusbsen(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - SOF output enable" ]
    pub fn sofouten(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - VBUS sensing disable option" ]
    pub fn novbussens(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGccfgW {
    bits: u32,
}

impl OtgHsGccfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsGccfgW { bits: 0 }
    }
    # [ doc = "Bit 16 - Power down" ]
    pub fn pwrdwn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Enable I2C bus connection for the external I2C PHY interface" ]
    pub fn i2cpaden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Enable the VBUS sensing device" ]
    pub fn vbusasen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Enable the VBUS sensing device" ]
    pub fn vbusbsen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - SOF output enable" ]
    pub fn sofouten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - VBUS sensing disable option" ]
    pub fn novbussens(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsCid {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsCid {
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
        where for<'w> F: FnOnce(&OtgHsCidR, &'w mut OtgHsCidW) -> &'w mut OtgHsCidW
    {
        let bits = self.register.read();
        let r = OtgHsCidR { bits: bits };
        let mut w = OtgHsCidW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsCidR {
        OtgHsCidR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsCidW) -> &mut OtgHsCidW
    {
        let mut w = OtgHsCidW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsCidR {
    bits: u32,
}

impl OtgHsCidR {
    # [ doc = "Bits 0:31 - Product ID field" ]
    pub fn product_id(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsCidW {
    bits: u32,
}

impl OtgHsCidW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsCidW { bits: 4608 }
    }
    # [ doc = "Bits 0:31 - Product ID field" ]
    pub fn product_id(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsHptxfsiz {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsHptxfsiz {
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
        where for<'w> F: FnOnce(&OtgHsHptxfsizR, &'w mut OtgHsHptxfsizW) -> &'w mut OtgHsHptxfsizW
    {
        let bits = self.register.read();
        let r = OtgHsHptxfsizR { bits: bits };
        let mut w = OtgHsHptxfsizW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsHptxfsizR {
        OtgHsHptxfsizR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsHptxfsizW) -> &mut OtgHsHptxfsizW
    {
        let mut w = OtgHsHptxfsizW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHptxfsizR {
    bits: u32,
}

impl OtgHsHptxfsizR {
    # [ doc = "Bits 0:15 - Host periodic TxFIFO start address" ]
    pub fn ptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Host periodic TxFIFO depth" ]
    pub fn ptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsHptxfsizW {
    bits: u32,
}

impl OtgHsHptxfsizW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsHptxfsizW { bits: 33555968 }
    }
    # [ doc = "Bits 0:15 - Host periodic TxFIFO start address" ]
    pub fn ptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Host periodic TxFIFO depth" ]
    pub fn ptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptxf1 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptxf1 {
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
        where for<'w> F: FnOnce(&OtgHsDieptxf1R, &'w mut OtgHsDieptxf1W) -> &'w mut OtgHsDieptxf1W
    {
        let bits = self.register.read();
        let r = OtgHsDieptxf1R { bits: bits };
        let mut w = OtgHsDieptxf1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptxf1R {
        OtgHsDieptxf1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptxf1W) -> &mut OtgHsDieptxf1W
    {
        let mut w = OtgHsDieptxf1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf1R {
    bits: u32,
}

impl OtgHsDieptxf1R {
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf1W {
    bits: u32,
}

impl OtgHsDieptxf1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptxf1W { bits: 33555456 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptxf2 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptxf2 {
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
        where for<'w> F: FnOnce(&OtgHsDieptxf2R, &'w mut OtgHsDieptxf2W) -> &'w mut OtgHsDieptxf2W
    {
        let bits = self.register.read();
        let r = OtgHsDieptxf2R { bits: bits };
        let mut w = OtgHsDieptxf2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptxf2R {
        OtgHsDieptxf2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptxf2W) -> &mut OtgHsDieptxf2W
    {
        let mut w = OtgHsDieptxf2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf2R {
    bits: u32,
}

impl OtgHsDieptxf2R {
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf2W {
    bits: u32,
}

impl OtgHsDieptxf2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptxf2W { bits: 33555456 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptxf3 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptxf3 {
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
        where for<'w> F: FnOnce(&OtgHsDieptxf3R, &'w mut OtgHsDieptxf3W) -> &'w mut OtgHsDieptxf3W
    {
        let bits = self.register.read();
        let r = OtgHsDieptxf3R { bits: bits };
        let mut w = OtgHsDieptxf3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptxf3R {
        OtgHsDieptxf3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptxf3W) -> &mut OtgHsDieptxf3W
    {
        let mut w = OtgHsDieptxf3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf3R {
    bits: u32,
}

impl OtgHsDieptxf3R {
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf3W {
    bits: u32,
}

impl OtgHsDieptxf3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptxf3W { bits: 33555456 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptxf4 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptxf4 {
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
        where for<'w> F: FnOnce(&OtgHsDieptxf4R, &'w mut OtgHsDieptxf4W) -> &'w mut OtgHsDieptxf4W
    {
        let bits = self.register.read();
        let r = OtgHsDieptxf4R { bits: bits };
        let mut w = OtgHsDieptxf4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptxf4R {
        OtgHsDieptxf4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptxf4W) -> &mut OtgHsDieptxf4W
    {
        let mut w = OtgHsDieptxf4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf4R {
    bits: u32,
}

impl OtgHsDieptxf4R {
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf4W {
    bits: u32,
}

impl OtgHsDieptxf4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptxf4W { bits: 33555456 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptxf5 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptxf5 {
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
        where for<'w> F: FnOnce(&OtgHsDieptxf5R, &'w mut OtgHsDieptxf5W) -> &'w mut OtgHsDieptxf5W
    {
        let bits = self.register.read();
        let r = OtgHsDieptxf5R { bits: bits };
        let mut w = OtgHsDieptxf5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptxf5R {
        OtgHsDieptxf5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptxf5W) -> &mut OtgHsDieptxf5W
    {
        let mut w = OtgHsDieptxf5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf5R {
    bits: u32,
}

impl OtgHsDieptxf5R {
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf5W {
    bits: u32,
}

impl OtgHsDieptxf5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptxf5W { bits: 33555456 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptxf6 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptxf6 {
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
        where for<'w> F: FnOnce(&OtgHsDieptxf6R, &'w mut OtgHsDieptxf6W) -> &'w mut OtgHsDieptxf6W
    {
        let bits = self.register.read();
        let r = OtgHsDieptxf6R { bits: bits };
        let mut w = OtgHsDieptxf6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptxf6R {
        OtgHsDieptxf6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptxf6W) -> &mut OtgHsDieptxf6W
    {
        let mut w = OtgHsDieptxf6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf6R {
    bits: u32,
}

impl OtgHsDieptxf6R {
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf6W {
    bits: u32,
}

impl OtgHsDieptxf6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptxf6W { bits: 33555456 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsDieptxf7 {
    register: ::volatile_register::RW<u32>,
}

impl OtgHsDieptxf7 {
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
        where for<'w> F: FnOnce(&OtgHsDieptxf7R, &'w mut OtgHsDieptxf7W) -> &'w mut OtgHsDieptxf7W
    {
        let bits = self.register.read();
        let r = OtgHsDieptxf7R { bits: bits };
        let mut w = OtgHsDieptxf7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgHsDieptxf7R {
        OtgHsDieptxf7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgHsDieptxf7W) -> &mut OtgHsDieptxf7W
    {
        let mut w = OtgHsDieptxf7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf7R {
    bits: u32,
}

impl OtgHsDieptxf7R {
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsDieptxf7W {
    bits: u32,
}

impl OtgHsDieptxf7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgHsDieptxf7W { bits: 33555456 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct OtgHsGrxstsrPeripheral {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsGrxstsrPeripheral {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsGrxstsrPeripheralR {
        OtgHsGrxstsrPeripheralR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGrxstsrPeripheralR {
    bits: u32,
}

impl OtgHsGrxstsrPeripheralR {
    # [ doc = "Bits 0:3 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:14 - Byte count" ]
    pub fn bcnt(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 15:16 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 17:20 - Packet status" ]
    pub fn pktsts(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:24 - Frame number" ]
    pub fn frmnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct OtgHsGrxstspPeripheral {
    register: ::volatile_register::RO<u32>,
}

impl OtgHsGrxstspPeripheral {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> OtgHsGrxstspPeripheralR {
        OtgHsGrxstspPeripheralR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgHsGrxstspPeripheralR {
    bits: u32,
}

impl OtgHsGrxstspPeripheralR {
    # [ doc = "Bits 0:3 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:14 - Byte count" ]
    pub fn bcnt(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 15:16 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 17:20 - Packet status" ]
    pub fn pktsts(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:24 - Frame number" ]
    pub fn frmnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}
