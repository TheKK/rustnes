use nom::le_u8;

#[derive(Debug)]
pub struct INes<'n> {
    header: Header,
    trainer: Option<&'n [u8]>,
    prg_rom_data: &'n [u8],
    chr_rom_data: Option<&'n [u8]>,
    play_choice_inst_rom: Option<&'n [u8]>,
    // TODO Skip this first
    // play_choice_prom: Option<&'static [u8]>,
}

#[derive(Debug)]
pub struct Header {
    prg_rom_size: u8,
    chr_rom_size: u8,
    flag6: Flag6,
    flag7: Flag7,
    prg_ram_size: u8,
    flag9: Flag9,
    // TODO Skip this first
    // flag10: Flag10,
}

#[derive(Debug, PartialEq)]
pub struct Flag6 {
    mirroring: Mirroring,
    contains_battery_backed_prg_ram: bool,
    has_trainer: bool,
    igore_mirroring_control: bool,
    lower_nybble_of_mapper_number: u8,
}

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq)]
pub struct Flag7 {
    vs_unisystem: bool,
    has_play_choice_10: bool,
    is_nes_2_0: bool,
    upper_nybble_of_mapper_numbe: u8,
}

#[derive(Debug, PartialEq)]
pub struct Flag9 {
    tv_system: TvSystem,
}

#[derive(Debug, PartialEq)]
pub enum TvSystem {
    NTSC,
    PAL,
}

named!(pub parse_ines(&[u8]) -> INes,
     do_parse!(
         header: parse_header >>
         trainer: cond!(header.flag6.has_trainer, take!(512)) >>
         prg_rom_data: take!(16384 * header.prg_rom_size as u32) >>
         chr_rom_data: cond!(header.chr_rom_size != 0, take!(8192 * header.chr_rom_size as u32)) >>
         play_choice_inst_rom: cond!(header.flag7.has_play_choice_10, take!(8192)) >>

         (
             INes {
                 header: header,
                 trainer: trainer,
                 prg_rom_data: prg_rom_data,
                 chr_rom_data: chr_rom_data,
                 play_choice_inst_rom: play_choice_inst_rom,
             }
         )
     )
);

named!(parse_header(&[u8]) -> Header,
    do_parse!(
        tag!(&[b'N', b'E', b'S', 0x1a][..]) >>
        prg_rom_size: le_u8 >>
        chr_rom_size: le_u8 >>
        flag6: parse_flag6 >>
        flag7: parse_flag7 >>
        prg_ram_size: le_u8 >>
        flag9: parse_flag9 >>
        flag10: le_u8 >>
        count!(tag!(&[0x00]), 5) >>

        (
            Header {
                prg_rom_size: prg_rom_size,
                chr_rom_size: chr_rom_size,
                flag6: flag6,
                flag7: flag7,
                prg_ram_size: prg_ram_size,
                flag9: flag9,
            }
        )
    )
);

named!(parse_flag6(&[u8]) -> Flag6,
    do_parse!(
        data: le_u8 >>

        ({
            let mirroring = match data & 0b00000001 {
                0b0 => Mirroring::Horizontal,
                _ => Mirroring::Vertical,
            };

            Flag6 {
                mirroring: mirroring,
                contains_battery_backed_prg_ram: (data & 0b00000010) != 0b0,
                has_trainer: (data & 0b00000100) != 0b0,
                igore_mirroring_control: (data & 0b00001000) != 0b0,
                lower_nybble_of_mapper_number: data >> 4,
            }
        })
    )
);

named!(parse_flag7(&[u8]) -> Flag7,
    do_parse!(
        data: le_u8 >>

        (
            Flag7 {
                vs_unisystem: (data & 0b00000001) != 0b0,
                has_play_choice_10: (data & 0b00000010) != 0b0,
                is_nes_2_0: (data & 0b00001100) == 0b1000,
                upper_nybble_of_mapper_numbe: (data >> 4) << 4,
            }
        )
    )
);

named!(parse_flag9(&[u8]) -> Flag9,
    do_parse!(
        data: le_u8 >>

        ({
            let tv_system = match data & 0b00000001 {
                0b0 => TvSystem::NTSC,
                _ => TvSystem::PAL,
            };

            Flag9 {
                tv_system: tv_system,
            }
        })
    )
);

#[cfg(test)]
mod test {
    pub use nom::IResult;
    pub use super::*;

    mod header_parser {
        use super::*;

        #[test]
        fn should_be_parsed_correctly() {
            let header = &[
                b'N',
                b'E',
                b'S',
                0x1a,
                0x11,
                0x11,
                0x11,
                0x11,
                0x11,
                0x11,
                0x11,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
            ]
                [..];

            match parse_header(header) {
                IResult::Error(e) => panic!("{}", e),
                _ => {}
            }
        }

        #[test]
        fn last_five_byte_are_not_zero() {
            let header = &[
                b'N',
                b'E',
                b'S',
                0x1a,
                0x11,
                0x11,
                0x11,
                0x11,
                0x11,
                0x11,
                0x11,
                0x00,
                0x00,
                0x00,
                0x00,
                0x11,
            ]
                [..];

            match parse_header(header) {
                IResult::Error(_) => {}
                _ => panic!("Last five bytes are not all zero"),
            }
        }
    }

    mod flag6_parser {
        use super::*;

        #[test]
        fn should_be_parsed_correctly() {
            let raw_byte = &[0b01011011][..];
            let expected = Flag6 {
                mirroring: Mirroring::Vertical,
                contains_battery_backed_prg_ram: true,
                has_trainer: false,
                igore_mirroring_control: true,
                lower_nybble_of_mapper_number: 0b0101,
            };

            let (_, actual) = parse_flag6(raw_byte).unwrap();

            assert_eq!(expected, actual);
        }
    }

    mod flag7_parser {
        use super::*;

        #[test]
        fn should_be_parsed_correctly() {
            let raw_byte = &[0b01111001][..];
            let expected = Flag7 {
                vs_unisystem: true,
                has_play_choice_10: false,
                is_nes_2_0: true,
                upper_nybble_of_mapper_numbe: 0b01110000,
            };

            let (_, actual) = parse_flag7(raw_byte).unwrap();

            assert_eq!(expected, actual);
        }
    }

    mod flag9_parser {
        use super::*;

        #[test]
        fn should_be_parsed_correctly() {
            let raw_byte = &[0b00000001][..];
            let expected = Flag9 { tv_system: TvSystem::PAL };

            let (_, actual) = parse_flag9(raw_byte).unwrap();

            assert_eq!(expected, actual);
        }
    }
}
