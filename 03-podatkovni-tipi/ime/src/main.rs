#[derive(Debug)]
struct AritmeticnoZaporedje {
    a0: i128,
    ai: i128,
    d: i128,
}

// kaj zaene zadeve pa želim, da jih počne:
// da mi izračuna n-ti člen
// da mi izračuna naslednji člen
// da mi izračuna vsoto n členov

use AritmeticnoZaporedje as AZ;

impl AZ {
    fn new(zacetnni: i128, razlika: i128) -> AZ {
        return AZ {
            a0: zacetnni,
            ai: zacetnni,
            d: razlika,
        };
    }

    fn next(zaporedje: &mut AZ) -> i128 {
        zaporedje.ai += zaporedje.d;
        return zaporedje.ai - zaporedje.d;
    }

    fn n_th(n: i128, zaporedje: AZ) -> i128 {
        return zaporedje.a0 + n * zaporedje.d;
    }

    fn reset(zaporedje: &mut AZ) -> () {
        zaporedje.ai = zaporedje.a0;
    }

    fn current(zaporedje: AZ) -> i128 {
        return zaporedje.ai;
    }

    fn sum(n: i128, zaporedje: AZ) -> i128 {
        let mut vsota = 0;
        for i in 0..n {
            vsota += zaporedje.a0 + i * zaporedje.d;
        }
        return vsota;
    }

    fn vsota(zap1: AZ, zap2: AZ) -> AZ {
        return AZ {
            a0: zap1.a0 + zap2.a0,
            ai: zap1.ai + zap2.ai,
            d: zap1.d + zap2.d,
        };
    }

    //fn produkt(zap1: AZ, zap2: AZ) -> AZ {
    //    //ne da se. fuj
    //}
}

#[derive(Debug)]
struct GeometrijskoZaporedje {
    a0: i128,
    ai: i128,
    q: i128,
}

use GeometrijskoZaporedje as GZ;

impl GZ {
    fn new(zacetnni: i128, razlika: i128) -> GZ {
        return GZ {
            a0: zacetnni,
            ai: zacetnni,
            q: razlika,
        };
    }

    fn next(zaporedje: &mut GZ) -> i128 {
        zaporedje.ai *= zaporedje.q;
        return zaporedje.ai / zaporedje.q;
    }

    fn n_th(n: u32, zaporedje: &mut GZ) -> i128 {
        let zmnozen_q = i128::pow(zaporedje.q, n);
        return zaporedje.a0 * zmnozen_q;
    }

    fn reset(zaporedje: &mut GZ) -> () {
        zaporedje.ai = zaporedje.a0;
    }

    fn current(zaporedje: GZ) -> i128 {
        return zaporedje.ai;
    }

    fn sum(n: i128, zaporedje: &mut GZ) -> i128 {
        let mut vsota = 0;
        for i in 0..n {
            let j = i.try_into().unwrap();
            let i_ti_clen = GZ::n_th(j, zaporedje);
            vsota += i_ti_clen;
        }
        return vsota;
    }

    //fn vsota(zap1: GZ, zap2: GZ) -> GZ {
    //    //ne
    //}

    fn produkt(zap1: GZ, zap2: GZ) -> GZ {
        return GZ {
            a0: zap1.a0 * zap2.a0,
            ai: zap1.ai * zap2.ai,
            q: zap1.q * zap2.q,
        };
    }
}

enum BinOperacija {
    Plus,
    Minus,
    Times,
    Power,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

//brez box ne dela ker box rezrvira kok prostora rabiš na kopici
//ne rabiš oklepajev, ker vsak izreaz ki ma operacijo je svoje drevesce in najprj se od spodi gor zračunajo drevesca in pol na konc veliko drevo kjut;

impl Izraz {
    fn eval(&self) -> u32 {
        use crate::BinOperacija::Minus;
        use crate::BinOperacija::Plus;
        use crate::BinOperacija::Power;
        use crate::BinOperacija::Times;
        use crate::Izraz::Konstanta;
        use crate::Izraz::Operacija;

        fn aux(izraz: &Izraz) -> u32 {
            match izraz {
                Konstanta(x) => *x,
                Operacija(x, y, z) => match y {
                    Plus => aux(&x) + aux(&z),
                    Minus => aux(&x) - aux(&z),
                    Times => aux(&x) * aux(&z),
                    Power => aux(&x).pow(aux(&z)),
                },
            }
        }
        aux(&self)
    }

    fn collect(&self) -> u32 {
        match &self {
            Izraz::Konstanta(x) => 1,
            Izraz::Operacija(x, _y, z) => x.collect() + z.collect(),
        }
    }

    fn izpis(&self) -> String {
        match &self {
            Izraz::Operacija(x, BinOperacija::Minus, z) => {
                String::from("(")
                    + &x.izpis()
                    + &String::from("-")
                    + &z.izpis()
                    + &String::from(")")
            }

            Izraz::Operacija(x, BinOperacija::Plus, z) => {
                String::from("(")
                    + &x.izpis()
                    + &String::from("+")
                    + &z.izpis()
                    + &String::from(")")
            }

            Izraz::Operacija(x, BinOperacija::Times, z) => {
                String::from("(")
                    + &x.izpis()
                    + &String::from("*")
                    + &z.izpis()
                    + &String::from(")")
            }

            Izraz::Operacija(x, BinOperacija::Power, z) => {
                String::from("(")
                    + &x.izpis()
                    + &String::from("**")
                    + &z.izpis()
                    + &String::from(")")
            }
            Izraz::Konstanta(x) => x.to_string(),
        }
    }
}

fn main() {
    let c: Izraz = Izraz::Konstanta(2);
    let izraz1: Izraz = Izraz::Operacija(
        Box::new(Izraz::Konstanta(1)),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(2)),
            BinOperacija::Times,
            Box::new(Izraz::Konstanta(3)),
        )),
    );

    let izraz3: Izraz = Izraz::Operacija(
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(1)),
            BinOperacija::Plus,
            Box::new(Izraz::Konstanta(2)),
        )),
        BinOperacija::Plus,
        Box::new(Izraz::Konstanta(3)),
    );

    let izraz4: Izraz = Izraz::Operacija(
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(5)),
            BinOperacija::Power,
            Box::new(Izraz::Konstanta(2)),
        )),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(3)),
            BinOperacija::Power,
            Box::new(Izraz::Konstanta(2)),
        )),
    );

    println!("{}", Izraz::eval(&izraz3));
    println!("{}", Izraz::eval(&izraz4));
    println!("{}", Izraz::izpis(&izraz1));
}
