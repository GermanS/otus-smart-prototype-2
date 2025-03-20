// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

use std::sync::Arc;

use lesson_3::{
    BorrowingDeviceInfoProvider, OwningDeviceInfoProvider, SmartHouse, SmartRoom, SmartSocket,
    SmartThermometer,
};

fn main() {
    // /**
    // Инициализация устройств
    let socket1 = SmartSocket::new("foo".to_string());
    let socket2 = SmartSocket::new("bar".to_string());
    let thermo = SmartThermometer::new("baz".to_string());

    // Инициализация дома
    let mut house = SmartHouse::new("quix".to_string());
    let mut room = SmartRoom::new("foobar".to_string());

    if let Err(e) = room.plug(Arc::new(socket1.clone())) {
        panic!("{e}");
    }

    if let Err(e) = room.plug(Arc::new(thermo.clone())) {
        panic!("{e}");
    }

    if let Err(e) = house.add(room) {
        panic!("{e}");
    }

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    match house.create_report(info_provider_1) {
        Ok(v) => println!("Report #1:\n {}", v),
        Err(e) => println!("Report #1 (Error): {e}"),
    }

    // // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };

    match house.create_report(info_provider_2) {
        Ok(v) => println!("Report #2:\n {}", v),
        Err(e) => println!("Report #2 Error: {e}"),
    };
}
