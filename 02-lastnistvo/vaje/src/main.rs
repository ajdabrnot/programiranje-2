use std::time::{Duration, Instant};

fn time_it<F: FnOnce() -> R, R>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn on_stack() {
    // Narišite shemo spreminjanja sklada in kopice
    // Za vsako vrstico napiši, kolikokrat se v pomnilniku pojavi 13?
    let mut a = [13; 100]; // 100 trinajstk v pomnilniku
    let mut b = a; //se prekopira, ne kaže istega seznama kot a, ker je samo en loh lastnik česarkoli -> imamo zdj 200 trinajstk v pomnilniku
    let q = Box::new(13); //še ena dodatna trinajstka
                          //let q = String::from("13");
    println!("{}", q); //ne rabiš pisat zvezdice, ki bi rekla, poglej v škatlo, ker rust sam ve da rab tja gledat? (let r 0 *q; mi ubistvu reče r je odpakiran q??)
    let r = q; // q zgubi lastništvo in zdj r kaže na to škatlo, nismo dodali nobene 13
    let p = &r; //ta & pravi da si sam sposodm vrednost od r, lastnik te vrednosti je še kr r aka če spreminjam p se spremeni tut r? in se ne ustvari nobena 13
    let y = **p; //prva zvezdica da ni več sposojen, druga zvezdica da ni več box
    a[0] = 1; //prvi elt arraya je zdj 1 aka ena 13 manj, pomembno: spremeni se a ne pa tudi b ker je b kopija od a
    {
        let c = &b; //ta c je tuki lokalna spremenljivka
        println!("{}", c[0]);
    }
    println!("{}", b[0]); //sprinta 13
    println!("{}", a[0]); //sprinta 1
    println!("{}", p); //sprinta 13
    println!("{}", r); //sprinta 13
                       // println!("{}", q); // Razloži, zakaj to ne deluje -> ker je r vzel lastništvo nad q in je q zdj brez vsega :(
}

/// Napišite funkcijo `swap`, ki zamenja vrednosti dveh celoštevilskih spremenljivk.
///
///

fn swap(x: &mut i32, y: &mut i32) {
    let c = *x;
    *x = *y;
    *y = c;
}

fn test_swap() {
    // V spremenljivko `a` shranite vrednost 13, v spremenljivko `b` pa vrednost 42.
    let mut a = 13;
    let mut b = 42;
    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 13, b: 42`.

    // Naredite swap s pomočjo pomožne funkcije `swap`.
    swap(&mut a, &mut b);
    //

    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 42, b: 13`.
}

/// Popravite zakomentiran del spodnje funkcije, da bo deloval
fn str_own() {
    // let x = String::from("Hello world");//string se ustvari na kopici keer ga loh spreminjamo in ne vemo kok prostora rabmo
    // let y = x; //y vzame lastništvo nad x in ydj x ne obstaja več
    // println!("{}, {}", x, y);// in tle smo ga probal izpisat čerov ne obstaja
}

fn str_own1() {
    let x = String::from("Hello world");
    let y = x.clone(); // se nrdi še en string na kopici in y kaže tja
    println!("{}, {}", x, y);
}

/// Popravite brez uporabe funkcije `clone`
/// Namig: sklad in kopiranje na skladu - kodo lahko spremenite
fn str_own2() {
    let x = (1, 2, (), String::from("Hello world")); //tuple je lastnik stringa in x je lastnik tupla
    let y = &x;
    println!("{:?}, {:?}", x, y);
}

fn str_own22() {
    let x = (1, 2, (), "Hello world");
    let y = x; //to je zdj okj ker vsi elti tupla živioj na stacku in torej se vse skopira v y in ne le premakne!!
    println!("{:?}, {:?}", x, y);
}

/// Popravite spodnji dve funkciji, da bosta delovali

fn wrong() {
    let s = String::from("Hello World");
    print_str(&s);
    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s)
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala
fn fn1() {
    let s = String::from("Hello ");

    // let s1 = s; ni mutable in pol ne dela :(
    let mut s1 = s;
    s1.push_str("World!");

    println!("Success! {}", s1);
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala

fn fn2() {
    let x = Box::new(5);

    // // Popravite zgolj tukaj vmes
    let mut y = Box::new(42);
    // //
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------

fn fn3() {
    let t = (
        String::from("hello"),
        String::from("world"),
        String::from("!"),
    );

    let _s = t.1;

    // Izpišite čim večji del t-ja.
    println!("{}{}{}", t.0, _s, t.2);
}

/// ------------------------------------------------------------------------------------------------

fn fn4() {
    let x = 5;
    // Izpišite naslov spremenljivke x as in v pomnilniku
    println!("{:p}", &x); //as in pointer
}

/// ------------------------------------------------------------------------------------------------

fn fn5() {
    let x = 13;
    let y = &x;

    // Popravite spodnjo vrstico, da bo bo enakost držala
    // assert_eq!(13, y);
    assert_eq!(13, *y);
}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper` se mora poklicati čim bolj učinkovito.
fn fn6() {
    let mut s = String::from("hello, ");

    helper(&s);

    println!("Success!");
}

// Te funkcije ne spreminjajte
fn helper(s: &String) {}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper2` se mora poklicati čim bolj učinkovito.
fn fn7() {
    let mut s = String::from("hello, ");

    helper2(&mut s);

    println!("Success!");
}
// Te funkcije ne spreminjajte
fn helper2(s: &mut String) {
    s.push_str("world")
}

/// ------------------------------------------------------------------------------------------------

/// Pojasnite, zakaj spodnja koda ne deluje -> ker p ni mut
fn fn8() {
    let mut s = String::from("hello, ");

    // let p = &mut s;
    {
        let p = &mut s;

        p.push_str("world");
        println!("Success! {}", p);
    }
    println!("Success! {}", s); //problem je da s sposodmo kot immutable, ampak ne morš si ga pa mutably sposodt več kot enkrat
    s.push_str("!");
}

/// ------------------------------------------------------------------------------------------------
/// Pojasnite, zakaj spodnja koda ne deluje in jo popravite
/// Pojasnite tudi zakaj je popravek ok

//fn fn9() {
//    let mut s = String::from("hello");
//    let r1 = &mut s;
//    let r2 = &mut s;
//    println!("{}, {}", r1, r2);
//    println!("Success!");
//}
//
//fn fn99() {
//    let mut s = String::from("hello");
//    {
//        let r1 = &mut s;
//        print!(nevemmmmmmmm)
//    }
//    let r2 = &mut s;
//    println!("{}, {}", r1, r2);
//    println!("Success!");
//}

/// ------------------------------------------------------------------------------------------------
fn fn10() {
    // // Popravite spodnjo vrstico
    // let s = String::from("hello, ");

    // helper3(&mut s);

    // println!("Success!");
}

fn helper3(s: &mut String) {}

/// ------------------------------------------------------------------------------------------------

fn main() {}
