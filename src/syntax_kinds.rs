#![allow(bad_style, missing_docs, unreachable_pub)]
#![cfg_attr(rustfmt, rustfmt_skip)]
//! Generated from grammar.ron
use tree::SyntaxInfo;

/// The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT_DEF`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    USE_KW,
    FN_KW,
    STRUCT_KW,
    ENUM_KW,
    TRAIT_KW,
    IMPL_KW,
    TRUE_KW,
    FALSE_KW,
    AS_KW,
    EXTERN_KW,
    CRATE_KW,
    MOD_KW,
    PUB_KW,
    SELF_KW,
    SUPER_KW,
    IN_KW,
    WHERE_KW,
    FOR_KW,
    LOOP_KW,
    WHILE_KW,
    IF_KW,
    MATCH_KW,
    CONST_KW,
    STATIC_KW,
    MUT_KW,
    UNSAFE_KW,
    ERROR,
    IDENT,
    UNDERSCORE,
    WHITESPACE,
    INT_NUMBER,
    FLOAT_NUMBER,
    SEMI,
    COMMA,
    DOT,
    DOTDOT,
    DOTDOTDOT,
    DOTDOTEQ,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    AT,
    POUND,
    TILDE,
    QUESTION,
    COLON,
    COLONCOLON,
    DOLLAR,
    EQ,
    EQEQ,
    FAT_ARROW,
    NEQ,
    EXCL,
    LIFETIME,
    CHAR,
    BYTE,
    STRING,
    RAW_STRING,
    BYTE_STRING,
    RAW_BYTE_STRING,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    AMPERSAND,
    PIPE,
    THIN_ARROW,
    COMMENT,
    DOC_COMMENT,
    SHEBANG,
    FILE,
    STRUCT_ITEM,
    ENUM_ITEM,
    FN_ITEM,
    EXTERN_CRATE_ITEM,
    MOD_ITEM,
    USE_ITEM,
    STATIC_ITEM,
    CONST_ITEM,
    EXTERN_BLOCK,
    ENUM_VARIANT,
    NAMED_FIELD,
    POS_FIELD,
    ATTR,
    META_ITEM,
    USE_TREE,
    PATH,
    PATH_SEGMENT,
    LITERAL,
    ALIAS,
    VISIBILITY,
    TYPE_PARAM_LIST,
    LIFETIME_PARAM,
    TYPE_PARAM,
    ABI,

    // Technical SyntaxKinds: they appear temporally during parsing,
    // but never end up in the final tree
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
}
pub(crate) use self::SyntaxKind::*;

