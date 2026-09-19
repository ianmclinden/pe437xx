macro_rules! test_pe437xx_addr {
    ($ic:ident, $name:ident) => {
        mod $name {
            use embedded_hal_mock::eh1::{
                digital::{Mock as DigitalMock, State, Transaction as DigitalTransaction},
                spi::{Mock as SpiMock, Transaction as SpiTransaction},
            };
            use pe437xx::{spi::*, Address, Attenuation};

            #[test]
            fn test_init() {
                let le_expects = [DigitalTransaction::set(State::Low)];

                let spi = SpiMock::new(&[]); // Init should not write to SPI
                let le = DigitalMock::new(&le_expects);

                let addr = Address::new(0x5).expect("Known address was invalid");
                let ic = $ic::new(spi, le, addr)
                    .expect(&format!("Failed to initialize {}", stringify!($ic)));

                assert_eq!(addr, ic.address());

                let (mut spi, mut le) = ic.destroy();

                spi.done();
                le.done();
            }

            #[test]
            fn test_set_attenuation() {
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
                let mut ic = $ic::new(spi, le, addr)
                    .unwrap_or_else(|_| panic!("Failed to initialize {}", stringify!($ic)));

                assert_eq!(Attenuation::MAX, ic.attenuation());

                let att = Attenuation::from_steps(STEPS).expect("Invalid attenuation");
                ic.set_attenuation(att).expect("Failed to set attenuation");

                assert_eq!(att, ic.attenuation());

                let (mut spi, mut le) = ic.destroy();

                spi.done();
                le.done();
            }
        }
    };
}

macro_rules! test_pe437xx_noaddr {
    ($ic:ident, $name:ident) => {
        mod $name {
            use embedded_hal_mock::eh1::{
                digital::{Mock as DigitalMock, State, Transaction as DigitalTransaction},
                spi::{Mock as SpiMock, Transaction as SpiTransaction},
            };
            use pe437xx::{spi::*, Attenuation};

            #[test]
            fn test_init() {
                let le_expects = [DigitalTransaction::set(State::Low)];

                let spi = SpiMock::new(&[]);
                let le = DigitalMock::new(&le_expects);

                let ic =
                    $ic::new(spi, le).expect(&format!("Failed to initialize {}", stringify!($ic)));

                let (mut spi, mut le) = ic.destroy();

                spi.done();
                le.done();
            }

            #[test]
            fn test_set_attenuation() {
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

                let mut ic =
                    $ic::new(spi, le).expect(&format!("Failed to initialize {}", stringify!($ic)));

                assert_eq!(Attenuation::MAX, ic.attenuation());

                let att = Attenuation::from_steps(STEPS).expect("Invalid attenuation");
                ic.set_attenuation(att).expect("Failed to set attenuation");

                assert_eq!(att, ic.attenuation());

                let (mut spi, mut le) = ic.destroy();

                spi.done();
                le.done();
            }
        }
    };
}

mod blocking {
    test_pe437xx_addr!(PE43701, pe43701);
    test_pe437xx_addr!(PE43703, pe43703);
    test_pe437xx_addr!(PE43704, pe43704);
    test_pe437xx_addr!(PE43705, pe43705);
    test_pe437xx_addr!(PE43712, pe43712);
    test_pe437xx_addr!(PE43713, pe43713);

    test_pe437xx_noaddr!(PE43702, pe43702);
    test_pe437xx_noaddr!(PE43711, pe43711);
}
