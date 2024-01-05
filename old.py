#!/usr/bin/python3
import time, copy
import modbus
import boards
from anchor.can_util import Node, Msg, Field as Fd, debug
from anchor import canable

#

# pgnHelm = [
field = Fd(off = 48 - 16, len = 2, mul = 1, add = 0, n = 'Gear', u = '',
       enum = {
           0x0: 'Forward',
           0x1: 'Neutral',
           0x2: 'Backward',
           0x3: 'Stop',  # apparently when engine stops
       }, val = 0x3)
# ]

# node = Node()
# star = Node()
# node.msgs[65284] = Msg("Helm", pgnHelm)
# star.msgs[65284] = Msg("Helm", copy.deepcopy(pgnHelm))

global star_val
global port_val

#

_modbus = modbus.Modbus(serial_port='/dev/ttyUSB0', verbose=False)
_modbus.open()

board = boards.R421A08(_modbus,
                address=1,
                board_name='gears',
                verbose=False)

board.off_all()

#

def update():
    global star_val
    global port_val

    if star_val == 0:  # Forward
        # print('star forward')
        board.on(2)
        board.off(1)
    if star_val == 1:  # neutral
        # print('star neutral')
        board.off(2)
        board.off(1)
    if star_val == 2:  # Backward
        # print('star backward')
        board.off(2)
        board.on(1)
    if star_val == 3:  # Stop
        # print('star stop')
        board.off(2)
        board.off(1)

    # gear: Fd = port.msgs[65284].fields[0]

    if port_val == 0:  # Forward
        # print('port forward')
        board.on(6)
        board.off(7)
    if port_val == 1:  # Neutral
        # print('port neutral')
        board.off(6)
        board.off(7)
    if port_val == 2:  # Backward
        # print('port backward')
        board.off(6)
        board.on(7)
    if port_val == 3:  # Stop
        # print('port stop')
        board.off(6)
        board.off(7)


def recv(nid: int, data: int):
    global star_val
    global port_val

    # pri = nid >> 26
    pgn = (nid >> 8) & ((1 << 17) - 1)
    # src = nid & 0xff
    # debug(f'parsing {hex(nid)}, data={hex(data)}')
    # debug(f'pgn {pgn}')
    if pgn == 65284:
        if data & (1 << (5 + 16)):
            # msg = star.msgs.get(pgn)
            # print('star')
            val = field.recv(data)
            if star_val != val:
                star_val = val
                update()
        else:
            # msg = port.msgs.get(pgn)
            # print('port')
            val = field.recv(data)
            if port_val != val:
                port_val = val
                update()
    # if msg:
    #     for f in msg.fields:
    #         val = f.recv(data)
    #         # val = f.recv_NMEA2000(data)
    #         if f.val != val:
    #             print(val)
    #             f.val = f.recv(data)
    # debug(msg)


def main():
    global star_val
    global port_val

    # exit(0)

    #

    can = canable.Can(port = 'can0')

    #

    last = time.monotonic()
    star_val = 3
    port_val = 3

    while True:
        nid, data = can.recv_little()
        recv(nid, data)

        now = time.monotonic()
        if now - last > 1:
            last = now
            # s = ''
            # for m in port.msgs.values():
            #     s += str(m)
            # s += '| '
            # for m in star.msgs.values():
            #     s += str(m)
            # print(s)

            # gear: Fd = star.msgs[65284].fields[0]
            update()


if __name__ == '__main__':
    main()
