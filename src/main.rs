// RUST experiments
// Copyright 2016 Thomas Jäger


extern crate rand;

//use std::io;
//use rand::Rng;

// Yeah like i said EXPERIMENTS
// This is a terrible idea. Sorry!

macro_rules! generate_elm_struct {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
	    fn new() -> $name {
		$name { $( $field_name: 0, )* }
	    }
	    fn printall(&self) {
		$(
			print!("{0}={1}, ", stringify!($field_name), self.$field_name);
		)*
		println!("")
	    }
	    fn mix(&self, m : &$name) -> $name {
		let mut mixed = $name::new();
		$(
			mixed.$field_name = ((self.$field_name as f32 + m.$field_name as f32) / 2.0).ceil() as u32;
		)*
		mixed
	    }
	    fn check(&self, s : &$name) -> bool {
		$(
			if self.$field_name > s.$field_name { 
				return false;
			}
		)*
		true
	    }
	    fn set(&mut self, s : &'static str, i : u32) {
		$(
			if stringify!($field_name) == s {
				self.$field_name = i;
			}
		)*
	    }
	    fn get(&self, s : &'static str) -> Option<u32> {
		$(
			if stringify!($field_name) == s {
				return Some(self.$field_name);
			}
		)*
		None
	    }
        }
    }
}



generate_elm_struct! {
 struct elements {
	si: u32,
	fe: u32,
	cu: u32,
	mg: u32,
	ag: u32,
	_as: u32,
	b: u32,
	ba: u32,
	be: u32,
	bi: u32,
	ca: u32,
	cd: u32,
	ce: u32,
	cl: u32,
	co: u32,
	cr: u32,
	cs: u32,
	f: u32,
	ga: u32,
	ge: u32,
	_in: u32,
	k: u32,
	la: u32,
	li: u32,
	mn: u32,
	mo: u32,
	na: u32,
	nd: u32,
	ni: u32,
	o: u32,
	p: u32,
	pb: u32,
	pd: u32,
	pr: u32,
	pt: u32,
	s: u32,
	sb: u32,
	sn: u32,
	sr: u32,
	ti: u32,
	th: u32,
	u: u32,
	v: u32,
	w: u32,
	zn: u32,
	zr: u32,
	y: u32,
 }
}

#[test]
fn get_set_test() {
	let mut elms = elements::new();
	elms.set("y", 99);
	assert_eq!(elms.get("y"), Some(99))
}

#[test]
fn mix_test() {
	let mut elms1 = elements::new();
	let mut elms2 = elements::new();
	elms1.set("si", 50);
	elms2.set("si", 100);
	elms1.set("cu", 61);
	elms2.set("cu", 84);
	let elms1 = elms1.mix(&elms2);
	assert_eq!(elms1.si, 75);	
	assert_eq!(elms1.cu, 73); // always round up	
}

#[test]
fn check_test() {
	let mut elms1 = elements::new();
	let mut s1 = elements::new();
	elms1.si = 9;
	elms1.cu = 11;
	elms1.mg = 100;
	s1.si = 10;
	s1.cu = 10;
	s1.mg = 100;
	assert!(!elms1.check(&s1));
	elms1.cu = 9;
	assert!(elms1.check(&s1));
	elms1.mg = 101;
	assert!(!elms1.check(&s1));
}

fn main() {

    let mut elms = elements::new();
    let mut elms2 = elements::new();

    elms.si = 6;
    elms._as = 6;
    elms.set("sn", 10);
    let foo = elms.mix(&elms2);
    elms.printall();
    elms2.printall();
    foo.printall();
    println!("{}", foo.check(&elms));

}
