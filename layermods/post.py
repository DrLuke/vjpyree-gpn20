from pathlib import Path

from PyreeEngine.layers import BaseEntry
from PyreeEngine.shaders import HotloadingShader
from PyreeEngine.simpleshader import SimpleShader
from pyutil.osccontrols import OscRandomVal, OscToggle


class LayerEntry(BaseEntry):
    def __init__(self, context):
        super(LayerEntry, self).__init__(context)

        self.hotloadingshader = None
        self.simpleshader = None

        self.toggles = []
        for i in range(6):
            self.toggles.append(OscToggle(i+10, self.context))
        for i in range(6):
            self.toggles.append(OscToggle(i+20, self.context))
        self.randomvals = []
        self.randomvalsaccum = []
        for i in range(10):
            self.randomvals.append(OscRandomVal(i, self.context))
            self.randomvalsaccum.append(0)
        self.paltoggles = []
        for i in range(6):
            self.paltoggles.append(OscToggle(i+30, self.context))

    def init(self):
        self.hotloadingshader = HotloadingShader(Path("assets/glsl/vert.glsl"),
                                                 Path("assets/glsl/post.glsl"))
        self.simpleshader = SimpleShader(self.context)
        self.simpleshader.updateshader(self.hotloadingshader)

    def __del__(self):
        pass

    def tick(self):
        self.simpleshader.setuniform("res", list(self.context.resolution))

        dt = self.context.dt

        self.simpleshader.fsquad.textures = [
            self.simpleshader.framebuffer.texture,
            self.context.data["rd"],
            self.context.data["tunnel"],
        ]

        uniforms = {
            "beat": self.context.data["beat"],
            "beataccum": self.context.data["beataccum"],
            "beatpt1": self.context.data["beatpt1"],
            "beataccumpt1": self.context.data["beataccumpt1"],
            "res": self.context.resolution
        }
        for toggle in self.toggles:
            uniforms.update(toggle())
        for randomval in self.randomvals:
            uniforms.update(randomval())
        for i in range(6):
            if self.paltoggles[i].val > 0:
                uniforms["palval"] = i
                break
        for i in range(10):
            self.randomvalsaccum[i] += self.randomvals[i].val * self.context.dt
            uniforms[f"randomVal{i}accum"] = self.randomvalsaccum[i]
        for k, v in uniforms.items():
            self.simpleshader.setuniform(k, v)

        self.hotloadingshader.tick()
        self.simpleshader.tick()

        self.context.data["posttex"] = self.simpleshader.framebuffer.texture

        self.simpleshader.framebuffer.rendertoscreen()
