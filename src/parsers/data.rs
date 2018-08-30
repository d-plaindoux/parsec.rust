#[derive(Debug)]
pub struct FloatLiteral<'a>(pub &'a [u8], pub usize, pub usize);

#[derive(Debug)]
pub struct StringLiteral<'a>(pub &'a [u8], pub usize, pub usize);

pub trait Evaluation<A> {
    fn to_native_value(&self) -> A;
    fn to_string(&self) -> String;
}

fn to_string(s: &[u8], o: usize, n: usize) -> String {
    String::from_utf8_lossy(&s[o..n]).to_string()
}

impl<'a> Evaluation<f64> for FloatLiteral<'a> {
    fn to_native_value(&self) -> f64 {
        self.to_string().parse::<f64>().unwrap()
    }

    fn to_string(&self) -> String {
        let FloatLiteral(s, o, n) = self;

        to_string(s, *o, *n)
    }
}

impl<'a> Evaluation<String> for StringLiteral<'a> {
    fn to_native_value(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        let StringLiteral(s, o, n) = self;

        to_string(s, *o, *n)
    }
}
