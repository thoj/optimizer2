// RUST experiments
// Copyright 2016 Thomas Jäger

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
	    fn mix(&self, m : &$name, w1 : f32, w2 : f32) -> $name {
		let mut mixed = $name::new();
		$(
			mixed.$field_name = ( ( (self.$field_name as f32 * w1) + (m.$field_name as f32 * w2) ) / (w1 + w2) ).ceil() as u32;
		)*
		mixed
	    }
	    fn check(&self, s : &$name) -> CheckResult {
		$(
			if self.$field_name > s.$field_name { 
				return CheckResult::Failed { element: &stringify!($field_name)[1..], spec: s.$field_name, value: self.$field_name };
			}
		)*
		CheckResult::Pass
	    }
	    fn set(&mut self, s : &'static str, i : u32) {
		$(
			if &stringify!($field_name)[1..] == s {
				self.$field_name = i;
			}
		)*
	    }
	    fn get(&self, s : &'static str) -> Option<u32> {
		$(
			if &stringify!($field_name)[1..] == s {
				return Some(self.$field_name);
			}
		)*
		None
	    }
        }
    }
}



#[derive(Debug)]
#[derive(PartialEq)]
enum CheckResult {
	Failed { element: &'static str, spec : u32, value: u32 },
	Pass,
}


struct Sp {
	priority: u32,
	elements: Elements,
}

struct Fu {
	number: u32,
	weight: u32,
	elements: Elements,
}

generate_elm_struct! {
 struct Elements {
	_si: u32,
	_fe: u32,
	_cu: u32,
	_mg: u32,
	_ag: u32,
	_as: u32,
	_b: u32,
	_ba: u32,
	_be: u32,
	_bi: u32,
	_ca: u32,
	_cd: u32,
	_ce: u32,
	_cl: u32,
	_co: u32,
	_cr: u32,
	_cs: u32,
	_f: u32,
	_ga: u32,
	_ge: u32,
	_in: u32,
	_k: u32,
	_la: u32,
	_li: u32,
	_mn: u32,
	_mo: u32,
	_na: u32,
	_nd: u32,
	_ni: u32,
	_o: u32,
	_p: u32,
	_pb: u32,
	_pd: u32,
	_pr: u32,
	_pt: u32,
	_s: u32,
	_sb: u32,
	_sn: u32,
	_sr: u32,
	_ti: u32,
	_th: u32,
	_u: u32,
	_v: u32,
	_w: u32,
	_zn: u32,
	_zr: u32,
	_y: u32,
 }
}

#[test]
fn get_set_test() {
	let mut elms = Elements::new();
	elms.set("y", 99);
	assert_eq!(elms.get("y"), Some(99))
}

#[test]
fn mix_test() {
	let mut elms1 = Elements::new();
	let mut elms2 = Elements::new();
	elms1.set("si", 50);
	elms2.set("si", 100);
	elms1.set("cu", 61);
	elms2.set("cu", 84);
	let elms2 = elms2; // immutable
	let mut elms1 = elms1.mix(&elms2, 950.0, 950.0);
	assert_eq!(elms1._si, 75);	
	assert_eq!(elms1._cu, 73); // always round up	
	//reset
	elms1.set("si", 50);
	elms1.set("cu", 61);
	let elms1 = elms1.mix(&elms2, 1000.0, 500.0);
	assert_eq!(elms1._si, 67);	
	assert_eq!(elms1._cu, 69); // always round up	
}

#[test]
fn check_test() {
	let mut elms1 = Elements::new();
	let mut s1 = Elements::new();
	elms1._si = 9;
	elms1._cu = 11;
	elms1._mg = 100;
	s1._si = 10;
	s1._cu = 10;
	s1._mg = 100;
	assert_eq!(elms1.check(&s1), CheckResult::Failed { element: "cu", spec: 10, value: 11 });
	elms1._cu = 9;
	assert_eq!(elms1.check(&s1), CheckResult::Pass );
	elms1._mg = 101;
	assert_eq!(elms1.check(&s1), CheckResult::Failed { element: "mg", spec: 100, value: 101});
}

fn main() {

    let mut elms = Elements::new();
    let mut elms2 = Elements::new();

    elms._si = 6;
    elms._as = 6;
    elms.set("sn", 10);
    let foo = elms.mix(&elms2, 950.0, 950.0 );
    elms.printall();
    elms2.printall();
    foo.printall();
//    println!("{}", foo.check(&elms));

}
