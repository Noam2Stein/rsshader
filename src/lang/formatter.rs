#[derive(Debug)]
pub struct Formatter<'o> {
    output: Option<&'o mut [u8]>,
    output_len: usize,
}

impl<'o> Formatter<'o> {
    pub const fn with_output(output: &'o mut [u8]) -> Self {
        Self {
            output: Some(output),
            output_len: 0,
        }
    }

    pub const fn without_output() -> Self {
        Self {
            output: None,
            output_len: 0,
        }
    }

    pub const fn output_len(&self) -> usize {
        self.output_len
    }

    pub const fn write_char(&mut self, c: char) {
        if let Some(output) = &mut self.output {
            c.encode_utf8(output);
            // SAFETY: new slice is in bounds of `output`
            *output = unsafe {
                core::slice::from_raw_parts_mut(
                    output.as_mut_ptr().add(c.len_utf8()),
                    (*output).len() - c.len_utf8(),
                )
            };
        }

        self.output_len += c.len_utf8();
    }

    pub const fn write_str(&mut self, s: &str) {
        if let Some(output) = &mut self.output {
            // SAFETY: new slice is in bounds of `output`
            unsafe { core::slice::from_raw_parts_mut(output.as_mut_ptr(), s.len()) }
                .copy_from_slice(s.as_bytes());

            // SAFETY: new slice is in bounds of `output`
            *output = unsafe {
                core::slice::from_raw_parts_mut(
                    output.as_mut_ptr().add(s.len()),
                    (*output).len() - s.len(),
                )
            };
        }

        self.output_len += s.len();
    }

    pub const fn write_i128(&mut self, mut value: i128) {
        if value == 0 {
            self.write_char('0');
            return;
        }

        if value < 0 {
            self.write_char('-');
            value = -value;
        }

        let mut digits = [b'_'; 40];
        let mut digit_count = 0;

        while value != 0 {
            let digit = value % 10;
            value /= 10;

            digits[digits.len() - 1 - digit_count] = b'0' + digit as u8;
            digit_count += 1;
        }

        // SAFETY: the end of the slice is always `digits.len()`
        let bytes = unsafe {
            core::slice::from_raw_parts(
                digits.as_ptr().add(digits.len() - digit_count),
                digit_count,
            )
        };

        // The bytes are guaranteed to be b'0'..=b'9'
        let s = unsafe { str::from_utf8_unchecked(bytes) };

        self.write_str(s);
    }

    pub const fn write_u32_hex(&mut self, value: u32) {
        let value_digits = [
            (value >> 28) & 0x0000000F,
            (value >> 24) & 0x0000000F,
            (value >> 20) & 0x0000000F,
            (value >> 16) & 0x0000000F,
            (value >> 12) & 0x0000000F,
            (value >> 8) & 0x0000000F,
            (value >> 4) & 0x0000000F,
            (value >> 0) & 0x0000000F,
        ];

        let mut i = 0;
        while i < value_digits.len() {
            let digit = value_digits[i];

            let hex_digit = match digit {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'A',
                11 => 'B',
                12 => 'C',
                13 => 'D',
                14 => 'E',
                15 => 'F',
                16.. => panic!(),
            };

            self.write_char(hex_digit);
            i += 1;
        }
    }
}
