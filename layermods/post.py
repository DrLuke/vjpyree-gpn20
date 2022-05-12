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
            "beataccum": self.context.data["beataccum"]
        }
        for toggle in self.toggles:
            uniforms.update(toggle())
        for k, v in uniforms.items():
            self.simpleshader.setuniform(k, v)

        self.hotloadingshader.tick()
        self.simpleshader.tick()

        self.context.data["posttex"] = self.simpleshader.framebuffer.texture
