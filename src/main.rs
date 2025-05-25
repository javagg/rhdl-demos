use rhdl::prelude::*;
use miette;

#[derive(Clone, Debug, Default, Synchronous)]
pub struct U {}

impl SynchronousIO for U {
    type I = b8;
    type O = b16;
    type Kernel = my_kernel;
}

impl SynchronousDQ for U {
    type D = ();
    type Q = ();
}

#[kernel]
fn sub(i: b8) -> b16 {
    i.resize()
}

#[kernel]
pub fn my_kernel(cr: ClockReset, i: b8, _q: ()) -> (b16, ()) {
    if cr.reset.any() {
        (b16::default(), ())
    } else {
        (sub(i), ())
    }
}
// pub struct Counter {
//     pub count: Signal<Bits<4>>,
//     // pub inc: Signal<bool>,
//     // pub dec: Signal<bool>,
// }

// mod comb_adder {
//     use rhdl::prelude::*;

//     #[derive(Clone, Debug, Default, Synchronous)]
//     pub struct U<const N: usize> {}

//     impl<const N: usize> SynchronousIO for U<N> {
//         type I = (Bits<N>, Bits<N>);
//         type O = Bits<N>;
//         type Kernel = adder<{ N }>;
//     }

//     impl<const N: usize> SynchronousDQ for U<N> {
//         type D = ();
//         type Q = ();
//     }

//     #[kernel]
//     pub fn adder<const N: usize>(_cr: ClockReset, i: (Bits<N>, Bits<N>), _q: ()) -> (Bits<N>, ()) {
//         let a = i;
//         (a.0 + a.1, ())
//     }
// }


fn main() -> miette::Result<()> {
    let uut: U = U::default();
    let hdl = uut.hdl("uut")?;
    std::fs::write("counter.v", format!("{}", hdl.as_module())).unwrap();
    Ok(())
}