impl SyntaxKind {
    pub(crate) fn info(self) -> &'static SyntaxInfo {
        match self {
            USE_KW => &SyntaxInfo { name: "USE_KW" },
            FN_KW => &SyntaxInfo { name: "FN_KW" },
            STRUCT_KW => &SyntaxInfo { name: "STRUCT_KW" },
            ENUM_KW => &SyntaxInfo { name: "ENUM_KW" },
            TRAIT_KW => &SyntaxInfo { name: "TRAIT_KW" },
            IMPL_KW => &SyntaxInfo { name: "IMPL_KW" },
            TRUE_KW => &SyntaxInfo { name: "TRUE_KW" },
            FALSE_KW => &SyntaxInfo { name: "FALSE_KW" },
            AS_KW => &SyntaxInfo { name: "AS_KW" },
            EXTERN_KW => &SyntaxInfo { name: "EXTERN_KW" },
            CRATE_KW => &SyntaxInfo { name: "CRATE_KW" },
            MOD_KW => &SyntaxInfo { name: "MOD_KW" },
            PUB_KW => &SyntaxInfo { name: "PUB_KW" },
            SELF_KW => &SyntaxInfo { name: "SELF_KW" },
            SUPER_KW => &SyntaxInfo { name: "SUPER_KW" },
            IN_KW => &SyntaxInfo { name: "IN_KW" },
            WHERE_KW => &SyntaxInfo { name: "WHERE_KW" },
            FOR_KW => &SyntaxInfo { name: "FOR_KW" },
            LOOP_KW => &SyntaxInfo { name: "LOOP_KW" },
            WHILE_KW => &SyntaxInfo { name: "WHILE_KW" },
            IF_KW => &SyntaxInfo { name: "IF_KW" },
            MATCH_KW => &SyntaxInfo { name: "MATCH_KW" },
            CONST_KW => &SyntaxInfo { name: "CONST_KW" },
            STATIC_KW => &SyntaxInfo { name: "STATIC_KW" },
            MUT_KW => &SyntaxInfo { name: "MUT_KW" },
            UNSAFE_KW => &SyntaxInfo { name: "UNSAFE_KW" },
            ERROR => &SyntaxInfo { name: "ERROR" },
            IDENT => &SyntaxInfo { name: "IDENT" },
            UNDERSCORE => &SyntaxInfo { name: "UNDERSCORE" },
            WHITESPACE => &SyntaxInfo { name: "WHITESPACE" },
            INT_NUMBER => &SyntaxInfo { name: "INT_NUMBER" },
            FLOAT_NUMBER => &SyntaxInfo { name: "FLOAT_NUMBER" },
            SEMI => &SyntaxInfo { name: "SEMI" },
            COMMA => &SyntaxInfo { name: "COMMA" },
            DOT => &SyntaxInfo { name: "DOT" },
            DOTDOT => &SyntaxInfo { name: "DOTDOT" },
            DOTDOTDOT => &SyntaxInfo { name: "DOTDOTDOT" },
            DOTDOTEQ => &SyntaxInfo { name: "DOTDOTEQ" },
            L_PAREN => &SyntaxInfo { name: "L_PAREN" },
            R_PAREN => &SyntaxInfo { name: "R_PAREN" },
            L_CURLY => &SyntaxInfo { name: "L_CURLY" },
            R_CURLY => &SyntaxInfo { name: "R_CURLY" },
            L_BRACK => &SyntaxInfo { name: "L_BRACK" },
            R_BRACK => &SyntaxInfo { name: "R_BRACK" },
            L_ANGLE => &SyntaxInfo { name: "L_ANGLE" },
            R_ANGLE => &SyntaxInfo { name: "R_ANGLE" },
            AT => &SyntaxInfo { name: "AT" },
            POUND => &SyntaxInfo { name: "POUND" },
            TILDE => &SyntaxInfo { name: "TILDE" },
            QUESTION => &SyntaxInfo { name: "QUESTION" },
            COLON => &SyntaxInfo { name: "COLON" },
            COLONCOLON => &SyntaxInfo { name: "COLONCOLON" },
            DOLLAR => &SyntaxInfo { name: "DOLLAR" },
            EQ => &SyntaxInfo { name: "EQ" },
            EQEQ => &SyntaxInfo { name: "EQEQ" },
            FAT_ARROW => &SyntaxInfo { name: "FAT_ARROW" },
            NEQ => &SyntaxInfo { name: "NEQ" },
            EXCL => &SyntaxInfo { name: "EXCL" },
            LIFETIME => &SyntaxInfo { name: "LIFETIME" },
            CHAR => &SyntaxInfo { name: "CHAR" },
            BYTE => &SyntaxInfo { name: "BYTE" },
            STRING => &SyntaxInfo { name: "STRING" },
            RAW_STRING => &SyntaxInfo { name: "RAW_STRING" },
            BYTE_STRING => &SyntaxInfo { name: "BYTE_STRING" },
            RAW_BYTE_STRING => &SyntaxInfo { name: "RAW_BYTE_STRING" },
            PLUS => &SyntaxInfo { name: "PLUS" },
            MINUS => &SyntaxInfo { name: "MINUS" },
            STAR => &SyntaxInfo { name: "STAR" },
            SLASH => &SyntaxInfo { name: "SLASH" },
            CARET => &SyntaxInfo { name: "CARET" },
            PERCENT => &SyntaxInfo { name: "PERCENT" },
            AMPERSAND => &SyntaxInfo { name: "AMPERSAND" },
            PIPE => &SyntaxInfo { name: "PIPE" },
            THIN_ARROW => &SyntaxInfo { name: "THIN_ARROW" },
            COMMENT => &SyntaxInfo { name: "COMMENT" },
            DOC_COMMENT => &SyntaxInfo { name: "DOC_COMMENT" },
            SHEBANG => &SyntaxInfo { name: "SHEBANG" },
            FILE => &SyntaxInfo { name: "FILE" },
            STRUCT_ITEM => &SyntaxInfo { name: "STRUCT_ITEM" },
            ENUM_ITEM => &SyntaxInfo { name: "ENUM_ITEM" },
            FN_ITEM => &SyntaxInfo { name: "FN_ITEM" },
            EXTERN_CRATE_ITEM => &SyntaxInfo { name: "EXTERN_CRATE_ITEM" },
            MOD_ITEM => &SyntaxInfo { name: "MOD_ITEM" },
            USE_ITEM => &SyntaxInfo { name: "USE_ITEM" },
            STATIC_ITEM => &SyntaxInfo { name: "STATIC_ITEM" },
            CONST_ITEM => &SyntaxInfo { name: "CONST_ITEM" },
            EXTERN_BLOCK => &SyntaxInfo { name: "EXTERN_BLOCK" },
            ENUM_VARIANT => &SyntaxInfo { name: "ENUM_VARIANT" },
            NAMED_FIELD => &SyntaxInfo { name: "NAMED_FIELD" },
            POS_FIELD => &SyntaxInfo { name: "POS_FIELD" },
            ATTR => &SyntaxInfo { name: "ATTR" },
            META_ITEM => &SyntaxInfo { name: "META_ITEM" },
            USE_TREE => &SyntaxInfo { name: "USE_TREE" },
            PATH => &SyntaxInfo { name: "PATH" },
            PATH_SEGMENT => &SyntaxInfo { name: "PATH_SEGMENT" },
            LITERAL => &SyntaxInfo { name: "LITERAL" },
            ALIAS => &SyntaxInfo { name: "ALIAS" },
            VISIBILITY => &SyntaxInfo { name: "VISIBILITY" },
            TYPE_PARAM_LIST => &SyntaxInfo { name: "TYPE_PARAM_LIST" },
            LIFETIME_PARAM => &SyntaxInfo { name: "LIFETIME_PARAM" },
            TYPE_PARAM => &SyntaxInfo { name: "TYPE_PARAM" },
            ABI => &SyntaxInfo { name: "ABI" },

            TOMBSTONE => &SyntaxInfo { name: "TOMBSTONE" },
            EOF => &SyntaxInfo { name: "EOF" },
        }
    }
}

pub(crate) fn ident_to_keyword(ident: &str) -> Option<SyntaxKind> {
    match ident {
        "use" => Some(USE_KW),
        "fn" => Some(FN_KW),
        "struct" => Some(STRUCT_KW),
        "enum" => Some(ENUM_KW),
        "trait" => Some(TRAIT_KW),
        "impl" => Some(IMPL_KW),
        "true" => Some(TRUE_KW),
        "false" => Some(FALSE_KW),
        "as" => Some(AS_KW),
        "extern" => Some(EXTERN_KW),
        "crate" => Some(CRATE_KW),
        "mod" => Some(MOD_KW),
        "pub" => Some(PUB_KW),
        "self" => Some(SELF_KW),
        "super" => Some(SUPER_KW),
        "in" => Some(IN_KW),
        "where" => Some(WHERE_KW),
        "for" => Some(FOR_KW),
        "loop" => Some(LOOP_KW),
        "while" => Some(WHILE_KW),
        "if" => Some(IF_KW),
        "match" => Some(MATCH_KW),
        "const" => Some(CONST_KW),
        "static" => Some(STATIC_KW),
        "mut" => Some(MUT_KW),
        "unsafe" => Some(UNSAFE_KW),
        _ => None,
    }
}
