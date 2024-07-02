pub mod consts {
    pub const CHUNK_SIZE: usize = 2;

    pub mod colors {
        use termcolor::Color;

        const GRAY: Color = Color::Ansi256(8);
        const LIGHT_GRAY: Color = Color::Ansi256(255); // поменять
        const YELLOW: Color = Color::Yellow;

        pub const HIGHLIGHTED_WORD_BG_COLOR: Color = YELLOW;
        pub const HIGHLIGHTED_LINE_FG_COLOR: Color = LIGHT_GRAY;
        pub const LINE_NUMBER_FG_COLOR: Color = GRAY;
    }
}
