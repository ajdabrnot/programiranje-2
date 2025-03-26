#[derive(Debug, PartialEq)]
struct AritmeticnoZaporedje<T> {
    a0: T,
    ai: T,
    d: T,
}

use core::str;
use std::fmt::format;

use AritmeticnoZaporedje as AZ;

impl<
        T: Copy
            + Clone
            + std::ops::AddAssign
            + std::ops::Mul<Output = T>
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>,
    > AZ<T>
{
    fn new(zacetnni: T, razlika: T) -> AZ<T> {
        return AZ {
            a0: zacetnni,
            ai: zacetnni,
            d: razlika,
        };
    }

    fn next(zaporedje: &mut AZ<T>) -> T {
        zaporedje.ai += zaporedje.d;
        return zaporedje.ai - zaporedje.d;
    }

    fn n_th(&mut self, n: i128) -> T {
        let mut n_ti_clen = self.a0;
        for _ in 0..n {
            n_ti_clen += self.d;
        }
        return n_ti_clen;
    }

    fn reset(zaporedje: &mut AZ<T>) -> () {
        zaporedje.ai = zaporedje.a0;
    }

    fn current(zaporedje: AZ<T>) -> T {
        return zaporedje.ai;
    }

    fn sum(&mut self, n: i128) -> T {
        let mut vsota = self.a0;
        for i in 1..n {
            vsota += self.n_th(i);
        }
        return vsota;
    }

    fn vsota(zap1: AZ<T>, zap2: AZ<T>) -> AZ<T> {
        return AZ {
            a0: zap1.a0 + zap2.a0,
            ai: zap1.ai + zap2.ai,
            d: zap1.d + zap2.d,
        };
    }

    fn mnozi(&mut self, other: &mut Self) -> AZ<T> {
        return AZ {
            a0: self.a0 * other.a0,
            ai: self.a0 * other.a0,
            d: self.d * other.d,
        };
    }
}

//impl<T> PartialEq for AZ<T>{
//    fn eq(&self, other: &Self) -> bool {
//        self.a0 < other.a0 & self.d < other.d
//    }
//}

trait Sequence<T> {
    fn name(&self) -> String;
    fn k_th(&self, k: usize) -> Option<T>;
    fn contains(&self, item: T) -> bool;
    fn start(&self) -> T;
}

pub struct Constanta<T> {
    //pub struct je fino za knjižnice če želiš importat al neki
    c: T,
}

pub struct Contatnt_int {
    c: i64,
}

impl<T> Constanta<T> {
    fn new(c: T) -> Constanta<T> {
        Constanta { c }
    }
}

impl Contatnt_int {
    fn new(c: i64) -> Contatnt_int {
        Contatnt_int { c }
    }
}

impl<T: std::cmp::PartialEq + Copy + Clone> Sequence<T> for Constanta<T> {
    fn name(&self) -> String {
        format!("Constanta")
    }
    fn k_th(&self, _k: usize) -> Option<T> {
        return Some(self.c);
    }
    fn contains(&self, item: T) -> bool {
        return self.c == item;
    }
    fn start(&self) -> T {
        return self.c;
    }
}

//impl Sequence<i64> for Constanta<i64> {
//    fn name(&self) -> String {
//        format!("Constanta")
//    }
//    fn k_th(&self, _k: usize) -> Option<i64> {
//        return Some(self.c);
//    }
//    fn contains(&self, item: i64) -> bool {
//        return self.c == item;
//    }
//    fn start(&self) -> i64 {
//        return self.c;
//    }
//}

fn main() {
    println!("Are you my mommy?");
    println!("ne da se mi")
}
