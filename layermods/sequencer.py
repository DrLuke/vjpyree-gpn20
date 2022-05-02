from PyreeEngine.layers import BaseEntry


class LayerEntry(BaseEntry):
    def __init__(self, context):
        super(LayerEntry, self).__init__(context)

        self.keep = False  # Keep beat on 1 for 1 tick
        self.context.data["beat"] = 0
        self.context.data["beataccum"] = 0

        self.context.oscdispatcher.map("/beat", self.recv_beat)

    def recv_beat(self, addr, *args):
        if addr == "/beat":
            self.context.data["beat"] = 1
            self.keep = True
            self.context.data["beataccum"] += 1

    def init(self):
        pass

    def __del__(self):
        pass

    def tick(self):
        if self.keep:
            self.keep = False
        else:
            self.context.data["beat"] = 0
