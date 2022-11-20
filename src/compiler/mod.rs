#![allow(dead_code)]

use std::{fs::File, io::Write};

use crate::parse::{
    ast::{Expr, Statement, StatementList},
    token::TokenType,
};
