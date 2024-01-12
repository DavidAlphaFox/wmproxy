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
// Created Date: 2023/10/20 10:19:47



use std::{net::SocketAddr, time::Duration};

use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use serde_with::DurationSeconds;


use crate::{HealthCheck};


fn default_weight() -> u16 {
    100
}

fn fail_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_fall_times() -> usize {
    2
}

fn default_rise_times() -> usize {
    2
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleStreamConfig {
    /// 访问地址
    pub addr: SocketAddr,
    /// 权重
    #[serde(default = "default_weight")]
    pub weight: u16,
    /// 失败的恢复时间
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "fail_timeout")]
    fail_timeout: Duration,
    /// 当前连续失败的次数
    #[serde(default = "default_fall_times")]
    fall_times: usize,
    /// 当前连续成功的次数
    #[serde(default = "default_rise_times")]
    rise_times: usize,

    #[serde(skip)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamConfig {
    pub name: String,
    #[serde(default = "String::new")]
    pub bind: String,
    #[serde(default = "Vec::new")]
    pub server: Vec<SingleStreamConfig>,
}

impl UpstreamConfig {
    pub fn get_server_addr(&self) -> Option<SocketAddr> {
        if self.server.is_empty() {
            return None;
        }
        let (sum, sum_all) = self.calc_sum_weight();
        let mut rng = rand::thread_rng();
        if sum != 0 {
            let mut random_weight = rng.gen_range(0..sum);
            for server in &self.server {
                if !HealthCheck::check_fall_down(&server.addr, &server.fail_timeout, &server.fall_times, &server.rise_times) {
                    if random_weight <= server.weight {
                        return Some(server.addr.clone());
                    }
                    random_weight -= server.weight;
                }
            }
        } else {
            let mut random_weight = rng.gen_range(0..sum_all);
            for server in &self.server {
                if random_weight <= server.weight {
                    return Some(server.addr.clone());
                }
                random_weight -= server.weight;
            }
        }
        return None;
        
    }

    pub fn calc_sum_weight(&self) -> (u16, u16) {
        let mut sum = 0;
        let mut sum_all = 0;
        for server in &self.server {
            if !HealthCheck::is_fall_down(&server.addr) {
                sum += server.weight;
            }
            sum_all += server.weight;
        }
        return (sum, sum_all);
    }
}