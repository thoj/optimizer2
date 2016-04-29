extern crate rand;

//use std::io;
//use rand::Rng;

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
	    fn mix(&self, m : $name) -> $name {
		let mixed = $name::new();
		$(
			mixed.$field_name = (self.$field.name + m.$field_name)
		)
	    }
            fn get_field_names(&self) -> Vec<&'static str> {
                vec![$(stringify!($field_name)),*]
            }
        }
    }
}

generate_elm_struct! {
 struct ElementValues {
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

fn main() {

    let mut elms = ElementValues::new();

    println!("{:?}", elms.get_field_names());
    elms.si = 6;
    elms._as = 6;
    println!("si: {}", elms.si);
    elms.printall();
}