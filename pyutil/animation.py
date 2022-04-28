from PyreeEngine.layers import LayerContext


class Wipe:
    """Just goes from 0 to 1 with a certain speed when triggered"""

    def __init__(self, button_index: int, context: LayerContext, speed: float = 1.):
        self.progress: float = 0.0
        self.speed: float = speed

        self.running: bool = False

        self.context = context
        self.button_index = button_index
        self.osc_handler = context.oscdispatcher.map(f"/button/{button_index}", self.trigger)

    def trigger(self, addr, *args):
        if len(args) == 1 and args[0] == 1.:
            self.progress = 0.
            self.running = True

    def tick(self, dt: float) -> float:
        if self.running == False:
            return 0.
        self.progress += self.speed * dt
        if self.progress >= 1.:
            self.progress = 1.
            self.running = False
        return self.progress

    def __del__(self):
        self.context.oscdispatcher.unmap(f"/button/{self.button_index}", self.osc_handler)
