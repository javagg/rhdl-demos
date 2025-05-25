use miette;
use rhdl::prelude::*;

type nibble = Bits<b4>;

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

mod suba {
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct I {
        pub enable: bool,
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct I {
    pub clock_reset: Signal<ClockReset, Red>,
    // pub enable: Signal<suba::I, Red>,
    // pub enable: Signal<crate::counter::I, Red>,
}

mod aaauu {
    // use rhdl::prelude::Digital;
    use rhdl::{core::Digital, prelude::Timed};

    #[derive(Debug, Clone, Copy, PartialEq, Timed)]
    pub struct ZDriver {
        // pub bus: BitZ<N>,
        // pub mask: Bits<N>,
        // pub data: Bits<N>,
    }
    
    impl Digital for ZDriver {
        const BITS: usize = 8;
    
        fn static_kind() -> rhdl::prelude::Kind {
            todo!()
        }
    
        fn static_trace_type() -> rhdl::rtt::TraceType {
            todo!()
        }
    
        fn bin(self) -> Vec<rhdl::prelude::BitX> {
            todo!()
        }
    
        fn dont_care() -> Self {
            todo!()
        }
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

    // let dff = DFF::<u8>::default();
    let uut: U = U::default();
    let hdl = uut.hdl("uut")?;
    std::fs::write("counter.v", format!("{}", hdl.as_module())).unwrap();

    let fg = &uut.descriptor("uut")?.flow_graph;
    let mut file = std::fs::File::create("inverter.dot").unwrap();
    write_dot(fg, &mut file).unwrap();

    // let test_bench = uut.run(inputs)?.collect::<TestBench<_, _>>();
    // let tm_rtl = test_bench.rtl(uut, &TestBenchOptions::default())?;
    // tm_rtl.run_iverilog()?;
    // simple_traced_synchronous_run(&uut, stream, "auto_double.vcd");
    Ok(())
}
