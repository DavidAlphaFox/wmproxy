// Copyright 2022 - 2023 Wenmeng See the COPYRIGHT
// file at the top-level directory of this distribution.
// 
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
// 
// Author: tickbh
// -----
// Created Date: 2023/10/11 09:07:03

use webparse::{Buf, BufMut};

use crate::{
    prot::{ProtFlag, ProtKind},
    ProxyResult,
};

use super::{read_short_string, write_short_string, ProtFrameHeader};

/// 进行身份的认证
#[derive(Debug)]
pub struct ProtToken {
    username: String,
    password: String,
}

impl ProtToken {
    pub fn new(username: String, password: String) -> ProtToken {
        Self { username, password }
    }

    pub fn parse<T: Buf>(_header: ProtFrameHeader, mut buf: T) -> ProxyResult<ProtToken> {
        let username = read_short_string(&mut buf)?;
        let password = read_short_string(&mut buf)?;
        Ok(Self { username, password })
    }

    pub fn encode<B: Buf + BufMut>(self, buf: &mut B) -> ProxyResult<usize> {
        let mut head = ProtFrameHeader::new(ProtKind::Token, ProtFlag::zero(), 0);
        head.length = self.username.as_bytes().len() as u32 + 1 + self.password.as_bytes().len() as u32 + 1;
        let mut size = 0;
        size += head.encode(buf)?;
        size += write_short_string(buf, &self.username)?;
        size += write_short_string(buf, &self.password)?;
        Ok(size)
    }

    pub fn username(&self) -> &String {
        &self.username
    }
    
    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn is_check_succ(&self, username: &Option<String>, password: &Option<String>) -> bool {
        if username.is_some() && username.as_ref().unwrap() != &self.username {
            return false;
        }
        if password.is_some() && password.as_ref().unwrap() != &self.password {
            return false;
        }
        return true
    }

    pub fn sock_map(&self) -> u64 {
        0
    }
}
