use embedded_hal_mock::eh1::{
    digital::{Mock as DigitalMock, State, Transaction as DigitalTransaction},
    spi::{Mock as SpiMock, Transaction as SpiTransaction},
};
use pe437xx::{spi::*, Address, Attenuation};

macro_rules! test_pe437xx_addr {
    ($ic:ident) => { paste::item! {
        #[test]
        fn [<test_ $ic:lower _init>]() {
            let le_expects = [DigitalTransaction::set(State::Low)];

            let spi = SpiMock::new(&[]); // Init should not write to SPI
            let le = DigitalMock::new(&le_expects);

            let addr = Address::new(0x0).expect("Known address was invalid");
            let ic = $ic::new(spi, le, addr).expect(&format!("Failed to initialize {}", stringify!($ic)));

            let (mut spi, mut le) = ic.destroy();

            spi.done();
            le.done();
        }

        #[test]
        fn [<test_ $ic:lower _set_attenuation>]() {
            const STEPS: u8 = 0x5F;
            const ADDR: u8 = 0x05;

            let le_expects = [
                // Init
                DigitalTransaction::set(State::Low),
                // Latch
                DigitalTransaction::set(State::High),
                DigitalTransaction::set(State::Low),
            ];
            let spi_expects = [
                SpiTransaction::transaction_start(),
                SpiTransaction::write_vec(vec![STEPS, ADDR]),
                SpiTransaction::transaction_end(),
            ];

            let spi = SpiMock::new(&spi_expects);
            let le = DigitalMock::new(&le_expects);

            let addr = Address::new(ADDR).expect("Known address was invalid");
            let mut ic = $ic::new(spi, le, addr).expect(&format!("Failed to initialize {}", stringify!($ic)));

            let att = Attenuation::from_steps(STEPS).expect("Invalid attenuation");
            ic.set_attenuation(att).expect("Failed to set attenuation");

            let (mut spi, mut le) = ic.destroy();

            spi.done();
            le.done();
        }
    }
    };
}

test_pe437xx_addr!(PE43701);
test_pe437xx_addr!(PE43703);
test_pe437xx_addr!(PE43704);
test_pe437xx_addr!(PE43705);
test_pe437xx_addr!(PE43712);
test_pe437xx_addr!(PE43713);

macro_rules! test_pe437xx_noaddr {
    ($ic:ident) => { paste::item! {
        #[test]
        fn [<test_ $ic:lower _init>]() {
            let le_expects = [DigitalTransaction::set(State::Low)];

            let spi = SpiMock::new(&[]);
            let le = DigitalMock::new(&le_expects);

            let ic = $ic::new(spi, le).expect(&format!("Failed to initialize {}", stringify!($ic)));

            let (mut spi, mut le) = ic.destroy();

            spi.done();
            le.done();
        }

        #[test]
        fn [<test_ $ic:lower _set_attenuation>]() {
            const STEPS: u8 = 0x5F;

            let le_expects = [
                DigitalTransaction::set(State::Low),
                DigitalTransaction::set(State::High),
                DigitalTransaction::set(State::Low),
            ];
            let spi_expects = [
                SpiTransaction::transaction_start(),
                SpiTransaction::write_vec(vec![STEPS]),
                SpiTransaction::transaction_end(),
            ];

            let spi = SpiMock::new(&spi_expects);
            let le = DigitalMock::new(&le_expects);

            let mut ic = $ic::new(spi, le).expect(&format!("Failed to initialize {}", stringify!($ic)));

            let att = Attenuation::from_steps(STEPS).expect("Invalid attenuation");
            ic.set_attenuation(att).expect("Failed to set attenuation");

            let (mut spi, mut le) = ic.destroy();

            spi.done();
            le.done();
        }
    }
    };
}

test_pe437xx_noaddr!(PE43702);
test_pe437xx_noaddr!(PE43711);
