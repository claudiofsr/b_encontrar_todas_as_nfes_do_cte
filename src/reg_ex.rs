use regex::Regex;
use std::sync::LazyLock as Lazy;

/*
Regex, flags.
All flags are by default disabled unless stated otherwise. They are:

    i     case-insensitive: letters match both upper and lower case
    m     multi-line mode: ^ and $ match begin/end of line
    s     allow . to match \n
    R     enables CRLF mode: when multi-line mode is enabled, \r\n is used
    U     swap the meaning of x* and x*?
    u     Unicode support (enabled by default)
    x     verbose mode, ignores whitespace and allow line comments (starting with `#`)
*/

/*
let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
let hay = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
let mut movies = vec![];
for (_, [title, year]) in re.captures_iter(hay).map(|c| c.extract()) {
    movies.push((title, year.parse::<i64>()?));
}
assert_eq!(movies, vec![
    ("Citizen Kane", 1941),
    ("The Wizard of Oz", 1939),
    ("M", 1931),
]);
*/

pub static REGEX_CTE_PROC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?isx)
        # non-capturing group: (?:regex)
        \s*<\s*
        (:?
            cteProc|
            procEventoCTe
        )
    ",
    )
    .unwrap()
});

pub static REGEX_CHAVE_CTE_COMPL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?isx)

        # <chCTe> | <chCTeCompl> | <chCTeSubcon> | <chCTeRedespInter> | <chCTeMultimodal> | <infCteComp> | ...
        <\w+CTe\w*>

        # non-capturing group: (?:regex)
        (?:<chave>)?

        # capturar 44 dígitos
        (\d{44})

        # non-capturing group: (?:regex)
        (?:</chave>)?

        # <\chCTe> | </chCTeCompl> | </chCTeSubcon> | </chCTeRedespInter> | </chCTeMultimodal> | </infCteComp> | ...
        </\w+CTe\w*>
    ",
    )
    .unwrap()
});

pub static REGEX_CHAVE_NFE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?isx)

        # <infNFe>
        <(\w*NFe\w*)>

        # non-capturing group: (?:regex)
        (?:<chave>)?

        # capturar 44 dígitos
        (\d{44})

        # non-capturing group: (?:regex)
        (?:</chave>)?

        # <infUnidTransp> | <idUnidTransp> | ...
        .*?

        # </infNFe>
        </(\w*NFe\w*)>
    ",
    )
    .unwrap()
});

pub static REGEX_REMOVE_NEWLINES_OR_SPACES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?isx)
        # \n: new_line
        # \s: space

        \n+|\s+
    ",
    )
    .unwrap()
});

pub static REGEX_REMOVE_NON_DIGITS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?isx)
        \D
    ",
    )
    .unwrap()
});

pub static REGEX_CODE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?isx)
        .*
        \d{20}

        # capturar código de 2 dígitos
        (\d{2})

        \d{22}
        .*
    ",
    )
    .unwrap()
});
