from pathlib import Path

from PyreeEngine.layers import BaseEntry
from PyreeEngine.shaders import HotloadingShader
from PyreeEngine.simpleshader import SimpleShader
from pyutil.osccontrols import OscRandomVal

class LayerEntry(BaseEntry):
    def __init__(self, context):
        super(LayerEntry, self).__init__(context)

        self.hotloadingshader = None
        self.simpleshader = None

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
        for k, v in uniforms.items():
            self.simpleshader.setuniform(k, v)

        self.hotloadingshader.tick()
        self.simpleshader.tick()

        self.context.data["posttex"] = self.simpleshader.framebuffer.texture
