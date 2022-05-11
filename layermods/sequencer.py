from PyreeEngine.layers import BaseEntry

from pyutil.transition import PT1Transition


class LayerEntry(BaseEntry):
    def __init__(self, context):
        super(LayerEntry, self).__init__(context)

        self.keep = False  # Keep beat on 1 for 1 tick
        self.context.data["beat"] = 0
        self.context.data["beataccum"] = 0

        pt1constant = 5.
        self.beattransition = PT1Transition(pt1constant)
        self.beataccumtransition = PT1Transition(pt1constant)

        self.context.oscdispatcher.map("/beat", self.recv_beat)

    def recv_beat(self, addr, *args):
        if addr == "/beat":
            self.context.data["beat"] = 1
            self.keep = True
            self.context.data["beataccum"] += 1
            self.beattransition.hardset(1)
            self.beattransition.settarget(0)
            self.beataccumtransition.settarget(self.context.data["beataccum"])

    def init(self):
        pass

    def __del__(self):
        pass

    def tick(self):
        if self.keep:
            self.keep = False
        else:
            self.context.data["beat"] = 0

        self.beattransition.tick(self.context.dt)
        self.beataccumtransition.tick(self.context.dt)

        self.context.data["beatpt1"] = self.beattransition.curval
        self.context.data["beataccumpt1"] = self.beataccumtransition.curval
