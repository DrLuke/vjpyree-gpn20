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

        self.randomval = []
        for i in range(10):
            self.randomval.append(OscRandomVal(i, self.context))

        self.toggles = []
        for i in range(5):
            self.toggles.append(OscToggle(i, self.context))

    def init(self):
        self.hotloadingshader = HotloadingShader(Path("assets/glsl/vert.glsl"),
                                                 Path("assets/glsl/rd.glsl"))
        self.simpleshader = SimpleShader(self.context)
        self.simpleshader.updateshader(self.hotloadingshader)

    def __del__(self):
        pass

    def tick(self):
        self.simpleshader.setuniform("res", list(self.context.resolution))

        self.simpleshader.fsquad.textures = [
            self.simpleshader.framebuffer.texture,
        ]

        uniforms = {
        }
        for i in range(10):
            uniforms.update(self.randomval[i]())
        for toggle in self.toggles:
            uniforms.update(toggle())
        for k, v in uniforms.items():
            self.simpleshader.setuniform(k, v)

        self.hotloadingshader.tick()
        self.simpleshader.tick()

        self.context.data["rd"] = self.simpleshader.framebuffer.texture
