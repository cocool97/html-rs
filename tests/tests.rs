#[cfg(test)]
mod tests {
    use html_rs::anchors::Comment;
    use html_rs::anchors::Meta;
    use html_rs::anchors::Title;
    use html_rs::HTMLHead;
    use html_rs::HTMLHeadAnchor;
    use html_rs::HTMLDOM;

    #[test]
    fn test_build() {
        use std::fs::File;

        let mut dom = HTMLDOM::new();
        dom.set_lang("fr-FR");

        let mut head = HTMLHead::new();
        head.add_anchor(HTMLHeadAnchor::Meta(Meta::new(Some("SS"))));
        head.add_anchor(HTMLHeadAnchor::Title(Title::new("Great website")));
        head.add_anchor(HTMLHeadAnchor::Comment(Comment::new(
            "This should be a comment",
        )));

        dom.set_head(head);

        let writer = File::create("/tmp/eifjbzb.html").unwrap();

        dom.build_dom(writer).unwrap();
    }
}
