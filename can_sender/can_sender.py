#!/usr/bin/env python

import can
import argparse

class CanMessage:
    def __init__(self, timestamp, arbitration_id, data):
        self.timestamp = timestamp
        self.arbitration_id = arbitration_id
        self.data = data

    def __str__(self):
        return self.timestamp + ' ' + str(self.arbitration_id) + ' ' + str(self.data)

    def __repr__(self):
        return self.__str__()

# Create a parser
parser = argparse.ArgumentParser(description='Send CAN messages from a file')

# Add arguments
parser.add_argument('--channel', type=str, help='The CAN channel to use')
parser.add_argument('--bitrate', type=int, help='The CAN bitrate to use')
parser.add_argument('--path', type=str, help='The path to the file')

# Parse the arguments
args = parser.parse_args()

# Get channel and bitrate from the args --channel and --bitrate
channel = args.channel
bitrate = args.bitrate

# Get path from the args --path
path = args.path

# Create a bus instance
bus = can.interface.Bus(interface='socketcan', channel=channel, bitrate=bitrate)

# Open the file
file = open(path, 'r')

# Read the file line by line
can_messages = []
for line in file:
    # Split the line into a list
    line = line.split()

    # Get the timestamp
    timestamp = line[0] + ' ' + line[1]

    # Get the arbitration ID
    arbitration_id = int(line[4])

    # Get the data
    data = line[10:]

    # Append the message to the list
    message = CanMessage(timestamp, arbitration_id, data)
    can_messages.append(message)

# Close the file
file.close()

# Send the messages
for message in can_messages:
    # Create a message
    msg = can.Message(arbitration_id=message.arbitration_id, data=message.data, extended_id=False)

    # Send the message
    bus.send(msg)

    # Print the message
    print(message)