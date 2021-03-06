//
// Copyright (C) 2019 Swift Navigation Inc.
// Contact: https://support.swiftnav.com
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

// This file was auto-generated from spec/tests/yaml/swiftnav/sbp/test_piksi.yaml by generate.py. Do not modify by hand!

use sbp::iter_messages;
use sbp::messages::SBPMessage;

mod common;
#[allow(unused_imports)]
use common::AlmostEq;

use std::io::Cursor;

#[test]
fn test_auto_check_sbp_piksi_46() {
    {
        let mut payload = Cursor::new(vec![
            85, 23, 0, 195, 4, 26, 109, 97, 105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 148, 9, 0, 0, 195, 212,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgThreadState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x17,
                    "Incorrect message type, expected 0x17, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.cpu, 0,
                    "incorrect value for cpu, expected 0, is {}",
                    msg.cpu
                );
                assert_eq!(
                    Into::<String>::into(msg.name.clone()),
                    "main                ".to_string(),
                    "incorrect value for msg.name, expected string '{}', is '{}'",
                    "main                ".to_string(),
                    msg.name
                );
                assert_eq!(
                    msg.stack_free, 2452,
                    "incorrect value for stack_free, expected 2452, is {}",
                    msg.stack_free
                );
            }
            _ => panic!("Invalid message type! Expected a MsgThreadState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 23, 0, 195, 4, 26, 105, 100, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 228, 1, 36, 0, 0, 0, 225, 18,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgThreadState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x17,
                    "Incorrect message type, expected 0x17, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.cpu, 484,
                    "incorrect value for cpu, expected 484, is {}",
                    msg.cpu
                );
                assert_eq!(
                    Into::<String>::into(msg.name.clone()),
                    "idle                ".to_string(),
                    "incorrect value for msg.name, expected string '{}', is '{}'",
                    "idle                ".to_string(),
                    msg.name
                );
                assert_eq!(
                    msg.stack_free, 36,
                    "incorrect value for stack_free, expected 36, is {}",
                    msg.stack_free
                );
            }
            _ => panic!("Invalid message type! Expected a MsgThreadState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 23, 0, 195, 4, 26, 78, 65, 80, 32, 73, 83, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 138, 1, 92, 7, 0, 0, 166, 116,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgThreadState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x17,
                    "Incorrect message type, expected 0x17, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.cpu, 394,
                    "incorrect value for cpu, expected 394, is {}",
                    msg.cpu
                );
                assert_eq!(
                    Into::<String>::into(msg.name.clone()),
                    "NAP ISR             ".to_string(),
                    "incorrect value for msg.name, expected string '{}', is '{}'",
                    "NAP ISR             ".to_string(),
                    msg.name
                );
                assert_eq!(
                    msg.stack_free, 1884,
                    "incorrect value for stack_free, expected 1884, is {}",
                    msg.stack_free
                );
            }
            _ => panic!("Invalid message type! Expected a MsgThreadState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 23, 0, 195, 4, 26, 83, 66, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 4, 12, 0, 0, 229, 174,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgThreadState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x17,
                    "Incorrect message type, expected 0x17, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.cpu, 1,
                    "incorrect value for cpu, expected 1, is {}",
                    msg.cpu
                );
                assert_eq!(
                    Into::<String>::into(msg.name.clone()),
                    "SBP                 ".to_string(),
                    "incorrect value for msg.name, expected string '{}', is '{}'",
                    "SBP                 ".to_string(),
                    msg.name
                );
                assert_eq!(
                    msg.stack_free, 3076,
                    "incorrect value for stack_free, expected 3076, is {}",
                    msg.stack_free
                );
            }
            _ => panic!("Invalid message type! Expected a MsgThreadState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 23, 0, 195, 4, 26, 109, 97, 110, 97, 103, 101, 32, 97, 99, 113, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 10, 0, 124, 9, 0, 0, 52, 2,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgThreadState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x17,
                    "Incorrect message type, expected 0x17, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.cpu, 10,
                    "incorrect value for cpu, expected 10, is {}",
                    msg.cpu
                );
                assert_eq!(
                    Into::<String>::into(msg.name.clone()),
                    "manage acq          ".to_string(),
                    "incorrect value for msg.name, expected string '{}', is '{}'",
                    "manage acq          ".to_string(),
                    msg.name
                );
                assert_eq!(
                    msg.stack_free, 2428,
                    "incorrect value for stack_free, expected 2428, is {}",
                    msg.stack_free
                );
            }
            _ => panic!("Invalid message type! Expected a MsgThreadState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 23, 0, 195, 4, 26, 109, 97, 110, 97, 103, 101, 32, 116, 114, 97, 99, 107, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 28, 9, 0, 0, 122, 54,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgThreadState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x17,
                    "Incorrect message type, expected 0x17, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.cpu, 0,
                    "incorrect value for cpu, expected 0, is {}",
                    msg.cpu
                );
                assert_eq!(
                    Into::<String>::into(msg.name.clone()),
                    "manage track        ".to_string(),
                    "incorrect value for msg.name, expected string '{}', is '{}'",
                    "manage track        ".to_string(),
                    msg.name
                );
                assert_eq!(
                    msg.stack_free, 2332,
                    "incorrect value for stack_free, expected 2332, is {}",
                    msg.stack_free
                );
            }
            _ => panic!("Invalid message type! Expected a MsgThreadState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 24, 0, 195, 4, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 154, 153, 57, 65, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 255, 255, 255,
            255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 247, 5,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgUartStateDepa(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x18,
                    "Incorrect message type, expected 0x18, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.latency.avg, -1,
                    "incorrect value for latency.avg, expected -1, is {}",
                    msg.latency.avg
                );
                assert_eq!(
                    msg.latency.current, -1,
                    "incorrect value for latency.current, expected -1, is {}",
                    msg.latency.current
                );
                assert_eq!(
                    msg.latency.lmax, 0,
                    "incorrect value for latency.lmax, expected 0, is {}",
                    msg.latency.lmax
                );
                assert_eq!(
                    msg.latency.lmin, 0,
                    "incorrect value for latency.lmin, expected 0, is {}",
                    msg.latency.lmin
                );
                assert_eq!(
                    msg.uart_a.crc_error_count, 0,
                    "incorrect value for uart_a.crc_error_count, expected 0, is {}",
                    msg.uart_a.crc_error_count
                );
                assert_eq!(
                    msg.uart_a.io_error_count, 0,
                    "incorrect value for uart_a.io_error_count, expected 0, is {}",
                    msg.uart_a.io_error_count
                );
                assert_eq!(
                    msg.uart_a.rx_buffer_level, 0,
                    "incorrect value for uart_a.rx_buffer_level, expected 0, is {}",
                    msg.uart_a.rx_buffer_level
                );
                assert!(msg.uart_a.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.rx_throughput);
                assert_eq!(
                    msg.uart_a.tx_buffer_level, 0,
                    "incorrect value for uart_a.tx_buffer_level, expected 0, is {}",
                    msg.uart_a.tx_buffer_level
                );
                assert!(msg.uart_a.tx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.tx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.tx_throughput);
                assert_eq!(
                    msg.uart_b.crc_error_count, 0,
                    "incorrect value for uart_b.crc_error_count, expected 0, is {}",
                    msg.uart_b.crc_error_count
                );
                assert_eq!(
                    msg.uart_b.io_error_count, 0,
                    "incorrect value for uart_b.io_error_count, expected 0, is {}",
                    msg.uart_b.io_error_count
                );
                assert_eq!(
                    msg.uart_b.rx_buffer_level, 0,
                    "incorrect value for uart_b.rx_buffer_level, expected 0, is {}",
                    msg.uart_b.rx_buffer_level
                );
                assert!(msg.uart_b.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.rx_throughput);
                assert_eq!(
                    msg.uart_b.tx_buffer_level, 0,
                    "incorrect value for uart_b.tx_buffer_level, expected 0, is {}",
                    msg.uart_b.tx_buffer_level
                );
                assert!(msg.uart_b.tx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.tx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.tx_throughput);
                assert_eq!(
                    msg.uart_ftdi.crc_error_count, 0,
                    "incorrect value for uart_ftdi.crc_error_count, expected 0, is {}",
                    msg.uart_ftdi.crc_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.io_error_count, 0,
                    "incorrect value for uart_ftdi.io_error_count, expected 0, is {}",
                    msg.uart_ftdi.io_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.rx_buffer_level, 0,
                    "incorrect value for uart_ftdi.rx_buffer_level, expected 0, is {}",
                    msg.uart_ftdi.rx_buffer_level
                );
                assert!(msg.uart_ftdi.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_ftdi.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_ftdi.rx_throughput);
                assert_eq!(
                    msg.uart_ftdi.tx_buffer_level, 15,
                    "incorrect value for uart_ftdi.tx_buffer_level, expected 15, is {}",
                    msg.uart_ftdi.tx_buffer_level
                );
                assert!(msg.uart_ftdi.tx_throughput.almost_eq( 1.16000003814697266e+01 ), "incorrect value for uart_ftdi.tx_throughput, expected 1.16000003814697266e+01, is {:e}", msg.uart_ftdi.tx_throughput);
            }
            _ => panic!("Invalid message type! Expected a MsgUartStateDepa"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 24, 0, 195, 4, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 2, 43, 135, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 65, 110,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgUartStateDepa(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x18,
                    "Incorrect message type, expected 0x18, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.latency.avg, -1,
                    "incorrect value for latency.avg, expected -1, is {}",
                    msg.latency.avg
                );
                assert_eq!(
                    msg.latency.current, -1,
                    "incorrect value for latency.current, expected -1, is {}",
                    msg.latency.current
                );
                assert_eq!(
                    msg.latency.lmax, 0,
                    "incorrect value for latency.lmax, expected 0, is {}",
                    msg.latency.lmax
                );
                assert_eq!(
                    msg.latency.lmin, 0,
                    "incorrect value for latency.lmin, expected 0, is {}",
                    msg.latency.lmin
                );
                assert_eq!(
                    msg.uart_a.crc_error_count, 0,
                    "incorrect value for uart_a.crc_error_count, expected 0, is {}",
                    msg.uart_a.crc_error_count
                );
                assert_eq!(
                    msg.uart_a.io_error_count, 0,
                    "incorrect value for uart_a.io_error_count, expected 0, is {}",
                    msg.uart_a.io_error_count
                );
                assert_eq!(
                    msg.uart_a.rx_buffer_level, 0,
                    "incorrect value for uart_a.rx_buffer_level, expected 0, is {}",
                    msg.uart_a.rx_buffer_level
                );
                assert!(msg.uart_a.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.rx_throughput);
                assert_eq!(
                    msg.uart_a.tx_buffer_level, 0,
                    "incorrect value for uart_a.tx_buffer_level, expected 0, is {}",
                    msg.uart_a.tx_buffer_level
                );
                assert!(msg.uart_a.tx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.tx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.tx_throughput);
                assert_eq!(
                    msg.uart_b.crc_error_count, 0,
                    "incorrect value for uart_b.crc_error_count, expected 0, is {}",
                    msg.uart_b.crc_error_count
                );
                assert_eq!(
                    msg.uart_b.io_error_count, 0,
                    "incorrect value for uart_b.io_error_count, expected 0, is {}",
                    msg.uart_b.io_error_count
                );
                assert_eq!(
                    msg.uart_b.rx_buffer_level, 0,
                    "incorrect value for uart_b.rx_buffer_level, expected 0, is {}",
                    msg.uart_b.rx_buffer_level
                );
                assert!(msg.uart_b.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.rx_throughput);
                assert_eq!(
                    msg.uart_b.tx_buffer_level, 0,
                    "incorrect value for uart_b.tx_buffer_level, expected 0, is {}",
                    msg.uart_b.tx_buffer_level
                );
                assert!(msg.uart_b.tx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.tx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.tx_throughput);
                assert_eq!(
                    msg.uart_ftdi.crc_error_count, 0,
                    "incorrect value for uart_ftdi.crc_error_count, expected 0, is {}",
                    msg.uart_ftdi.crc_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.io_error_count, 0,
                    "incorrect value for uart_ftdi.io_error_count, expected 0, is {}",
                    msg.uart_ftdi.io_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.rx_buffer_level, 0,
                    "incorrect value for uart_ftdi.rx_buffer_level, expected 0, is {}",
                    msg.uart_ftdi.rx_buffer_level
                );
                assert!(msg.uart_ftdi.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_ftdi.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_ftdi.rx_throughput);
                assert_eq!(
                    msg.uart_ftdi.tx_buffer_level, 0,
                    "incorrect value for uart_ftdi.tx_buffer_level, expected 0, is {}",
                    msg.uart_ftdi.tx_buffer_level
                );
                assert!(msg.uart_ftdi.tx_throughput.almost_eq( 6.59999996423721313e-02 ), "incorrect value for uart_ftdi.tx_throughput, expected 6.59999996423721313e-02, is {:e}", msg.uart_ftdi.tx_throughput);
            }
            _ => panic!("Invalid message type! Expected a MsgUartStateDepa"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 24, 0, 195, 4, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 4, 86, 14, 62, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 198, 36,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgUartStateDepa(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x18,
                    "Incorrect message type, expected 0x18, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.latency.avg, -1,
                    "incorrect value for latency.avg, expected -1, is {}",
                    msg.latency.avg
                );
                assert_eq!(
                    msg.latency.current, -1,
                    "incorrect value for latency.current, expected -1, is {}",
                    msg.latency.current
                );
                assert_eq!(
                    msg.latency.lmax, 0,
                    "incorrect value for latency.lmax, expected 0, is {}",
                    msg.latency.lmax
                );
                assert_eq!(
                    msg.latency.lmin, 0,
                    "incorrect value for latency.lmin, expected 0, is {}",
                    msg.latency.lmin
                );
                assert_eq!(
                    msg.uart_a.crc_error_count, 0,
                    "incorrect value for uart_a.crc_error_count, expected 0, is {}",
                    msg.uart_a.crc_error_count
                );
                assert_eq!(
                    msg.uart_a.io_error_count, 0,
                    "incorrect value for uart_a.io_error_count, expected 0, is {}",
                    msg.uart_a.io_error_count
                );
                assert_eq!(
                    msg.uart_a.rx_buffer_level, 0,
                    "incorrect value for uart_a.rx_buffer_level, expected 0, is {}",
                    msg.uart_a.rx_buffer_level
                );
                assert!(msg.uart_a.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.rx_throughput);
                assert_eq!(
                    msg.uart_a.tx_buffer_level, 0,
                    "incorrect value for uart_a.tx_buffer_level, expected 0, is {}",
                    msg.uart_a.tx_buffer_level
                );
                assert!(msg.uart_a.tx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.tx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.tx_throughput);
                assert_eq!(
                    msg.uart_b.crc_error_count, 0,
                    "incorrect value for uart_b.crc_error_count, expected 0, is {}",
                    msg.uart_b.crc_error_count
                );
                assert_eq!(
                    msg.uart_b.io_error_count, 0,
                    "incorrect value for uart_b.io_error_count, expected 0, is {}",
                    msg.uart_b.io_error_count
                );
                assert_eq!(
                    msg.uart_b.rx_buffer_level, 0,
                    "incorrect value for uart_b.rx_buffer_level, expected 0, is {}",
                    msg.uart_b.rx_buffer_level
                );
                assert!(msg.uart_b.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.rx_throughput);
                assert_eq!(
                    msg.uart_b.tx_buffer_level, 0,
                    "incorrect value for uart_b.tx_buffer_level, expected 0, is {}",
                    msg.uart_b.tx_buffer_level
                );
                assert!(msg.uart_b.tx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.tx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.tx_throughput);
                assert_eq!(
                    msg.uart_ftdi.crc_error_count, 0,
                    "incorrect value for uart_ftdi.crc_error_count, expected 0, is {}",
                    msg.uart_ftdi.crc_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.io_error_count, 0,
                    "incorrect value for uart_ftdi.io_error_count, expected 0, is {}",
                    msg.uart_ftdi.io_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.rx_buffer_level, 0,
                    "incorrect value for uart_ftdi.rx_buffer_level, expected 0, is {}",
                    msg.uart_ftdi.rx_buffer_level
                );
                assert!(msg.uart_ftdi.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_ftdi.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_ftdi.rx_throughput);
                assert_eq!(
                    msg.uart_ftdi.tx_buffer_level, 10,
                    "incorrect value for uart_ftdi.tx_buffer_level, expected 10, is {}",
                    msg.uart_ftdi.tx_buffer_level
                );
                assert!(msg.uart_ftdi.tx_throughput.almost_eq( 1.38999998569488525e-01 ), "incorrect value for uart_ftdi.tx_throughput, expected 1.38999998569488525e-01, is {:e}", msg.uart_ftdi.tx_throughput);
            }
            _ => panic!("Invalid message type! Expected a MsgUartStateDepa"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 24, 0, 195, 4, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 2, 43, 135, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 65, 110,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgUartStateDepa(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x18,
                    "Incorrect message type, expected 0x18, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.latency.avg, -1,
                    "incorrect value for latency.avg, expected -1, is {}",
                    msg.latency.avg
                );
                assert_eq!(
                    msg.latency.current, -1,
                    "incorrect value for latency.current, expected -1, is {}",
                    msg.latency.current
                );
                assert_eq!(
                    msg.latency.lmax, 0,
                    "incorrect value for latency.lmax, expected 0, is {}",
                    msg.latency.lmax
                );
                assert_eq!(
                    msg.latency.lmin, 0,
                    "incorrect value for latency.lmin, expected 0, is {}",
                    msg.latency.lmin
                );
                assert_eq!(
                    msg.uart_a.crc_error_count, 0,
                    "incorrect value for uart_a.crc_error_count, expected 0, is {}",
                    msg.uart_a.crc_error_count
                );
                assert_eq!(
                    msg.uart_a.io_error_count, 0,
                    "incorrect value for uart_a.io_error_count, expected 0, is {}",
                    msg.uart_a.io_error_count
                );
                assert_eq!(
                    msg.uart_a.rx_buffer_level, 0,
                    "incorrect value for uart_a.rx_buffer_level, expected 0, is {}",
                    msg.uart_a.rx_buffer_level
                );
                assert!(msg.uart_a.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.rx_throughput);
                assert_eq!(
                    msg.uart_a.tx_buffer_level, 0,
                    "incorrect value for uart_a.tx_buffer_level, expected 0, is {}",
                    msg.uart_a.tx_buffer_level
                );
                assert!(msg.uart_a.tx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.tx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.tx_throughput);
                assert_eq!(
                    msg.uart_b.crc_error_count, 0,
                    "incorrect value for uart_b.crc_error_count, expected 0, is {}",
                    msg.uart_b.crc_error_count
                );
                assert_eq!(
                    msg.uart_b.io_error_count, 0,
                    "incorrect value for uart_b.io_error_count, expected 0, is {}",
                    msg.uart_b.io_error_count
                );
                assert_eq!(
                    msg.uart_b.rx_buffer_level, 0,
                    "incorrect value for uart_b.rx_buffer_level, expected 0, is {}",
                    msg.uart_b.rx_buffer_level
                );
                assert!(msg.uart_b.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.rx_throughput);
                assert_eq!(
                    msg.uart_b.tx_buffer_level, 0,
                    "incorrect value for uart_b.tx_buffer_level, expected 0, is {}",
                    msg.uart_b.tx_buffer_level
                );
                assert!(msg.uart_b.tx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.tx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.tx_throughput);
                assert_eq!(
                    msg.uart_ftdi.crc_error_count, 0,
                    "incorrect value for uart_ftdi.crc_error_count, expected 0, is {}",
                    msg.uart_ftdi.crc_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.io_error_count, 0,
                    "incorrect value for uart_ftdi.io_error_count, expected 0, is {}",
                    msg.uart_ftdi.io_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.rx_buffer_level, 0,
                    "incorrect value for uart_ftdi.rx_buffer_level, expected 0, is {}",
                    msg.uart_ftdi.rx_buffer_level
                );
                assert!(msg.uart_ftdi.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_ftdi.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_ftdi.rx_throughput);
                assert_eq!(
                    msg.uart_ftdi.tx_buffer_level, 0,
                    "incorrect value for uart_ftdi.tx_buffer_level, expected 0, is {}",
                    msg.uart_ftdi.tx_buffer_level
                );
                assert!(msg.uart_ftdi.tx_throughput.almost_eq( 6.59999996423721313e-02 ), "incorrect value for uart_ftdi.tx_throughput, expected 6.59999996423721313e-02, is {:e}", msg.uart_ftdi.tx_throughput);
            }
            _ => panic!("Invalid message type! Expected a MsgUartStateDepa"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 24, 0, 195, 4, 58, 0, 0, 0, 0, 138, 75, 6, 60, 0, 0, 0, 0, 0, 0, 80, 113, 201, 61,
            0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 145, 237, 252, 62, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 255,
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 112, 111,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgUartStateDepa(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x18,
                    "Incorrect message type, expected 0x18, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.latency.avg, -1,
                    "incorrect value for latency.avg, expected -1, is {}",
                    msg.latency.avg
                );
                assert_eq!(
                    msg.latency.current, -1,
                    "incorrect value for latency.current, expected -1, is {}",
                    msg.latency.current
                );
                assert_eq!(
                    msg.latency.lmax, 0,
                    "incorrect value for latency.lmax, expected 0, is {}",
                    msg.latency.lmax
                );
                assert_eq!(
                    msg.latency.lmin, 0,
                    "incorrect value for latency.lmin, expected 0, is {}",
                    msg.latency.lmin
                );
                assert_eq!(
                    msg.uart_a.crc_error_count, 0,
                    "incorrect value for uart_a.crc_error_count, expected 0, is {}",
                    msg.uart_a.crc_error_count
                );
                assert_eq!(
                    msg.uart_a.io_error_count, 0,
                    "incorrect value for uart_a.io_error_count, expected 0, is {}",
                    msg.uart_a.io_error_count
                );
                assert_eq!(
                    msg.uart_a.rx_buffer_level, 0,
                    "incorrect value for uart_a.rx_buffer_level, expected 0, is {}",
                    msg.uart_a.rx_buffer_level
                );
                assert!(msg.uart_a.rx_throughput.almost_eq( 8.19672085344791412e-03 ), "incorrect value for uart_a.rx_throughput, expected 8.19672085344791412e-03, is {:e}", msg.uart_a.rx_throughput);
                assert_eq!(
                    msg.uart_a.tx_buffer_level, 0,
                    "incorrect value for uart_a.tx_buffer_level, expected 0, is {}",
                    msg.uart_a.tx_buffer_level
                );
                assert!(msg.uart_a.tx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.tx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.tx_throughput);
                assert_eq!(
                    msg.uart_b.crc_error_count, 0,
                    "incorrect value for uart_b.crc_error_count, expected 0, is {}",
                    msg.uart_b.crc_error_count
                );
                assert_eq!(
                    msg.uart_b.io_error_count, 0,
                    "incorrect value for uart_b.io_error_count, expected 0, is {}",
                    msg.uart_b.io_error_count
                );
                assert_eq!(
                    msg.uart_b.rx_buffer_level, 0,
                    "incorrect value for uart_b.rx_buffer_level, expected 0, is {}",
                    msg.uart_b.rx_buffer_level
                );
                assert!(msg.uart_b.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.rx_throughput);
                assert_eq!(
                    msg.uart_b.tx_buffer_level, 2,
                    "incorrect value for uart_b.tx_buffer_level, expected 2, is {}",
                    msg.uart_b.tx_buffer_level
                );
                assert!(msg.uart_b.tx_throughput.almost_eq( 9.83606576919555664e-02 ), "incorrect value for uart_b.tx_throughput, expected 9.83606576919555664e-02, is {:e}", msg.uart_b.tx_throughput);
                assert_eq!(
                    msg.uart_ftdi.crc_error_count, 0,
                    "incorrect value for uart_ftdi.crc_error_count, expected 0, is {}",
                    msg.uart_ftdi.crc_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.io_error_count, 0,
                    "incorrect value for uart_ftdi.io_error_count, expected 0, is {}",
                    msg.uart_ftdi.io_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.rx_buffer_level, 0,
                    "incorrect value for uart_ftdi.rx_buffer_level, expected 0, is {}",
                    msg.uart_ftdi.rx_buffer_level
                );
                assert!(msg.uart_ftdi.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_ftdi.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_ftdi.rx_throughput);
                assert_eq!(
                    msg.uart_ftdi.tx_buffer_level, 38,
                    "incorrect value for uart_ftdi.tx_buffer_level, expected 38, is {}",
                    msg.uart_ftdi.tx_buffer_level
                );
                assert!(msg.uart_ftdi.tx_throughput.almost_eq( 4.93999987840652466e-01 ), "incorrect value for uart_ftdi.tx_throughput, expected 4.93999987840652466e-01, is {:e}", msg.uart_ftdi.tx_throughput);
            }
            _ => panic!("Invalid message type! Expected a MsgUartStateDepa"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![
            85, 24, 0, 195, 4, 58, 166, 155, 68, 60, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 166, 155, 68,
            60, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 236, 81, 168, 63, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 255,
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 22, 72,
        ]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgUartStateDepa(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x18,
                    "Incorrect message type, expected 0x18, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.latency.avg, -1,
                    "incorrect value for latency.avg, expected -1, is {}",
                    msg.latency.avg
                );
                assert_eq!(
                    msg.latency.current, -1,
                    "incorrect value for latency.current, expected -1, is {}",
                    msg.latency.current
                );
                assert_eq!(
                    msg.latency.lmax, 0,
                    "incorrect value for latency.lmax, expected 0, is {}",
                    msg.latency.lmax
                );
                assert_eq!(
                    msg.latency.lmin, 0,
                    "incorrect value for latency.lmin, expected 0, is {}",
                    msg.latency.lmin
                );
                assert_eq!(
                    msg.uart_a.crc_error_count, 0,
                    "incorrect value for uart_a.crc_error_count, expected 0, is {}",
                    msg.uart_a.crc_error_count
                );
                assert_eq!(
                    msg.uart_a.io_error_count, 0,
                    "incorrect value for uart_a.io_error_count, expected 0, is {}",
                    msg.uart_a.io_error_count
                );
                assert_eq!(
                    msg.uart_a.rx_buffer_level, 0,
                    "incorrect value for uart_a.rx_buffer_level, expected 0, is {}",
                    msg.uart_a.rx_buffer_level
                );
                assert!(msg.uart_a.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_a.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_a.rx_throughput);
                assert_eq!(
                    msg.uart_a.tx_buffer_level, 2,
                    "incorrect value for uart_a.tx_buffer_level, expected 2, is {}",
                    msg.uart_a.tx_buffer_level
                );
                assert!(msg.uart_a.tx_throughput.almost_eq( 1.20000001043081284e-02 ), "incorrect value for uart_a.tx_throughput, expected 1.20000001043081284e-02, is {:e}", msg.uart_a.tx_throughput);
                assert_eq!(
                    msg.uart_b.crc_error_count, 0,
                    "incorrect value for uart_b.crc_error_count, expected 0, is {}",
                    msg.uart_b.crc_error_count
                );
                assert_eq!(
                    msg.uart_b.io_error_count, 0,
                    "incorrect value for uart_b.io_error_count, expected 0, is {}",
                    msg.uart_b.io_error_count
                );
                assert_eq!(
                    msg.uart_b.rx_buffer_level, 0,
                    "incorrect value for uart_b.rx_buffer_level, expected 0, is {}",
                    msg.uart_b.rx_buffer_level
                );
                assert!(msg.uart_b.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_b.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_b.rx_throughput);
                assert_eq!(
                    msg.uart_b.tx_buffer_level, 2,
                    "incorrect value for uart_b.tx_buffer_level, expected 2, is {}",
                    msg.uart_b.tx_buffer_level
                );
                assert!(msg.uart_b.tx_throughput.almost_eq( 1.20000001043081284e-02 ), "incorrect value for uart_b.tx_throughput, expected 1.20000001043081284e-02, is {:e}", msg.uart_b.tx_throughput);
                assert_eq!(
                    msg.uart_ftdi.crc_error_count, 0,
                    "incorrect value for uart_ftdi.crc_error_count, expected 0, is {}",
                    msg.uart_ftdi.crc_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.io_error_count, 0,
                    "incorrect value for uart_ftdi.io_error_count, expected 0, is {}",
                    msg.uart_ftdi.io_error_count
                );
                assert_eq!(
                    msg.uart_ftdi.rx_buffer_level, 0,
                    "incorrect value for uart_ftdi.rx_buffer_level, expected 0, is {}",
                    msg.uart_ftdi.rx_buffer_level
                );
                assert!(msg.uart_ftdi.rx_throughput.almost_eq( 0.00000000000000000e+00 ), "incorrect value for uart_ftdi.rx_throughput, expected 0.00000000000000000e+00, is {:e}", msg.uart_ftdi.rx_throughput);
                assert_eq!(
                    msg.uart_ftdi.tx_buffer_level, 50,
                    "incorrect value for uart_ftdi.tx_buffer_level, expected 50, is {}",
                    msg.uart_ftdi.tx_buffer_level
                );
                assert!(msg.uart_ftdi.tx_throughput.almost_eq( 1.31500005722045898e+00 ), "incorrect value for uart_ftdi.tx_throughput, expected 1.31500005722045898e+00, is {:e}", msg.uart_ftdi.tx_throughput);
            }
            _ => panic!("Invalid message type! Expected a MsgUartStateDepa"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![85, 25, 0, 195, 4, 4, 0, 0, 0, 0, 18, 176]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgIarState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x19,
                    "Incorrect message type, expected 0x19, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.num_hyps, 0,
                    "incorrect value for num_hyps, expected 0, is {}",
                    msg.num_hyps
                );
            }
            _ => panic!("Invalid message type! Expected a MsgIarState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![85, 25, 0, 195, 4, 4, 1, 0, 0, 0, 166, 198]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgIarState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x19,
                    "Incorrect message type, expected 0x19, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.num_hyps, 1,
                    "incorrect value for num_hyps, expected 1, is {}",
                    msg.num_hyps
                );
            }
            _ => panic!("Invalid message type! Expected a MsgIarState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![85, 25, 0, 195, 4, 4, 217, 2, 0, 0, 6, 133]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgIarState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x19,
                    "Incorrect message type, expected 0x19, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.num_hyps, 729,
                    "incorrect value for num_hyps, expected 729, is {}",
                    msg.num_hyps
                );
            }
            _ => panic!("Invalid message type! Expected a MsgIarState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![85, 25, 0, 195, 4, 4, 216, 2, 0, 0, 178, 243]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgIarState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x19,
                    "Incorrect message type, expected 0x19, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.num_hyps, 728,
                    "incorrect value for num_hyps, expected 728, is {}",
                    msg.num_hyps
                );
            }
            _ => panic!("Invalid message type! Expected a MsgIarState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![85, 25, 0, 195, 4, 4, 215, 2, 0, 0, 92, 39]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgIarState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x19,
                    "Incorrect message type, expected 0x19, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.num_hyps, 727,
                    "incorrect value for num_hyps, expected 727, is {}",
                    msg.num_hyps
                );
            }
            _ => panic!("Invalid message type! Expected a MsgIarState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
    {
        let mut payload = Cursor::new(vec![85, 25, 0, 195, 4, 4, 211, 2, 0, 0, 173, 237]);

        // Test the round trip payload parsing
        let sbp_msg = {
            let mut msgs = iter_messages(&mut payload);
            msgs.next()
                .expect("no message found")
                .expect("failed to parse message")
        };
        match &sbp_msg {
            sbp::messages::SBP::MsgIarState(msg) => {
                assert_eq!(
                    msg.get_message_type(),
                    0x19,
                    "Incorrect message type, expected 0x19, is {}",
                    msg.get_message_type()
                );
                let sender_id = msg.get_sender_id().unwrap();
                assert_eq!(
                    sender_id, 0x4c3,
                    "incorrect sender id, expected 0x4c3, is {}",
                    sender_id
                );
                assert_eq!(
                    msg.num_hyps, 723,
                    "incorrect value for num_hyps, expected 723, is {}",
                    msg.num_hyps
                );
            }
            _ => panic!("Invalid message type! Expected a MsgIarState"),
        };
        let frame = sbp_msg.to_frame().unwrap();
        assert_eq!(frame, payload.into_inner());
    }
}
