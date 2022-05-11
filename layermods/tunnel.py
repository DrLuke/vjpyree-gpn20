from pathlib import Path

from PyreeEngine.layers import BaseEntry
from PyreeEngine.shaders import HotloadingShader
from PyreeEngine.simpleshader import SimpleShader

from pyutil.osccontrols import OscToggle, OscRandomVal


class LayerEntry(BaseEntry):
    def __init__(self, context):
        super(LayerEntry, self).__init__(context)

        self.hotloadingshader = None
        self.simpleshader = None

        self.toggles = []
        for i in range(5):
            self.toggles.append(OscToggle(i+5, self.context))

        self.accumtime: float = 0

        self.randomval = []
        for i in range(4):
            self.randomval.append(OscRandomVal(i + 20, self.context))

    def init(self):
        self.hotloadingshader = HotloadingShader(Path("assets/glsl/vert.glsl"),
                                                 Path("assets/glsl/tunnel.glsl"))
        self.simpleshader = SimpleShader(self.context)
        self.simpleshader.updateshader(self.hotloadingshader)

    def __del__(self):
        pass

    def tick(self):
        self.simpleshader.setuniform("res", list(self.context.resolution))

        self.simpleshader.fsquad.textures = [
            self.simpleshader.framebuffer.texture,
        ]

        self.context.data["tunnel"] = self.simpleshader.framebuffer.texture

        uniforms = {
            "time": self.context.time,
            "beataccum": self.context.data["beataccum"],
            "beatpt1": self.context.data["beatpt1"],
            "beataccumpt1": self.context.data["beataccumpt1"]
        }
        for toggle in self.toggles:
            uniforms.update(toggle())
        for k, rv in enumerate(self.randomval):
            if k == 0:
                self.accumtime += self.context.dt * rv.val
                uniforms["accumtime"] = self.accumtime
            else:
                uniforms.update(rv())
        for k, v in uniforms.items():
            self.simpleshader.setuniform(k, v)

        self.hotloadingshader.tick()
        self.simpleshader.tick()

        #self.simpleshader.framebuffer.rendertoscreen()
