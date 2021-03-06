//! Default keymap

keymap! {
    CTRL 'C', 'q', ('Q') => Quit;
    Escape => Cancel;
    CTRL 'L', 'r' => Refresh;
    UpArrow, 'k', (CTRL 'K'), (CTRL 'P') => ScrollUpLines(1);
    DownArrow, 'j', (CTRL 'N'), Enter => ScrollDownLines(1);
    SHIFT UpArrow, (ApplicationUpArrow) => ScrollUpScreenFraction(4);
    SHIFT DownArrow, (ApplicationDownArrow) => ScrollDownScreenFraction(4);
    CTRL UpArrow, 'u', CTRL 'U' => ScrollUpScreenFraction(2);
    CTRL DownArrow, 'd', CTRL 'D' => ScrollDownScreenFraction(2);
    PageUp, Backspace, 'b', CTRL 'B' => ScrollUpScreenFraction(1);
    PageDown, ' ', 'f', CTRL 'F' => ScrollDownScreenFraction(1);
    Home, 'g', '<' => ScrollToTop;
    End, 'F', 'G', '>' => ScrollToBottom;
    LeftArrow => ScrollLeftColumns(4);
    RightArrow => ScrollRightColumns(4);
    SHIFT LeftArrow => ScrollLeftScreenFraction(4);
    SHIFT RightArrow => ScrollRightScreenFraction(4);
    '[', SHIFT Tab => PreviousFile;
    ']', Tab => NextFile;
    'h', F 1 => Help;
    '#' => ToggleLineNumbers;
    '\\' => ToggleLineWrapping;
    ':', '%' => PromptGoToLine;
    '/' => PromptSearchForwards;
    '?' => PromptSearchBackwards;
    ',' => PreviousMatch;
    '.' => NextMatch;
    'p', ('N') => PreviousMatchLine;
    'n' => NextMatchLine;
    '(' => FirstMatch;
    ')' => LastMatch;
}
