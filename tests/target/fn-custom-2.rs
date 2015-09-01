// rustfmt-fn_arg_indent: Inherit
// rustfmt-generics_indent: Tabbed
// rustfmt-where_indent: Inherit
// Test different indents.

fn foo(a: Aaaaaaaaaaaaaaa,
b: Bbbbbbbbbbbbbbbb,
c: Ccccccccccccccccc,
d: Ddddddddddddddddddddddddd,
e: Eeeeeeeeeeeeeeeeeee) {
    foo();
}

fn bar<'a: 'bbbbbbbbbbbbbbbbbbbbbbbbbbb,
    TTTTTTTTTTTTT,
    UUUUUUUUUUUUUUUUUUUU: WWWWWWWWWWWWWWWWWWWWWWWW>
    (a: Aaaaaaaaaaaaaaa) {
    bar();
}

fn baz()
where X: TTTTTTTT
{
    baz();
}

impl Foo {
    fn foo(self,
    a: Aaaaaaaaaaaaaaa,
    b: Bbbbbbbbbbbbbbbb,
    c: Ccccccccccccccccc,
    d: Ddddddddddddddddddddddddd,
    e: Eeeeeeeeeeeeeeeeeee) {
        foo();
    }

    fn bar<'a: 'bbbbbbbbbbbbbbbbbbbbbbbbbbb,
        TTTTTTTTTTTTT,
        UUUUUUUUUUUUUUUUUUUU: WWWWWWWWWWWWWWWWWWWWWWWW>
        (a: Aaaaaaaaaaaaaaa) {
        bar();
    }

    fn baz()
    where X: TTTTTTTT
    {
        baz();
    }
}

struct Foo<TTTTTTTTTTTTTTTTTTTTTTTTTTTT,
    UUUUUUUUUUUUUUUUUUUUUU,
    VVVVVVVVVVVVVVVVVVVVVVVVVVV,
    WWWWWWWWWWWWWWWWWWWWWWWW>
{
    foo: Foo,
}
