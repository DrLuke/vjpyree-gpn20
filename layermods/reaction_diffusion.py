from pathlib import Path

from PyreeEngine.layers import BaseEntry
from PyreeEngine.shaders import HotloadingShader
from PyreeEngine.simpleshader import SimpleShader

from pyutil.animation import Wipe
from pyutil.osccontrols import OscRandomVal, OscToggle

class LayerEntry(BaseEntry):
    def __init__(self, context):
        super(LayerEntry, self).__init__(context)

        self.hotloadingshader = None
        self.simpleshader = None

        self.randomval = []
        for i in range(4):
            self.randomval.append(OscRandomVal(i+10, self.context))

        self.toggles = []
        for i in range(5):
            self.toggles.append(OscToggle(i, self.context))

        self.wipes = []
        for i in range(5):
            self.wipes.append(Wipe(i, self.context))


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
        for rv in self.randomval:
            uniforms.update(rv())
        for toggle in self.toggles:
            uniforms.update(toggle())
        for i in range(5):
            uniforms[f"wipe{i}"] = self.wipes[i].tick(self.context.dt)
        for k, v in uniforms.items():
            self.simpleshader.setuniform(k, v)

        self.hotloadingshader.tick()
        self.simpleshader.tick()

        self.context.data["rd"] = self.simpleshader.framebuffer.texture
