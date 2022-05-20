import mido
from pythonosc import udp_client
import time


oscip = "127.0.0.1"
oscport = 31337


client = udp_client.SimpleUDPClient(oscip, oscport)

# Read from midi
#with mido.open_input() as inport:
#    while True:
#        for msg in inport:
#            client.send_message("/traktor/beat", msg.type)

while True:
    time.sleep(0.04)
    client.send_message("/traktor/beat", [])