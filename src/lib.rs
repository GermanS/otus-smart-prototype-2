use core::fmt;
use std::{error::Error, sync::Arc};

pub trait Named {
    fn name(&self) -> &str;
}
pub trait Pluggable: Named {}

#[derive(Debug, Clone)]
pub struct SmartSocket {
    name: String,
}

impl SmartSocket {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Named for SmartSocket {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Pluggable for SmartSocket {}

#[derive(Clone)]
pub struct SmartThermometer {
    name: String,
}

impl SmartThermometer {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Named for SmartThermometer {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Pluggable for SmartThermometer {}

#[derive(Clone)]
pub struct SmartRoom {
    name: String,
    devices: Vec<Arc<dyn Pluggable>>,
}

impl SmartRoom {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: Vec::default(),
        }
    }

    pub fn plug(&mut self, device: Arc<dyn Pluggable>) -> Result<(), Box<dyn Error>> {
        match &self.devices.iter().find(|&d| d.name() == device.name()) {
            Some(_) => Err(format!("Device with name {} already pluged", device.name()).into()),
            None => {
                self.devices.push(device);
                Ok(())
            }
        }
    }

    pub fn is_connected(&self, device: &dyn Pluggable) -> bool {
        self.devices.iter().any(|d| d.name() == device.name())
    }

    pub fn devices(&self) -> Vec<String> {
        self.devices.iter().map(|d| d.name().to_string()).collect()
    }
}

impl Named for SmartRoom {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub struct SmartHouse {
    name: String,
    rooms: Vec<SmartRoom>,
}

impl SmartHouse {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: Vec::default(),
        }
    }

    pub fn add(&mut self, room: SmartRoom) -> Result<(), Box<dyn Error>> {
        match self.get_rooms().iter().find(|&v| v.name() == room.name()) {
            Some(_) => Err(format!("room {} already constructed", room.name()).into()),
            None => {
                self.rooms.push(room);

                Ok(())
            }
        }
    }

    fn get_rooms(&self) -> &[SmartRoom] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        &self.rooms
    }

    // fn devices(&self, room: &str) -> Vec<String> {
    //     // Размер возвращаемого массива можно выбрать самостоятельно

    //     for r in &self.rooms {
    //         if r.name() == room {
    //             return  r.devices();
    //         }
    //     }

    //     Vec::new()
    // }

    pub fn create_report<T: Reportable>(&self, report: T) -> Result<String, Box<dyn Error>> {
        report.make(self)
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----> Device: Socket[{}]", self.name())
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----> Device: Thermometer[{}]", self.name())
    }
}

impl fmt::Display for SmartRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--> Room: {}", self.name())
    }
}

impl fmt::Display for SmartHouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-> House: {}", self.name)
    }
}

pub trait Reportable {
    fn make(&self, house: &SmartHouse) -> Result<String, Box<dyn Error>>;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

impl Reportable for OwningDeviceInfoProvider {
    fn make(&self, house: &SmartHouse) -> Result<String, Box<dyn Error>> {
        for room in house.rooms.iter() {
            if room.is_connected(&self.socket) {
                let out = format!("{} {} {}", house, room, &self.socket);

                return Ok(out);
            }
        }

        Err("Device not found".into())
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl Reportable for BorrowingDeviceInfoProvider<'_, '_> {
    fn make(&self, house: &SmartHouse) -> Result<String, Box<dyn Error>> {
        let mut plugged_socket_room = None;
        let mut plugged_thermo_room = None;

        for room in house.get_rooms().iter() {
            if room.is_connected(self.socket) {
                plugged_socket_room = Some(room);
            }

            if room.is_connected(self.thermo) {
                plugged_thermo_room = Some(room);
            }
        }

        if plugged_thermo_room.is_none() && plugged_socket_room.is_none() {
            return Err("Devices not found".into());
        }

        let mut out;

        if plugged_socket_room.is_some() && plugged_thermo_room.is_some() {
            let plugged_socket_room = plugged_socket_room.unwrap();
            let plugged_thermo_room = plugged_thermo_room.unwrap();

            if plugged_socket_room.name() == plugged_thermo_room.name() {
                out = format!(
                    "{} {} {} {}",
                    house, plugged_socket_room, self.socket, self.thermo
                );
            } else {
                out = format!(
                    "{} {} {} {} {}",
                    house, plugged_socket_room, self.socket, plugged_thermo_room, self.thermo
                );
            }
        } else {
            match plugged_socket_room.is_some() {
                true => {
                    out = format!("{} {} {}", house, plugged_socket_room.unwrap(), self.socket);
                }
                false => {
                    out = format!("not found {}", self.socket);
                }
            };

            match plugged_thermo_room.is_some() {
                true => {
                    out = format!(
                        "{}\n {} {} {}",
                        out,
                        house,
                        plugged_thermo_room.unwrap(),
                        self.thermo
                    );
                }
                false => {
                    out = format!("{} not found {}", out, self.thermo);
                }
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_house() {
        let mut hell = SmartHouse::new("hell".to_string());
        let limb = SmartRoom::new("limb".to_string());
        let lust = SmartRoom::new("lust".to_string());

        assert!(!hell.add(limb).is_err(), "Limb should not be added before");
        assert!(!hell.add(lust).is_err(), "Lust should not be added before");

        let limb = SmartRoom::new("limb".to_string());
        assert!(hell.add(limb).is_err(), "Limb has already been added")
    }

    #[test]
    fn plug_devices() {
        let mut boiler = SmartRoom::new("Boiler".to_string());

        let thermo = SmartThermometer::new("Thermometer 1".to_string());
        let socket = SmartSocket::new("Main socket".to_string());

        assert!(
            !boiler.plug(Arc::new(thermo)).is_err(),
            "Thermometer successfully connected"
        );
        assert!(
            !boiler.plug(Arc::new(socket)).is_err(),
            "Socket successfully connected"
        );

        let socket = SmartSocket::new("Main socket".to_string());
        assert!(
            boiler.plug(Arc::new(socket)).is_err(),
            "Socket already connected"
        );
    }
}
