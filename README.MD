[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![crates.io](https://img.shields.io/crates/v/sispm.svg)](https://crates.io/crates/sispm)
[![Released API docs](https://docs.rs/sispm/badge.svg)](https://docs.rs/sispm)
[![GitHub](https://img.shields.io/github/license/User65k/sispm)](./LICENSE)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/User65k/sispm/Rust)

Allows to control Gembird SIS-PM USB outlet devices via USB.

Rust port of [python-sispm](https://github.com/jerch/python-sispm).
See also [sispmctl](https://sourceforge.net/projects/sispmctl/) (C deamon)

# Permissions

Per default, only root is allowed to use devices directly, therefore the SiS-PM also only works as root.

To allow group sispmctl access create file `/lib/udev/rules.d/60-sispmctl.rules` with the following content

    SUBSYSTEM=="usb", ATTR{idVendor}=="04b4", ATTR{idProduct}=="fd10", GROUP="sispmctl", MODE="660"
    SUBSYSTEM=="usb", ATTR{idVendor}=="04b4", ATTR{idProduct}=="fd11", GROUP="sispmctl", MODE="660"
    SUBSYSTEM=="usb", ATTR{idVendor}=="04b4", ATTR{idProduct}=="fd12", GROUP="sispmctl", MODE="660"
    SUBSYSTEM=="usb", ATTR{idVendor}=="04b4", ATTR{idProduct}=="fd13", GROUP="sispmctl", MODE="660"
    SUBSYSTEM=="usb", ATTR{idVendor}=="04b4", ATTR{idProduct}=="fd15", GROUP="sispmctl", MODE="660"

Then reload the udev rules with

    udevadm control --reload-rules

