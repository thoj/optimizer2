
// RUST experiments
// Copyright 2016 Thomas Jäger

// Yeah like i said EXPERIMENTS
// This is a terrible idea. Sorry!

extern crate mysql;

use std::cmp;
use mysql as my;

macro_rules! generate_elm_struct {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
	#[derive(Debug)]
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
			mixed.$field_name = ( ( (self.$field_name as f32 * w1) + (m.$field_name as f32 * w2) ) / (w1 + w2) ).ceil() as i64;
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
	    fn set(&mut self, s : &str, i : i64) {
		$(
			if &stringify!($field_name)[1..] == s {
				self.$field_name = i;
			}
		)*
	    }
	    fn get(&self, s : &str) -> Option<i64> {
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
	Failed { element: &'static str, spec : i64, value: i64 },
	Pass,
}


#[derive(Debug)]
struct Sp {
	name: String,
	priority: i64,
	elements: Elements,
}

#[derive(Debug)]
struct Fu {
	number: u32,
	weight: u32,
	elements: Elements,
}

generate_elm_struct! {
 struct Elements {
	_si: i64,
	_fe: i64,
	_cu: i64,
	_mg: i64,
	_ag: i64,
	_as: i64,
	_b: i64,
	_ba: i64,
	_be: i64,
	_bi: i64,
	_ca: i64,
	_cd: i64,
	_ce: i64,
	_cl: i64,
	_co: i64,
	_cr: i64,
	_cs: i64,
	_f: i64,
	_ga: i64,
	_ge: i64,
	_in: i64,
	_k: i64,
	_la: i64,
	_li: i64,
	_mn: i64,
	_mo: i64,
	_na: i64,
	_nd: i64,
	_ni: i64,
	_o: i64,
	_p: i64,
	_pb: i64,
	_pd: i64,
	_pr: i64,
	_pt: i64,
	_s: i64,
	_sb: i64,
	_sn: i64,
	_sr: i64,
	_ti: i64,
	_th: i64,
	_u: i64,
	_v: i64,
	_w: i64,
	_zn: i64,
	_zr: i64,
	_y: i64,
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


fn rotate_arr(t: &mut Vec<u32>) {
	let v = t[1];
	let l = t.len(); //Borrow???
	for x in 1..(l-1)  {
		t[x] = t[x + 1];
	}
	t[l-1] = v;
}

fn rr_shed_pair(t: &Vec<u32>) -> Vec<(u32,u32)> {
	let mut p : Vec<(u32,u32)> = vec![];
	for x in 0..t.len()/2 {
		p.push((t[x], t[t.len()/2+x]));
	}
	p
}

fn next(s: &mut Vec<u32>, m: &mut Vec<u32>, n: usize) -> bool {
	let mut i = 0;
	s[i]+=1;
	while (i < (n - 1) as usize) && (s[i] > m[i] + 1) {
		s[i] = 1;
		i+=1;
		s[i]+=1;
	}
	if i == (n - 1) as usize { 
		return false;
	}
	let max = cmp::max(s[i], m[i]);
	if i > 0 {
		i-=1;
		while i >= 0 { 
			m[i] = max;
			if i > 0 {
				i-=1;
			}
			else {
				break;
			}
		}
	}
	return true;
}

//#[test]
fn test_rr_shed() {
//	let mut t : Vec<u32> = vec![1,2,3,4,5,6,7,8];
//	for x in 0..t.len()-1 {
//		let p = rr_shed_pair(&t);
//		println!("{:?}", p);
//		rotate_arr(&mut t);
//	}
//	let mut t : Vec<u32> = vec![1,2,3,4,5,6];
//	for x in 0..t.len() {
//		let p = rr_shed_triple(&t);
//		println!("{:?}", p);
//		rotate_arr(&mut t);
//	}
	let n = 12;
	let mut s: Vec<u32> = vec![1; n];
	let mut m: Vec<u32> = vec![1; n];
	let mut c = 1;
	while next(&mut s, &mut m, n) {
		let mut part_num: u32 = 1;
		let mut i 	: usize = 0;
		while i < n {
			if s[i] > part_num {
				part_num = s[i];
			}
			i+=1;
		}
		print!("{}.",c);
		let mut p = part_num;
		while p >= 1 {
			//print!("[");
			i = 0;
			while i < n as usize {
				if s[i] == p {
			//		print!("{},", i + 1);
				}
				
				i+=1;
			}
			//print!("]");
			p-=1;
		}
		c+=1;
		println!("");
	}	

}

fn subset(arr: &Vec<usize>, size: usize, left: usize, index: usize, list: &mut Vec<usize>) {
	if left == 0 {
		println!("{:?}", list);
		return;
	}
	for i in index..size-1 {
		list.push(arr[i]);
		subset(arr, size, left-1, i+1,list);
		let _ = list.pop();
	}
}

fn subset_all() {
	let mut list : Vec<usize> = vec![];
	let array = vec![1,2,3,4,5];
	subset(&array, 5, 3, 0, &mut list);
	println!("{:?}", list)
}

fn load_quality(pool: my::Pool) -> Vec<Sp> {
	let res = pool.prep_exec("SELECT * FROM quality ORDER BY priority ASC", ()).unwrap();
	let mut Sps: Vec<Sp> = vec![]; // hold data
	let idxs = res.column_indexes();
	let mut res = res;
	for row in res {

		let mut e = Elements::new();
		let mut r = row.unwrap();
		let name : String = r.take("name").expect("Name");
		let priority : i64 = r.take("priority").expect("Priority");
		for (x, y) in idxs.clone() {
			let v : Option<my::Value> = r.get(&x[..]);
			if let Some(vv) = v {
				if let my::Value::Int(ii) = vv {
					e.set(&x[..], ii);
				}
			}
		}	
		Sps.push(Sp {elements: e, name: name, priority: priority} );
	}
	Sps
}
//FIXME
fn load_data(pool: my:Pool) -> Vec<Fu> {
	//let res = pool.prep_exec("SELECT * FROM ovn WHERE date='2016-05-06' ORDER BY quality ASC", ()).unwrap();
}

fn main() {
	let pool = my::Pool::new("mysql://vmr:vmr@localhost:3306/vmr").unwrap();
	let Sps = load_quality(pool);
	println!("Loaded {} qualities", Sps.len());
}
