pub mod cst;
pub mod token;

#[cfg(test)]
mod tests {
    use crate::cst::module;
    use crate::token::tokenize;

    fn parse_module(source: &str) {
        let tokens = tokenize().parse(source).unwrap();
        module().parse(tokens).unwrap();
    }

    #[test]
    fn add_two() {
        parse_module(include_str!("examples/add_two.pika"));
    }

    #[test]
    fn bijele() {
        parse_module(include_str!("examples/kattis/bijele.pika"));
    }

    #[test]
    fn bluetooth() {
        parse_module(include_str!("examples/kattis/bluetooth.pika"));
    }
}
