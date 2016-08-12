#![allow(unused_must_use)]
macro_rules! write_html {
    ($w:expr, ) => (());
    ($w:expr, $e:tt) => (write!($w, "{}", $e));
    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

pub fn main() {
    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
        html[
            head[title["Macros guide"] hello["world"]]
            body[h1["Macros are the best!"]]
        ]);

    println!("{}", out);
}
