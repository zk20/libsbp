# Copyright (C) 2015 Swift Navigation Inc.
# Contact: Mark Fine <mark@swiftnav.com>
#
# This source is subject to the license found in the file 'LICENSE' which must
# be be distributed together with this source. All other rights reserved.
#
# THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
# EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
# WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

import io

from .base_driver import BaseDriver


class FileDriver(BaseDriver):
    """
    BaseDriver

    The :class:`BaseDriver` class wraps IO sources of SBP messages and provides
    context management.

    Parameters
    ----------
    handle : port
      Stream of bytes to read from and write to.
    """

    def read(self, size):
        """
        Read wrapper.

        Parameters
        ----------
        size : int
          Number of bytes to read.
        """
        return_val = self.handle.read(size)
        if not return_val:
            raise IOError
        else: 
            return return_val

    def readinto(self, b):
        """
        Read into wrapper.

        Parameters
        ----------
        b : buffer
          Buffer to read into
        """
        read = self._reader.readinto(b)
        if not read:
            raise IOError
        else: 
            return read

    def __init__(self, fd):
        self.handle = fd
        self._reader = io.open(fd.fileno(), 'rb')

    def __del__(self):
        self._reader.close()
