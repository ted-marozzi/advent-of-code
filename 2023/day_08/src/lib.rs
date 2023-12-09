use std::collections::HashMap;

pub mod part_1;
pub mod part_2;

#[derive(Debug, Clone, Copy)]
struct Register<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_registers<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, Register<'a>> {
    let mut registers = HashMap::<&str, Register>::new();

    lines.for_each(|line| {
        let mut line_parts = line.split(" = (");
        let key = line_parts.next().unwrap();
        let binding = line_parts.next().unwrap();
        let mut value = binding.split(", ");

        registers.insert(
            key,
            Register {
                left: value.next().unwrap(),
                right: value.next().unwrap().trim_end_matches(')'),
            },
        );
    });

    registers
}
