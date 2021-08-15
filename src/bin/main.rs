#[link(name = "tst_so", kind = "dylib", link_args = "-Wl,-rpath,./")]
extern "C" {
    pub fn get_sum(a: u32, b: u32) -> u32;
}

fn main()
{
    let a:u32 = 1;
    let b:u32 = 2;

    unsafe {
        let c = get_sum(a, b);
        println!("{}+{}={}",a,b,c);
    }
}