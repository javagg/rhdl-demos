use rhdl::prelude::*;

#[derive(PartialEq, Digital)]
pub struct Simple {
    a: bool,
    b: b8,
    c: s8,
}

// let simple = Simple {
//     a: true,
//     b: Bits::from(0b10101010),
// };

#[derive(PartialEq, Digital)]
pub enum Foo {
    Red(b8, bool),
    Green(b8, bool),
}

impl Default for Foo {
    fn default() -> Self {
        Foo::Red(bits(0), false)
    }
}

#[kernel]
fn get_color(a: Signal<Foo, Red>, c: Signal<bool, Red>) -> Signal<bool, Red> {
    signal(
        c.val()
            && match a.val() {
                Foo::Red(_x, z) => z,
                Foo::Green(_x, _z) => true,
            },
    )
}

#[derive(Clone, Debug, Default, Synchronous)]
pub struct Blinky {
    // pub led: b8,
    // pub clk: Signal<ClockReset, Red>,
    // pub led: Signal<b8, Red>,
}

impl SynchronousIO for Blinky {
    type I = ();
    type O = b8;
    type Kernel = blink;
}

impl SynchronousDQ for Blinky {
    type D = ();
    type Q = ();
}

#[kernel]
pub fn blink(cr: ClockReset, _i: (), _q: ()) -> (b8, ()) {
    if cr.reset.any() {
        (b8::default(), ())
    } else {
        (b8::default(), ())
    }
}

pub fn exhaustive<N: BitWidth>() -> Vec<Bits<N>> {
    (0..(1 << N::BITS)).map(bits).collect()
}

fn main() -> miette::Result<()> {
    let simple = Simple {
        a: true,
        b: Bits::from(0b10101010),
        c:  signed(-42),
    };
    let label = trace::svg::format_as_label(&simple.typed_bits()).unwrap();
    println!("{label}");
    
    let uut: Blinky = Blinky::default();
    let hdl = uut.hdl("uut")?;
    std::fs::write("blinky.v", format!("{}", hdl.as_module())).unwrap();
    Ok(())
}
