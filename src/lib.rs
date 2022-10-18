/*!
 * Allows to control Gembird SIS-PM USB outlet devices via USB.
 * 
 * Rust port of [python-sispm](https://github.com/jerch/python-sispm).
 * See also [sispmctl](https://sourceforge.net/projects/sispmctl/) (C deamon)
 * 
 # Example
```
for device in sispm::get_devices().expect("devs") {
    println!("{:?}", device);        
    println!("serial: {:x?}", device.get_serial());
    println!("status: {}",
        device.get_status(1).expect("1"),
    );
    device.toggle(1).expect("tgl");
}
```
 */
use core::time::Duration;
use core::fmt::Debug;

use rusb::{Device, devices, UsbContext, Result, GlobalContext};

/// Gembird vendor_id
const VENDOR: u16 = 0x04B4;

/// known working devices
const DEVICES: [u16; 5] = [0xFD10, 0xFD11, 0xFD12, 0xFD13, 0xFD15];

pub type GlobalSiSPM = SiSPM<GlobalContext>;

pub struct SiSPM<T: UsbContext> {
    device: Device<T>
}

/// Get all known Gembird devices currently connected
pub fn get_devices() -> Result<Vec<GlobalSiSPM>> {
    let mut ret = Vec::new();
    for device in devices()?.iter() {
        let device_desc = device.device_descriptor()?;
        if VENDOR == device_desc.vendor_id() && DEVICES.contains(&device_desc.product_id()) {
            ret.push(SiSPM {device});
        }
    }
    Ok(ret)
}

impl<T: UsbContext> Debug for SiSPM<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bus {:03} Device {:03}",
                self.device.bus_number(),
                self.device.address()))?;
        if let Ok(desc) = self.device.device_descriptor() {
            f.write_fmt(format_args!(" ID {:04x}:{:04x}",
            desc.vendor_id(),
            desc.product_id()))?
        }
        Ok(())
    }
}

impl<T: UsbContext> SiSPM<T> {
    fn usb_cmd(&self,
        request_type: u8,
        request: u8,
        value: u16,
        buf: &mut [u8]) -> Result<usize> {
        let d = self.device.open()?;
        d.read_control(request_type, request, (0x03 << 8) | value, 0, buf, Duration::from_millis(500))
    }
    /// Query the devices serial number
    pub fn get_serial(&self) -> Result<Vec<u8>> {
        let mut buf = vec![0;6];
        let new_len = self.usb_cmd(0xa1, 0x01, 1, &mut buf)?;
        unsafe {buf.set_len(new_len);}
        Ok(buf)
    }
    /// Query the status of socket number `num`. `true` if turned on.
    pub fn get_status(&self, num: u8) -> Result<bool> {
        let mut buf = [0;2];
        buf[0] = num*3;
        let r = self.usb_cmd(0x21 | 0x80, 0x01, num as u16 * 3, &mut buf)?;
        if r == 2 {
            //buf[0] == num*3
            Ok(buf[1]!=0)//==0xb ?
        }else{
            Err(rusb::Error::Other)
        }
    }
    /// Set the status of socket number `num`. `true` turns it on.
    pub fn set_status(&self, num: u8, on: bool) -> Result<()> {
        let mut buf = [0;2];
        buf[0] = num*3;
        if on {
            buf[1] = 3;
        }
        //let _ = self.usb_cmd(0x21, 0x09, num as u16 * 3, &mut buf)?;
        let d = self.device.open()?;
        let _ = d.write_control(0x21, 0x09, (0x03 << 8) | (num as u16 * 3), 0, &buf, Duration::from_millis(500))?;
        
        Ok(())
    }
    /// Toggle the status of socket number `num`.
    pub fn toggle(&self, num: u8) -> Result<()> {
        self.set_status(num, !self.get_status(num)?)
    }
}
