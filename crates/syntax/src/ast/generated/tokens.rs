//! Generated by `cargo xtask codegen grammar`, do not edit by hand.

use crate::{
    ast::AstToken,
    SyntaxKind::{self, *},
    SyntaxToken,
};
use std::{fmt, hash};
pub struct Byte {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Byte {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BYTE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Byte").field("syntax", &self.syntax).finish()
    }
}
impl Clone for Byte {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for Byte {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for Byte {}
impl PartialEq for Byte {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct ByteString {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for ByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for ByteString {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BYTE_STRING }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for ByteString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ByteString").field("syntax", &self.syntax).finish()
    }
}
impl Clone for ByteString {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for ByteString {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for ByteString {}
impl PartialEq for ByteString {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct CString {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for CString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for CString {
    fn can_cast(kind: SyntaxKind) -> bool { kind == C_STRING }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for CString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CString").field("syntax", &self.syntax).finish()
    }
}
impl Clone for CString {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for CString {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for CString {}
impl PartialEq for CString {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct Char {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Char {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CHAR }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Char").field("syntax", &self.syntax).finish()
    }
}
impl Clone for Char {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for Char {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for Char {}
impl PartialEq for Char {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct Comment {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Comment {
    fn can_cast(kind: SyntaxKind) -> bool { kind == COMMENT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Comment").field("syntax", &self.syntax).finish()
    }
}
impl Clone for Comment {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for Comment {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for Comment {}
impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct FloatNumber {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for FloatNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for FloatNumber {
    fn can_cast(kind: SyntaxKind) -> bool { kind == FLOAT_NUMBER }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for FloatNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FloatNumber").field("syntax", &self.syntax).finish()
    }
}
impl Clone for FloatNumber {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for FloatNumber {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for FloatNumber {}
impl PartialEq for FloatNumber {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct Ident {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Ident {
    fn can_cast(kind: SyntaxKind) -> bool { kind == IDENT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ident").field("syntax", &self.syntax).finish()
    }
}
impl Clone for Ident {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for Ident {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for Ident {}
impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct IntNumber {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for IntNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for IntNumber {
    fn can_cast(kind: SyntaxKind) -> bool { kind == INT_NUMBER }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for IntNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntNumber").field("syntax", &self.syntax).finish()
    }
}
impl Clone for IntNumber {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for IntNumber {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for IntNumber {}
impl PartialEq for IntNumber {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct String {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for String {
    fn can_cast(kind: SyntaxKind) -> bool { kind == STRING }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("String").field("syntax", &self.syntax).finish()
    }
}
impl Clone for String {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for String {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for String {}
impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct Whitespace {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Whitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Whitespace {
    fn can_cast(kind: SyntaxKind) -> bool { kind == WHITESPACE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Whitespace").field("syntax", &self.syntax).finish()
    }
}
impl Clone for Whitespace {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for Whitespace {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for Whitespace {}
impl PartialEq for Whitespace {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
