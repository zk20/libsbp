#!/usr/bin/env python
# Copyright (C) 2015-2018 Swift Navigation Inc.
# Contact: Swift Navigation <dev@swiftnav.com>
#
# This source is subject to the license found in the file 'LICENSE' which must
# be be distributed together with this source. All other rights reserved.
#
# THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
# EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
# WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.


"""
Inertial Measurement Unit (IMU) messages.
"""

import json

from sbp.jit.msg import SBP, SENDER_ID
from sbp.jit.msg import get_u8, get_u16, get_u32, get_u64
from sbp.jit.msg import get_s8, get_s16, get_s32, get_s64
from sbp.jit.msg import get_f32, get_f64
from sbp.jit.msg import get_string, get_fixed_string
from sbp.jit.msg import get_array, get_fixed_array

# Automatically generated from piksi/yaml/swiftnav/sbp/imu.yaml with generate.py.
# Please do not hand edit!
SBP_MSG_IMU_RAW = 0x0900
class MsgImuRaw(SBP):
  """SBP class for message MSG_IMU_RAW (0x0900).

  You can have MSG_IMU_RAW inherit its fields directly
  from an inherited SBP object, or construct it inline using a dict
  of its fields.

  
  Raw data from the Inertial Measurement Unit, containing accelerometer and
gyroscope readings. The sense of the measurements are to be aligned with 
the indications on the device itself. Measurement units, which are specific to the
device hardware and settings, are communicated via the MSG_IMU_AUX message.


  """
  __slots__ = ['tow',
               'tow_f',
               'acc_x',
               'acc_y',
               'acc_z',
               'gyr_x',
               'gyr_y',
               'gyr_z',
               ]
  @classmethod
  def parse_members(cls, buf, offset, length):
    o_0 = offset
    o_1, (__tow, offset, length) = offset, get_u32(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__tow_f, offset, length) = offset, get_u8(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__acc_x, offset, length) = offset, get_s16(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__acc_y, offset, length) = offset, get_s16(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__acc_z, offset, length) = offset, get_s16(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__gyr_x, offset, length) = offset, get_s16(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__gyr_y, offset, length) = offset, get_s16(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__gyr_z, offset, length) = offset, get_s16(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    return {
      'tow' : __tow,
      'tow_f' : __tow_f,
      'acc_x' : __acc_x,
      'acc_y' : __acc_y,
      'acc_z' : __acc_z,
      'gyr_x' : __gyr_x,
      'gyr_y' : __gyr_y,
      'gyr_z' : __gyr_z,
    }, offset, length

  def _unpack_members(self, buf, offset, length):
    res, off, length = self.parse_members(buf, offset, length)
    if off == offset:
      return {}, offset, length
    self.tow = res['tow']
    self.tow_f = res['tow_f']
    self.acc_x = res['acc_x']
    self.acc_y = res['acc_y']
    self.acc_z = res['acc_z']
    self.gyr_x = res['gyr_x']
    self.gyr_y = res['gyr_y']
    self.gyr_z = res['gyr_z']
    return res, off, length
  
SBP_MSG_IMU_AUX = 0x0901
class MsgImuAux(SBP):
  """SBP class for message MSG_IMU_AUX (0x0901).

  You can have MSG_IMU_AUX inherit its fields directly
  from an inherited SBP object, or construct it inline using a dict
  of its fields.

  
  Auxiliary data specific to a particular IMU. The `imu_type` field will
always be consistent but the rest of the payload is device specific and
depends on the value of `imu_type`.


  """
  __slots__ = ['imu_type',
               'temp',
               'imu_conf',
               ]
  @classmethod
  def parse_members(cls, buf, offset, length):
    o_0 = offset
    o_1, (__imu_type, offset, length) = offset, get_u8(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__temp, offset, length) = offset, get_s16(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__imu_conf, offset, length) = offset, get_u8(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    return {
      'imu_type' : __imu_type,
      'temp' : __temp,
      'imu_conf' : __imu_conf,
    }, offset, length

  def _unpack_members(self, buf, offset, length):
    res, off, length = self.parse_members(buf, offset, length)
    if off == offset:
      return {}, offset, length
    self.imu_type = res['imu_type']
    self.temp = res['temp']
    self.imu_conf = res['imu_conf']
    return res, off, length
  

msg_classes = {
  0x0900: MsgImuRaw,
  0x0901: MsgImuAux,
}