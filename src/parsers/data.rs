pub trait Conversion<'a> {
    fn to_f64(self) -> f64;
    fn to_string(self) -> String;
}

impl<'a> Conversion<'a> for &'a [u8] {
    fn to_f64(self) -> f64 {
        self.to_string().parse::<f64>().unwrap()
    }

    fn to_string(self) -> String {
        String::from_utf8_lossy(self).to_string()
    }
}
