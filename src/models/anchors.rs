use crate::anchors::{Comment, Link, Meta, Title};

/// Enum representing all available HTML anchors.
pub enum HTMLAnchor {
    /// Value representing a `<meta>` anchor.
    Meta(Meta),
    /// Value representing a `<title>` anchor.
    Title(Title),
    /// Value representing a `<link>` anchor.
    Link(Link),
    /// Value representing a comment.
    Comment(Comment),
    // Value representing a <a> anchor.
    // A(A),
    // Value representing a <abbr> anchor.
    // Abbr(Abbr),
    // Value representing a <address> anchor.
    // Address(Address),
    // Value representing a <area> anchor.
    // Area(Area),
    // Value representing a <article> anchor.
    // Article(Article),
    // Value representing a <aside> anchor.
    // Audio(Audio),
    // b
    // base
    // bdo
    // bdi
    // blockquote
    // body
    // br
    // button
    // c
    // canvas
    // caption
    // cite
    // code
    // col
    // colgroup
    // command
    // d
    // datalist
    // dd
    // del
    // details
    // dfn
    // div
    // dl
    // dt
    // e
    // em
    // embed
    // f
    // fieldset
    // figcaption
    // figure
    // footer
    // form
    // h
    // h1 à h6
    // head
    // header
    // hr
    // html
    // i
    // i
    // iframe
    // img
    // input
    // ins
    // k
    // keygen
    // kbd
    // l
    // label
    // legend
    // li
    // link
    // m
    // main
    // map
    // mark
    // menu
    // meta
    // meter
    // n
    // nav
    // noscript
    // o
    // object
    // ol
    // optgroup
    // option
    // output
    // p
    // p
    // param
    // pre
    // progress
    // q
    // q
    // r
    // rp
    // rt
    // ruby
    // s
    // s
    // samp
    // script
    // section
    // select
    // small
    // source
    // span
    // strong
    // style
    // sub
    // summary
    // sup
    // t
    // table
    // tbody
    // td
    // textarea
    // tfoot
    // th
    // thead
    // time
    // title
    // tr
    // track
    // ul
    // v
    // var
    // video
    // w
    // wbr
}
